use anyhow::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use axum_login::permission_required;
use fred::clients::RedisPool;
use fred::error::RedisError;
use fred::interfaces::KeysInterface;
use fred::prelude::RedisResult;
use http::StatusCode;
use validator::Validate;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::creator_access;
use crate::domain::requests::creator_access::CreateOfferRequest;
use crate::domain::requests::creator_access::SubmitMusicProductRequest;
use crate::error_chain_fmt;
use crate::routes::ResponseError;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
pub enum CreatorResponseError {
    #[error(transparent)]
    ResponseError(#[from] ResponseError),
    #[error("No upload info in cache")]
    NoUploadInfoInCacheError(#[from] RedisError),
}

impl std::fmt::Debug for CreatorResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for CreatorResponseError {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreatorResponseError::ResponseError(e) => e.into_response(),
            CreatorResponseError::NoUploadInfoInCacheError(_) => {
                tracing::error!("{:?}", self);
                StatusCode::EXPECTATION_FAILED.into_response()
            }
        }
    }
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn creator_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route("/submit_music_product", routing::post(submit_music_product))
        .route("/create_offer", routing::post(create_offer))
        .layer(permission_required!(crate::auth::users::Backend, "creator"))
}

#[tracing::instrument(name = "Creator's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument(name = "Submit a new song", skip_all)]
async fn submit_music_product(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(req): Json<SubmitMusicProductRequest>,
) -> Result<StatusCode, CreatorResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    req.validate().map_err(ResponseError::ValidationError)?;

    let (music_product, song) = match req {
        SubmitMusicProductRequest::Beat(m) => (m, None),
        SubmitMusicProductRequest::Song(s) => {
            (s.music_product, Some((s.lyric, s.sex)))
        }
    };

    let mut object_keys = vec![
        music_product.master_object_key.as_str(),
        music_product.multitrack_object_key.as_str(),
        music_product.cover_object_key.as_str(),
    ];
    if let Some(ref tagged) = music_product.master_tagged_object_key {
        object_keys.push(tagged.as_str());
    }

    verify_upload_request_data_in_redis(
        &app_state.redis_pool,
        &object_keys,
        user.id,
    )
    .await?;

    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")
        .map_err(ResponseError::UnexpectedError)?;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")
        .map_err(ResponseError::UnexpectedError)?;

    let product_id = creator_access::insert_product_and_get_product_id()
        .bind(
            &transaction,
            &user.id,
            &music_product.name,
            &music_product.description,
            &music_product.price,
        )
        .one()
        .await
        .context("Failed to insert music product into the pg")
        .map_err(ResponseError::UnexpectedError)?;

    // match
    let (beat_id, song_id) = match song {
        Some((lyric, sex)) => {
            let song_id = creator_access::insert_song_and_get_song_id()
                .bind(
                    &transaction,
                    &product_id,
                    &music_product.primary_genre,
                    &music_product.secondary_genre,
                    &sex.to_string(),
                    &music_product.tempo,
                    &music_product.key.clone().into(),
                    &music_product.duration,
                    &lyric,
                )
                .one()
                .await
                .context("Failed to insert song data into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            (None, Some(song_id))
        }
        None => {
            let beat_id = creator_access::insert_beat_and_get_beat_id()
                .bind(
                    &transaction,
                    &product_id,
                    &music_product.primary_genre,
                    &music_product.secondary_genre,
                    &music_product.tempo,
                    &music_product.key.clone().into(),
                    &music_product.duration,
                )
                .one()
                .await
                .context("Failed to insert beat data into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            (Some(beat_id), None)
        }
    };

    creator_access::insert_product_cover_object_key()
        .bind(&transaction, &music_product.cover_object_key, &product_id)
        .await
        .context("Failed to insert cover_object_key into pg")
        .map_err(ResponseError::UnexpectedError)?;

    creator_access::insert_music_product_master_object_key()
        .bind(
            &transaction,
            &music_product.master_object_key,
            &song_id,
            &beat_id,
        )
        .await
        .context("Failed to insert song_master_object_key into pg")
        .map_err(ResponseError::UnexpectedError)?;

    if let Some(ref tagged_key) = music_product.master_tagged_object_key {
        creator_access::insert_music_product_master_tagged_object_key()
            .bind(&transaction, &tagged_key, &song_id, &beat_id)
            .await
            .context("Failed to insert song_master_object_key into pg")
            .map_err(ResponseError::UnexpectedError)?;
    }

    creator_access::insert_music_product_multitrack_object_key()
        .bind(
            &transaction,
            &music_product.multitrack_object_key,
            &song_id,
            &beat_id,
        )
        .await
        .context("Failed to insert song_master_object_key into pg")
        .map_err(ResponseError::UnexpectedError)?;

    for mood in music_product.moods.iter() {
        creator_access::insert_product_mood_by_name()
            .bind(&transaction, &product_id, mood)
            .await
            .context("Failed to insert mood for product into pg")
            .map_err(ResponseError::UnexpectedError)?;
    }

    if let Err(e) = transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")
    {
        return Err(ResponseError::UnexpectedError(e).into());
    }

    // Now we can safely delete upload data from redis
    delete_upload_request_data_from_redis(
        &app_state.redis_pool,
        &object_keys,
        user.id,
    )
    .await?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(name = "Create a new offer", skip_all)]
async fn create_offer(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(params): Json<CreateOfferRequest>,
) -> Result<StatusCode, ResponseError> {
    let _user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    creator_access::create_offer()
        .bind(
            &db_client,
            &params.conversation_id,
            &params.service_id,
            &params.text,
            &params.price,
            &params.delivery_date,
            &params.free_revisions,
            &params.revision_price,
        )
        .await
        .context("Failed to insert new offer into pg")?;

    Ok(StatusCode::CREATED)
}
// ───── Functions ────────────────────────────────────────────────────────── //

/// Verify upload requests of a given music product, and if all is ok, delete all requests.
#[tracing::instrument(
    name = "Verify upload requests for a given music product.",
    skip_all
)]
async fn verify_upload_request_data_in_redis(
    con: &RedisPool,
    obj_keys: &[&str],
    user_id: i32,
) -> RedisResult<()> {
    for obj_key in obj_keys.into_iter() {
        let key = format!("upload_request:{}:{}", user_id, obj_key);
        let _created_at: String = con.get(&key).await?;
    }
    Ok(())
}

#[tracing::instrument(
    name = "Delete upload requests for a given music product.",
    skip_all
)]
async fn delete_upload_request_data_from_redis(
    con: &RedisPool,
    obj_keys: &[&str],
    user_id: i32,
) -> RedisResult<()> {
    for obj_key in obj_keys.into_iter() {
        let key = format!("upload_request:{}:{}", user_id, obj_key);
        con.del(&key).await?;
    }
    Ok(())
}

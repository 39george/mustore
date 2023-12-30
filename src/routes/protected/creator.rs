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
use tower_http::trace::TraceLayer;
use validator::ValidateArgs;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::creator_access;
use crate::domain::requests::SubmitSongRequest;
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
        .route("/submit_song", routing::post(submit_song))
        .layer(permission_required!(crate::auth::users::Backend, "creator"))
}

#[tracing::instrument(name = "Creator's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument(name = "Submit a new song", skip_all)]
#[axum::debug_handler]
async fn submit_song(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(params): Json<SubmitSongRequest>,
) -> Result<StatusCode, CreatorResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    params
        .validate_args((1, 50))
        .map_err(ResponseError::ValidationError)?;

    verify_upload_request_data_in_redis(
        &app_state.redis_pool,
        &params,
        user.id,
        false,
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
            &params.name,
            &params.description,
            &params.price,
        )
        .one()
        .await
        .context("Failed to insert song (product tab) into the pg")
        .map_err(ResponseError::UnexpectedError)?;

    let song_id = creator_access::insert_song_and_get_song_id()
        .bind(
            &transaction,
            &product_id,
            &params.primary_genre,
            &params.secondary_genre,
            &params.sex.to_string(),
            &params.tempo,
            &params.key.clone().into(),
            &params.duration,
            &params.lyric,
        )
        .one()
        .await
        .context("Failed to insert song (song tab) into the pg")
        .map_err(ResponseError::UnexpectedError)?;

    creator_access::insert_product_cover_object_key()
        .bind(&transaction, &params.song_cover_object_key, &product_id)
        .await
        .context("Failed to insert cover_object_key into pg")
        .map_err(ResponseError::UnexpectedError)?;

    creator_access::insert_song_master_object_key()
        .bind(&transaction, &params.song_master_object_key, &song_id)
        .await
        .context("Failed to insert song_master_object_key into pg")
        .map_err(ResponseError::UnexpectedError)?;

    if let Some(ref tagged_key) = params.song_master_tagged_object_key {
        creator_access::insert_song_master_tagged_object_key()
            .bind(&transaction, &tagged_key, &song_id)
            .await
            .context("Failed to insert song_master_object_key into pg")
            .map_err(ResponseError::UnexpectedError)?;
    }

    creator_access::insert_song_multitrack_object_key()
        .bind(&transaction, &params.song_multitrack_object_key, &song_id)
        .await
        .context("Failed to insert song_master_object_key into pg")
        .map_err(ResponseError::UnexpectedError)?;

    if let Err(e) = transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")
    {
        return Err(ResponseError::UnexpectedError(e).into());
    }

    // Now we can safely delete upload data from redis
    verify_upload_request_data_in_redis(
        &app_state.redis_pool,
        &params,
        user.id,
        true,
    )
    .await?;

    Ok(StatusCode::CREATED)
}

// ───── Functions ────────────────────────────────────────────────────────── //

/// Verify upload requests of given song, and if all is ok, delete requests.
#[tracing::instrument(name = "Verify upload requests of given song.", skip_all)]
async fn verify_upload_request_data_in_redis(
    con: &RedisPool,
    req: &SubmitSongRequest,
    user_id: i32,
    should_delete: bool,
) -> RedisResult<()> {
    let tagged = req.song_master_tagged_object_key.as_ref();
    let object_keys = [
        &req.song_master_object_key,
        &req.song_multitrack_object_key,
        &req.song_cover_object_key,
    ];

    for obj_key in object_keys.into_iter() {
        let key = format!("upload_request:{}:{}", user_id, obj_key);
        let _created_at: String = con.get(&key).await?;
        if should_delete {
            con.del(&key).await?;
        }
    }

    if let Some(obj_key) = tagged {
        let key = format!("upload_request:{}:{}", user_id, obj_key);
        let _created_at: String = con.get(&key).await?;
        if should_delete {
            con.del(&key).await?;
        }
    }
    Ok(())
}

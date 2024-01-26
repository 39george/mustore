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
use garde::Validate;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::creator_access;
use crate::cornucopia::types::public::Objecttype;
use crate::domain::object_key::ObjectKey;
use crate::domain::requests::creator_access::CreateOfferRequest;
use crate::domain::requests::creator_access::SubmitProductRequest;
use crate::domain::requests::creator_access::SubmitServiceRequest;
use crate::domain::upload_request::UploadRequest;
use crate::error_chain_fmt;
use crate::routes::ResponseError;
use crate::startup::api_doc::InternalErrorResponse;
use crate::startup::api_doc::NotFoundResponse;
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
                StatusCode::BAD_REQUEST.into_response()
            }
        }
    }
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn creator_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route("/submit_product", routing::post(submit_product))
        .route("/submit_service", routing::post(submit_service))
        .route("/create_offer", routing::post(create_offer))
        .layer(permission_required!(crate::auth::users::Backend, "creator"))
}

/// Check access to creator's endpoint.
#[utoipa::path(
        get,
        path = "/api/protected/creator/health_check",
        responses(
            (status = 200, description = "Accessed to protected health check"),
            (status = 403, description = "Forbidden")
        ),
        security(
         ("api_key" = [])
        ),
        tag = "health_checks"
)]
#[tracing::instrument(name = "Creator's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

// TODO: receive attachments, and check if they exists
/// Submit a new product.
#[utoipa::path(
    post,
    path = "/api/protected/creator/submit_product",
    request_body(
        content = SubmitProductRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Product was submitted"),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.creators"
)]
#[tracing::instrument(
    name = "Submit a new product",
    skip_all,
    fields(username)
)]
async fn submit_product(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(req): Json<SubmitProductRequest>,
) -> Result<StatusCode, CreatorResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    req.validate(&()).map_err(ResponseError::ValidationError)?;

    let s3 = app_state.object_storage;

    let (product, music_product, lyric, sex) = match req {
        SubmitProductRequest::Beat {
            product,
            music_product,
        } => (product, Some(music_product), None, None),
        SubmitProductRequest::Song {
            product,
            music_product,
            lyric,
            sex,
        } => (product, Some(music_product), Some(lyric), Some(sex)),
        SubmitProductRequest::Lyric {
            product,
            lyric,
            sex,
        } => (product, None, Some(lyric), sex),
        SubmitProductRequest::Cover { product } => (product, None, None, None),
    };

    let mut object_keys = vec![&product.cover_object_key];

    if let Some(ref mp) = music_product {
        object_keys.push(&mp.master_object_key);
        object_keys.push(&mp.multitrack_object_key);
        if let Some(ref tagged) = mp.master_tagged_object_key {
            object_keys.push(tagged);
        }
    }

    verify_upload_request_data_in_redis(
        &app_state.redis_pool,
        &object_keys,
        user.id,
    )
    .await?;

    // Check that keys are exist
    for &key in object_keys.iter() {
        let _meta = s3
            .get_object_meta(key)
            .await
            .map_err(ResponseError::ObjectStorageError)?;
    }

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
            &product.name,
            &product.description,
            &product.price,
        )
        .one()
        .await
        .context("Failed to insert music product into the pg")
        .map_err(ResponseError::UnexpectedError)?;

    creator_access::insert_product_cover_object_key()
        .bind(
            &transaction,
            &s3.receive(&product.cover_object_key)
                .await
                .map_err(ResponseError::ObjectStorageError)?
                .as_ref(),
            &product_id,
        )
        .await
        .context("Failed to insert cover_object_key into pg")
        .map_err(ResponseError::UnexpectedError)?;

    let (beat_id, song_id) = match (&music_product, lyric, sex) {
        (None, None, None) => {
            creator_access::insert_cover()
                .bind(&transaction, &product_id)
                .await
                .context("Failed to insert cover into pg")
                .map_err(ResponseError::UnexpectedError)?;
            (None, None)
        }
        (None, Some(lyric), sex) => {
            creator_access::insert_lyric()
                .bind(
                    &transaction,
                    &product_id,
                    &lyric.as_ref(),
                    &sex.map(|s| s.to_string()),
                )
                .await
                .context("Failed to insert lyric into pg")
                .map_err(ResponseError::UnexpectedError)?;
            (None, None)
        }
        (Some(music_product), None, None) => {
            let beat_id = creator_access::insert_beat_and_get_beat_id()
                .bind(
                    &transaction,
                    &product_id,
                    &music_product.primary_genre,
                    &music_product.secondary_genre,
                    &music_product.tempo,
                    &music_product.music_key.clone().into(),
                    &music_product.duration,
                )
                .one()
                .await
                .context("Failed to insert beat data into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            (Some(beat_id), None)
        }
        (Some(music_product), Some(lyric), Some(sex)) => {
            let song_id = creator_access::insert_song_and_get_song_id()
                .bind(
                    &transaction,
                    &product_id,
                    &music_product.primary_genre,
                    &music_product.secondary_genre,
                    &sex.to_string(),
                    &music_product.tempo,
                    &music_product.music_key.clone().into(),
                    &music_product.duration,
                    &lyric.as_ref(),
                )
                .one()
                .await
                .context("Failed to insert song data into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            (None, Some(song_id))
        }
        // TODO: doc, why this is unreachable
        _ => unreachable!(),
    };

    if let Some(ref music_product) = music_product {
        creator_access::insert_music_product_master_object_key()
            .bind(
                &transaction,
                &s3.receive(&music_product.master_object_key)
                    .await
                    .map_err(ResponseError::ObjectStorageError)?
                    .as_ref(),
                &song_id,
                &beat_id,
            )
            .await
            .context("Failed to insert song_master_object_key into pg")
            .map_err(ResponseError::UnexpectedError)?;

        if let Some(ref tagged_key) = music_product.master_tagged_object_key {
            creator_access::insert_music_product_master_tagged_object_key()
                .bind(
                    &transaction,
                    &s3.receive(&tagged_key)
                        .await
                        .map_err(ResponseError::ObjectStorageError)?
                        .as_ref(),
                    &song_id,
                    &beat_id,
                )
                .await
                .context("Failed to insert song_master_object_key into pg")
                .map_err(ResponseError::UnexpectedError)?;
        }

        creator_access::insert_music_product_multitrack_object_key()
            .bind(
                &transaction,
                &s3.receive(&music_product.multitrack_object_key)
                    .await
                    .map_err(ResponseError::ObjectStorageError)?
                    .as_ref(),
                &song_id,
                &beat_id,
            )
            .await
            .context("Failed to insert song_master_object_key into pg")
            .map_err(ResponseError::UnexpectedError)?;
    }

    for mood in product.moods.iter() {
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

#[tracing::instrument(name = "Submit service", skip_all, fields(username))]
async fn submit_service(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(req): Json<SubmitServiceRequest>,
) -> Result<StatusCode, CreatorResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    req.validate(&()).map_err(ResponseError::ValidationError)?;

    let s3 = app_state.object_storage;

    let (service, genres) = match &req {
        SubmitServiceRequest::Mixing(ref music_service)
        | SubmitServiceRequest::BeatWriting(ref music_service)
        | SubmitServiceRequest::SongWriting(ref music_service) => {
            (&music_service.service, &music_service.genres)
        }
        SubmitServiceRequest::GhostWriting { ref service, .. } => {
            (service, &None)
        }
        SubmitServiceRequest::CoverDesign(ref service) => (service, &None),
    };

    let mut object_keys = vec![&service.cover_object_key];
    if let Some(ref credits) = service.credits_object_keys {
        object_keys.extend(credits.into_iter().collect::<Vec<_>>());
    }

    verify_upload_request_data_in_redis(
        &app_state.redis_pool,
        &object_keys,
        user.id,
    )
    .await?;

    // Check that keys are exist
    for &key in object_keys.iter() {
        let _meta = s3
            .get_object_meta(key)
            .await
            .map_err(ResponseError::ObjectStorageError)?;
    }

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

    let service_id = creator_access::insert_service_get_id()
        .bind(
            &transaction,
            &user.id,
            &service.name,
            &service.description,
            &service.display_price,
        )
        .one()
        .await
        .context("Failed to insert service data into the pg")
        .map_err(ResponseError::UnexpectedError)?;

    creator_access::insert_service_cover_object_key()
        .bind(
            &transaction,
            &s3.receive(&service.cover_object_key)
                .await
                .map_err(ResponseError::ObjectStorageError)?
                .as_ref(),
            &service_id,
        )
        .await
        .context("Failed to insert service cover object key into the pg")
        .map_err(ResponseError::UnexpectedError)?;

    let (mixing_id, beat_writing_id, song_writing_id) = match req {
        SubmitServiceRequest::Mixing(_) => {
            let id = creator_access::insert_mixing()
                .bind(&transaction, &service_id)
                .one()
                .await
                .context("Failed to insert mixing into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            for key in service.credits_object_keys.iter().flatten() {
                creator_access::insert_mixing_credit_object_key()
                    .bind(
                        &transaction,
                        &s3.receive(key)
                            .await
                            .map_err(ResponseError::ObjectStorageError)?
                            .as_ref(),
                        &Objecttype::audio,
                        &id,
                    )
                    .await
                    .context("Failed to insert mixing credit into the pg")
                    .map_err(ResponseError::UnexpectedError)?;
            }
            (Some(id), None, None)
        }
        SubmitServiceRequest::SongWriting(_) => {
            let id = creator_access::insert_song_writing()
                .bind(&transaction, &service_id)
                .one()
                .await
                .context("Failed to insert song writing into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            for key in service.credits_object_keys.iter().flatten() {
                creator_access::insert_song_writing_credit_object_key()
                    .bind(
                        &transaction,
                        &s3.receive(key)
                            .await
                            .map_err(ResponseError::ObjectStorageError)?
                            .as_ref(),
                        &Objecttype::audio,
                        &id,
                    )
                    .await
                    .context("Failed to insert song writing credit into the pg")
                    .map_err(ResponseError::UnexpectedError)?;
            }
            (None, None, Some(id))
        }
        SubmitServiceRequest::BeatWriting(_) => {
            let id = creator_access::insert_beat_writing()
                .bind(&transaction, &service_id)
                .one()
                .await
                .context("Failed to insert beat writing into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            for key in service.credits_object_keys.iter().flatten() {
                creator_access::insert_beat_writing_credit_object_key()
                    .bind(
                        &transaction,
                        &s3.receive(key)
                            .await
                            .map_err(ResponseError::ObjectStorageError)?
                            .as_ref(),
                        &Objecttype::audio,
                        &id,
                    )
                    .await
                    .context("Failed to insert beat writing credit into the pg")
                    .map_err(ResponseError::UnexpectedError)?;
            }
            (None, Some(id), None)
        }
        SubmitServiceRequest::GhostWriting { ref credits, .. } => {
            creator_access::insert_ghost_writing()
                .bind(&transaction, &service_id, &credits)
                .await
                .context("Failed to insert ghost writing into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            (None, None, None)
        }
        SubmitServiceRequest::CoverDesign(_) => {
            let id = creator_access::insert_cover_design()
                .bind(&transaction, &service_id)
                .one()
                .await
                .context("Failed to insert cover design into the pg")
                .map_err(ResponseError::UnexpectedError)?;
            for key in service.credits_object_keys.iter().flatten() {
                creator_access::insert_cover_design_credit_object_key()
                    .bind(
                        &transaction,
                        &s3.receive(key)
                            .await
                            .map_err(ResponseError::ObjectStorageError)?
                            .as_ref(),
                        &Objecttype::image,
                        &id,
                    )
                    .await
                    .context("Failed to insert cover design credit into the pg")
                    .map_err(ResponseError::UnexpectedError)?;
            }
            (None, None, None)
        }
    };

    for genre in genres.iter().flatten() {
        creator_access::insert_music_service_genre()
            .bind(
                &transaction,
                genre,
                &beat_writing_id,
                &song_writing_id,
                &mixing_id,
            )
            .await
            .context("Failed to insert mixing credit into the pg")
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

#[tracing::instrument(name = "Create a new offer", skip_all, fields(username))]
async fn create_offer(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(params): Json<CreateOfferRequest>,
) -> Result<StatusCode, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

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
    obj_keys: &[&ObjectKey],
    user_id: i32,
) -> RedisResult<()> {
    for obj_key in obj_keys.into_iter() {
        let upload_request = UploadRequest::new(user_id, (*obj_key).clone());
        let _created_at: String = con.get(&upload_request.to_string()).await?;
    }
    Ok(())
}

#[tracing::instrument(
    name = "Delete upload requests for a given music product.",
    skip_all
)]
async fn delete_upload_request_data_from_redis(
    con: &RedisPool,
    obj_keys: &[&ObjectKey],
    user_id: i32,
) -> RedisResult<()> {
    for obj_key in obj_keys.into_iter() {
        let upload_request = UploadRequest::new(user_id, (*obj_key).clone());
        con.del(&upload_request.to_string()).await?;
    }
    Ok(())
}

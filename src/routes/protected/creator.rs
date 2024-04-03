use std::time::Duration;

use anyhow::Context;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing;
use axum::Json;
use axum::Router;
use axum_login::permission_required;
use banksim_api::register_card_token::RegisterCardToken;
use banksim_api::register_card_token::RegisterCardTokenRequest;
use fred::error::RedisError;
use futures::future::try_join_all;
use garde::Validate;
use http::StatusCode;
use reqwest::Url;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::creator_access;
use crate::cornucopia::types::public::Objecttype;
use crate::domain::general_types::ProductStatus;
use crate::domain::object_key::ObjectKey;
use crate::domain::requests::creator_access::CreateOfferRequest;
use crate::domain::requests::creator_access::SubmitProductRequest;
use crate::domain::requests::creator_access::SubmitServiceRequest;
use crate::domain::sessions::card_token_registration::CardTokenSession;
use crate::domain::upload_request::delete_upload_request_data_from_redis;
use crate::domain::upload_request::verify_upload_request_data_in_redis;
use crate::impl_debug;
use crate::routes::ErrorResponse;
use crate::startup::api_doc::BadRequestResponse;
use crate::startup::api_doc::ForbiddenResponse;
use crate::startup::api_doc::GetCreatorSongs;
use crate::startup::api_doc::InternalErrorResponse;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
pub enum CreatorErrorResponse {
    #[error(transparent)]
    ErrorResponse(#[from] ErrorResponse),
    #[error("No upload info in cache")]
    NoUploadInfoInCacheError(#[from] RedisError),
    #[error("Error with payment api client")]
    PaymentClientError(#[from] airactions::ClientError),
}

impl_debug!(CreatorErrorResponse);

impl IntoResponse for CreatorErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            CreatorErrorResponse::ErrorResponse(e) => e.into_response(),
            CreatorErrorResponse::NoUploadInfoInCacheError(_) => {
                tracing::error!("{:?}", self);
                StatusCode::BAD_REQUEST.into_response()
            }
            CreatorErrorResponse::PaymentClientError(_) => {
                tracing::error!("{:?}", self);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn creator_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route("/marks_avg", routing::get(marks_avg))
        .route("/submit_product", routing::post(submit_product))
        .route("/submit_service", routing::post(submit_service))
        .route("/create_offer", routing::post(create_offer))
        .route("/connect_card", routing::post(connect_card))
        .route("/songs/:status", routing::get(songs))
        .layer(permission_required!(crate::auth::users::Backend, "creator"))
}

#[tracing::instrument(name = "Creator's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[utoipa::path(
    get,
    path = "/api/protected/creator/marks_avg",
    responses(
        (
            status = 200,
            body = creator_access::GetCreatorMarksAvg,
            content_type = "application/json",
            description = "Marks avg for creator",
            example = json!(
                  {
                    "avg": 4.7,
                    "count": 15,
                  }
            )
        ),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.creators"
)]
#[tracing::instrument(name = "Get marks count & average for creator", skip_all)]
async fn marks_avg(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
) -> Result<Json<creator_access::GetCreatorMarksAvg>, CreatorErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")
        .map_err(ErrorResponse::UnexpectedError)?;

    let data = creator_access::get_creator_marks_avg()
        .bind(&db_client, &user.id)
        .one()
        .await
        .context("Failed to get marks avg data from database")
        .map_err(ErrorResponse::UnexpectedError)?;

    Ok(Json(data))
}

/// Initialize card connection operation
#[utoipa::path(
    post,
    path = "/api/protected/creator/connect_card",
    responses(
        (
            status = 200,
            body = creator_access::GetCreatorMarksAvg,
            content_type = "application/json",
            description = "Marks avg for creator",
            example = json!(
                  {
                    "avg": 4.7,
                    "count": 15,
                  }
            )
        ),
        (status = 403, description = "Forbidden"),
        (status = 403, response = ForbiddenResponse),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.creators"
)]
#[tracing::instrument(name = "Initiate creator's card connection", skip_all)]
async fn connect_card(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
) -> Result<Redirect, CreatorErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    let base: Url = app_state.settings.app_base_url.parse().unwrap();
    let success_url = base
        .join("/notification_center/card_connect_success")
        .unwrap();
    let fail_url = base
        .join("/notification_center/card_connect_failed")
        .unwrap();
    let notification_url = base
        .join("/notification_center/card_connect_notification")
        .unwrap();
    let cashbox_pass = &app_state.settings.payments.cashbox_password;
    let request = RegisterCardTokenRequest::new(
        notification_url,
        success_url,
        fail_url,
        cashbox_pass,
    );
    let response = app_state
        .airactions_c
        .execute(RegisterCardToken, request)
        .await?;

    let (operation_id, registration_url) = match response.status {
        banksim_api::OperationStatus::Success => (
            response.operation_id.unwrap(),
            response.registration_url.unwrap().to_string(),
        ),
        banksim_api::OperationStatus::Fail(e) => {
            return Err(CreatorErrorResponse::ErrorResponse(
                ErrorResponse::InternalError(anyhow::Error::msg(e)),
            ));
        }
        banksim_api::OperationStatus::Cancel => todo!(),
    };

    let redis_client = app_state.redis_pool;
    match CardTokenSession::new(
        redis_client.next_connected(),
        operation_id,
        user.id,
    )
    .await
    {
        Ok(()) => (),
        Err(e) => {
            return Err(ErrorResponse::InternalError(anyhow::anyhow!(
                "Failed to store card token session in redis: {e}"
            ))
            .into())
        }
    }

    Ok(Redirect::to(&registration_url))
}

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
        (status = 400, response = BadRequestResponse),
        (status = 403, response = ForbiddenResponse),
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
) -> Result<StatusCode, CreatorErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    req.validate(&()).map_err(ErrorResponse::ValidationError)?;

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
            .map_err(ErrorResponse::ObjectStorageError)?;
    }

    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")
        .map_err(ErrorResponse::UnexpectedError)?;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")
        .map_err(ErrorResponse::UnexpectedError)?;

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
        .map_err(ErrorResponse::UnexpectedError)?;

    creator_access::insert_product_cover_object_key()
        .bind(
            &transaction,
            &s3.receive(&product.cover_object_key)
                .await
                .map_err(ErrorResponse::ObjectStorageError)?
                .as_ref(),
            &product_id,
        )
        .await
        .context("Failed to insert cover_object_key into pg")
        .map_err(ErrorResponse::UnexpectedError)?;

    let (beat_id, song_id) = match (&music_product, lyric, sex) {
        (None, None, None) => {
            creator_access::insert_cover()
                .bind(&transaction, &product_id)
                .await
                .context("Failed to insert cover into pg")
                .map_err(ErrorResponse::UnexpectedError)?;
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
                .map_err(ErrorResponse::UnexpectedError)?;
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
                .map_err(ErrorResponse::UnexpectedError)?;
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
                .map_err(ErrorResponse::UnexpectedError)?;
            (None, Some(song_id))
        }
        // Unreachable, because when we parse
        // Beat, Song, Lyric, Cover cases, only provided paths available.
        _ => unreachable!(),
    };

    if let Some(ref music_product) = music_product {
        creator_access::insert_music_product_master_object_key()
            .bind(
                &transaction,
                &s3.receive(&music_product.master_object_key)
                    .await
                    .map_err(ErrorResponse::ObjectStorageError)?
                    .as_ref(),
                &song_id,
                &beat_id,
            )
            .await
            .context("Failed to insert song_master_object_key into pg")
            .map_err(ErrorResponse::UnexpectedError)?;

        if let Some(ref tagged_key) = music_product.master_tagged_object_key {
            creator_access::insert_music_product_master_tagged_object_key()
                .bind(
                    &transaction,
                    &s3.receive(&tagged_key)
                        .await
                        .map_err(ErrorResponse::ObjectStorageError)?
                        .as_ref(),
                    &song_id,
                    &beat_id,
                )
                .await
                .context("Failed to insert song_master_object_key into pg")
                .map_err(ErrorResponse::UnexpectedError)?;
        }

        creator_access::insert_music_product_multitrack_object_key()
            .bind(
                &transaction,
                &s3.receive(&music_product.multitrack_object_key)
                    .await
                    .map_err(ErrorResponse::ObjectStorageError)?
                    .as_ref(),
                &song_id,
                &beat_id,
            )
            .await
            .context("Failed to insert song_master_object_key into pg")
            .map_err(ErrorResponse::UnexpectedError)?;
    }

    for mood in product.moods.iter() {
        creator_access::insert_product_mood_by_name()
            .bind(&transaction, &product_id, mood)
            .await
            .context("Failed to insert mood for product into pg")
            .map_err(ErrorResponse::UnexpectedError)?;
    }

    if let Err(e) = transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")
    {
        return Err(ErrorResponse::UnexpectedError(e).into());
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

/// Submit a new service.
#[utoipa::path(
    post,
    path = "/api/protected/creator/submit_service",
    request_body(
        content = SubmitServiceRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Service was submitted"),
        (status = 400, response = BadRequestResponse),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.creators"
)]
#[tracing::instrument(name = "Submit service", skip_all, fields(username))]
async fn submit_service(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(req): Json<SubmitServiceRequest>,
) -> Result<StatusCode, CreatorErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    req.validate(&()).map_err(ErrorResponse::ValidationError)?;

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
            .map_err(ErrorResponse::ObjectStorageError)?;
    }

    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")
        .map_err(ErrorResponse::UnexpectedError)?;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")
        .map_err(ErrorResponse::UnexpectedError)?;

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
        .map_err(ErrorResponse::UnexpectedError)?;

    creator_access::insert_service_cover_object_key()
        .bind(
            &transaction,
            &s3.receive(&service.cover_object_key)
                .await
                .map_err(ErrorResponse::ObjectStorageError)?
                .as_ref(),
            &service_id,
        )
        .await
        .context("Failed to insert service cover object key into the pg")
        .map_err(ErrorResponse::UnexpectedError)?;

    let (mixing_id, beat_writing_id, song_writing_id) = match req {
        SubmitServiceRequest::Mixing(_) => {
            let id = creator_access::insert_mixing()
                .bind(&transaction, &service_id)
                .one()
                .await
                .context("Failed to insert mixing into the pg")
                .map_err(ErrorResponse::UnexpectedError)?;
            for key in service.credits_object_keys.iter().flatten() {
                creator_access::insert_mixing_credit_object_key()
                    .bind(
                        &transaction,
                        &s3.receive(key)
                            .await
                            .map_err(ErrorResponse::ObjectStorageError)?
                            .as_ref(),
                        &Objecttype::audio,
                        &id,
                    )
                    .await
                    .context("Failed to insert mixing credit into the pg")
                    .map_err(ErrorResponse::UnexpectedError)?;
            }
            (Some(id), None, None)
        }
        SubmitServiceRequest::SongWriting(_) => {
            let id = creator_access::insert_song_writing()
                .bind(&transaction, &service_id)
                .one()
                .await
                .context("Failed to insert song writing into the pg")
                .map_err(ErrorResponse::UnexpectedError)?;
            for key in service.credits_object_keys.iter().flatten() {
                creator_access::insert_song_writing_credit_object_key()
                    .bind(
                        &transaction,
                        &s3.receive(key)
                            .await
                            .map_err(ErrorResponse::ObjectStorageError)?
                            .as_ref(),
                        &Objecttype::audio,
                        &id,
                    )
                    .await
                    .context("Failed to insert song writing credit into the pg")
                    .map_err(ErrorResponse::UnexpectedError)?;
            }
            (None, None, Some(id))
        }
        SubmitServiceRequest::BeatWriting(_) => {
            let id = creator_access::insert_beat_writing()
                .bind(&transaction, &service_id)
                .one()
                .await
                .context("Failed to insert beat writing into the pg")
                .map_err(ErrorResponse::UnexpectedError)?;
            for key in service.credits_object_keys.iter().flatten() {
                creator_access::insert_beat_writing_credit_object_key()
                    .bind(
                        &transaction,
                        &s3.receive(key)
                            .await
                            .map_err(ErrorResponse::ObjectStorageError)?
                            .as_ref(),
                        &Objecttype::audio,
                        &id,
                    )
                    .await
                    .context("Failed to insert beat writing credit into the pg")
                    .map_err(ErrorResponse::UnexpectedError)?;
            }
            (None, Some(id), None)
        }
        SubmitServiceRequest::GhostWriting { ref credits, .. } => {
            creator_access::insert_ghost_writing()
                .bind(&transaction, &service_id, &credits)
                .await
                .context("Failed to insert ghost writing into the pg")
                .map_err(ErrorResponse::UnexpectedError)?;
            (None, None, None)
        }
        SubmitServiceRequest::CoverDesign(_) => {
            let id = creator_access::insert_cover_design()
                .bind(&transaction, &service_id)
                .one()
                .await
                .context("Failed to insert cover design into the pg")
                .map_err(ErrorResponse::UnexpectedError)?;
            for key in service.credits_object_keys.iter().flatten() {
                creator_access::insert_cover_design_credit_object_key()
                    .bind(
                        &transaction,
                        &s3.receive(key)
                            .await
                            .map_err(ErrorResponse::ObjectStorageError)?
                            .as_ref(),
                        &Objecttype::image,
                        &id,
                    )
                    .await
                    .context("Failed to insert cover design credit into the pg")
                    .map_err(ErrorResponse::UnexpectedError)?;
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
            .map_err(ErrorResponse::UnexpectedError)?;
    }

    if let Err(e) = transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")
    {
        return Err(ErrorResponse::UnexpectedError(e).into());
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

/// Create a new offer.
#[utoipa::path(
    post,
    path = "/api/protected/creator/create_offer",
    request_body(
        content = CreateOfferRequest,
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Offer is created"),
        (status = 400, response = BadRequestResponse),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.creators"
)]
#[tracing::instrument(name = "Create a new offer", skip_all, fields(username))]
async fn create_offer(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(req): Json<CreateOfferRequest>,
) -> Result<StatusCode, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    req.validate(&()).map_err(ErrorResponse::ValidationError)?;

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    creator_access::create_offer()
        .bind(
            &db_client,
            &req.conversation_id,
            &req.service_id,
            &req.text,
            &req.price,
            &req.delivery_date,
            &req.free_revisions,
            &req.revision_price,
        )
        .await
        .context("Failed to insert new offer into pg")?;

    Ok(StatusCode::CREATED)
}

/// Retrieve list of creator's songs.
#[utoipa::path(
    get,
    path = "/api/protected/creator/songs/{status}",
    params(
        ("status" = String, Path,
            description = "Song's status",
            example = "sold"
        )
    ),
    responses(
        (status = 200, response = GetCreatorSongs),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.creators"
)]
#[tracing::instrument(name = "Create a new offer", skip_all)]
async fn songs(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Path(status): Path<ProductStatus>,
) -> Result<Json<Vec<creator_access::GetCreatorSongs>>, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")
        .map_err(ErrorResponse::UnexpectedError)?;

    let songs = creator_access::get_creator_songs()
        .bind(&db_client, &user.id, &status.into())
        .all()
        .await
        .context("Failed to retrieve songs from pg")?
        .into_iter()
        .map(|mut song| {
            let obj_storage = app_state.object_storage.clone();
            async move {
                let object_key: ObjectKey =
                    song.key.parse().context("Failed to parse object key")?;
                let result = obj_storage
                    .generate_presigned_url(
                        &object_key,
                        Duration::from_secs(120),
                    ) // 2 minutes expiration
                    .await?;
                song.key = result;
                Ok::<creator_access::GetCreatorSongs, ErrorResponse>(song)
            }
        });

    let songs = try_join_all(songs).await?;

    Ok(Json(songs))
}

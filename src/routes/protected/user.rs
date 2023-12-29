use std::collections::HashMap;

use anyhow::Context;
use axum::extract::State;
use axum::routing;
use axum::Form;
use axum::Json;
use axum::Router;
use axum_login::permission_required;
use fred::clients::RedisPool;
use fred::interfaces::HashesInterface;
use fred::interfaces::SetsInterface;
use fred::interfaces::TransactionInterface;
use fred::interfaces::*;
use fred::prelude::RedisResult;
use http::StatusCode;
use mediatype::media_type;
use mediatype::MediaType;
use mediatype::MediaTypeBuf;
use rust_decimal::Decimal;
use serde::Deserialize;
use time::OffsetDateTime;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::creator_access;
use crate::cornucopia::queries::user_auth_queries;
use crate::cornucopia::queries::user_auth_queries::get_user_permissions;
use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::routes::ResponseError;
use crate::service_providers::object_storage::presigned_post_form::PresignedPostData;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

// Define the static variable MAX_SIZES for acceptable media types.
lazy_static::lazy_static! {
    static ref MAX_SIZES: HashMap<MediaTypeBuf, u64> = {
        let mut m = HashMap::new();
        m.insert(media_type!(IMAGE/PNG).into(), crate::MAX_IMAGE_SIZE_MB);
        m.insert(media_type!(IMAGE/JPEG).into(), crate::MAX_IMAGE_SIZE_MB);
        m.insert(media_type!(AUDIO/WAV).into(), crate::MAX_WAV_SIZE_MB);
        m.insert(media_type!(AUDIO/MPEG).into(), crate::MAX_MP3_SIZE_MB);
        m.insert(
            media_type!(APPLICATION/ZIP).into(),
            crate::MAX_MULTITRACK_SIZE_GB,
        );
        m.insert(media_type!(VIDEO/MP4).into(), crate::MAX_VIDEO_SIZE_MB);
        m.insert(
            media_type!(APPLICATION/PDF).into(),
            crate::MAX_DOCUMENT_SIZE_MB,
        );
        m.insert(
            media_type!(APPLICATION/vnd::OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT).into(),
            crate::MAX_DOCUMENT_SIZE_MB,
        );
        m
    };
}

#[derive(Deserialize, Debug)]
struct UploadFileQuery {
    media_type: mediatype::MediaTypeBuf,
    file_name: String,
}

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route("/upload", routing::get(request_obj_storage_upload_link))
        .layer(permission_required!(crate::auth::users::Backend, "user",))
}

#[tracing::instrument(name = "User's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument(
    name = "Request post form data for obj store upload.",
    skip_all
)]
async fn request_obj_storage_upload_link(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Form(params): Form<UploadFileQuery>,
) -> Result<Json<PresignedPostData>, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    let max_size = match MAX_SIZES.get(&params.media_type) {
        Some(&max_size) => max_size,
        None => {
            tracing::error!("Wrong media type: {}", params.media_type);
            return Err(ResponseError::NotAcceptableError);
        }
    };

    let object_key = format!(
        "{}-{}-{}",
        user.username,
        uuid::Uuid::new_v4(),
        params.file_name
    );

    let presigned_post_data =
        app_state.object_storage.generate_presigned_post_form(
            &object_key,
            params.media_type,
            max_size,
        )?;

    store_upload_request_data(&app_state.redis_pool, &object_key, user.id)
        .await
        .context("Failed to store upload data in the redis.")?;

    Ok(Json(presigned_post_data))
}

#[tracing::instrument(
    name = "Store upload request data in the redis",
    skip_all
)]
async fn store_upload_request_data(
    con: &RedisPool,
    object_key: &str,
    user_id: i32,
) -> RedisResult<()> {
    let user_id = user_id.to_string();
    let created_at = OffsetDateTime::now_utc().to_string();
    let key = format!("upload_request:{}", object_key);

    let mut hash_map: HashMap<&str, &str> = HashMap::new();
    hash_map.insert("object_key", object_key);
    hash_map.insert("user_id", &user_id);
    hash_map.insert("created_at", &created_at);
    let transaction = con.next().multi();
    transaction
        .hset(&key, &hash_map.try_into().unwrap())
        .await?;
    transaction.sadd("upload_requests", key).await?;
    transaction.exec(true).await?;
    Ok(())
}

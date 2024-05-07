use anyhow::Context;
use axum::Router;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::admin::admin_router;
use self::consumer::consumer_router;
use self::creator::creator_router;
use self::minio::minio_router;
use self::user::user_router;
use crate::cornucopia::queries::user_access;
use crate::startup::AppState;

use super::ErrorResponse;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod admin;
pub mod consumer;
pub mod creator;
pub mod minio;
pub mod user;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .nest("/user", user_router())
        .nest("/creator", creator_router())
        .nest("/consumer", consumer_router())
        .nest("/admin", admin_router())
        .nest("/minio", minio_router())
}

/// Check that user has access to the conversation
#[tracing::instrument(name = "Check conversation access", skip_all)]
pub async fn check_conversation_access<T: cornucopia_async::GenericClient>(
    db_client: &T,
    user_id: &i32,
    username: &str,
    conversation_id: &i32,
) -> Result<i32, ErrorResponse> {
    user_access::user_has_access_to_conversation()
        .bind(db_client, user_id, conversation_id)
        .opt()
        .await
        .context("Failed to fetch conversation access from db")?
        .ok_or(ErrorResponse::ForbiddenError(anyhow::anyhow!(
            "{} has no access to the requested conversation id: {}",
            username,
            conversation_id
        )))
}

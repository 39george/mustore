use axum::routing;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::admin::admin_router;
use self::consumer::consumer_router;
use self::creator::creator_router;
use self::minio::minio_router;
use self::user::user_router;
use crate::startup::AppState;

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

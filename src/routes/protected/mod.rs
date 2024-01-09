use axum::routing;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::admin::admin_router;
use self::consumer::consumer_router;
use self::creator::creator_router;
use self::user::user_router;
use crate::startup::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

mod admin;
mod consumer;
mod creator;
mod user;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route_layer(permission_required!(crate::auth::users::Backend, "user"))
        .nest("/user", user_router())
        .nest("/creator", creator_router())
        .nest("/consumer", consumer_router())
        .nest("/admin", admin_router())
}

#[tracing::instrument(name = "Protected health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

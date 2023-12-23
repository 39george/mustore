use axum::routing;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::startup::AppState;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .layer(permission_required!(crate::auth::users::Backend, "user",))
}

#[tracing::instrument(name = "User's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

use axum::routing;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::startup::AppState;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route_layer(permission_required!(
            crate::auth::users::Backend,
            "administrator",
        ))
}

#[tracing::instrument(name = "Creator's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

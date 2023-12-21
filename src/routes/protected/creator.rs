use axum::routing;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::startup::AppState;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn creator_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(creator))
        .route_layer(permission_required!(
            crate::auth::users::Backend,
            "creator",
        ))
}

#[tracing::instrument(name = "Creator's health check", skip_all)]
async fn creator() -> StatusCode {
    StatusCode::OK
}

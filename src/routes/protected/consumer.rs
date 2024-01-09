use axum::routing;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::startup::AppState;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn consumer_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .layer(permission_required!(
            crate::auth::users::Backend,
            "consumer"
        ))
}

#[tracing::instrument(name = "Consumer's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

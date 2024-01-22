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
        .layer(permission_required!(
            crate::auth::users::Backend,
            "administrator",
        ))
}

/// Check access to admin's endpoint.
#[utoipa::path(
        get,
        path = "/api/protected/admin/health_check",
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

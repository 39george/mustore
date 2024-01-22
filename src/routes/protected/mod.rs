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

pub mod admin;
pub mod consumer;
pub mod creator;
pub mod user;

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

/// Check access to top-level protected endpoint.
#[utoipa::path(
        get,
        path = "/api/protected/health_check",
        responses(
            (status = 200, description = "Accessed to protected health check"),
            (status = 403, description = "Forbidden")
        ),
        security(
         ("api_key" = [])
        ),
        tag = "health_checks"
)]
#[tracing::instrument(name = "Protected health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

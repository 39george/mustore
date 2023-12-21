use axum::routing;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use self::admin::admin_router;
use self::creator::creator_router;
use crate::startup::AppState;

// ───── Submodules ───────────────────────────────────────────────────────── //

mod admin;
mod creator;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route_layer(permission_required!(crate::auth::users::Backend, "user"))
        .nest("/creator", creator_router())
        .nest("/admin", admin_router())
}

#[tracing::instrument(name = "Protected health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

// use axum::http::StatusCode;
// use axum::response::IntoResponse;
// use axum::routing::get;
// use axum::Router;

// pub fn router() -> Router<()> {
//     Router::new().route("/", get(self::get::protected))
// }

// mod get {
//     use super::*;

//     pub async fn protected(
//         auth_session: crate::auth::users::AuthSession,
//     ) -> impl IntoResponse {
//         match auth_session.user {
//             Some(_user) => StatusCode::OK.into_response(),
//             None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//         }
//     }
// }

use axum::response::IntoResponse;
use axum::routing;
use axum::Router;
use axum_login::login_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::startup::AppState;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn private_router() -> Router<AppState> {
    Router::new()
        .route("/health_check_protected", routing::get(health_check))
        .layer(login_required!(crate::auth::users::UserBackend))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

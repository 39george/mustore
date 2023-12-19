use axum::response::IntoResponse;
use axum::routing;
use axum::Router;
use axum_login::login_required;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::startup::AppState;

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn private_router() -> Router<AppState> {
    Router::new()
        .route("/another_private", routing::get(|| async {}))
        .layer(login_required!(crate::auth::users::UserBackend))
}

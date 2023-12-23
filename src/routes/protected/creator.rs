use anyhow::anyhow;
use axum::extract::State;
use axum::routing;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::routes::ResponseError;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

// pub enum UserResponseError {
//     #[error(transparent)]
//     Common(#[from] ResponseError),
//     // User-specific errors here
// }

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn creator_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .layer(permission_required!(crate::auth::users::Backend, "creator",))
}

#[tracing::instrument(name = "Creator's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument(name = "Upload a new song", skip_all)]
async fn upload_song(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
) -> Result<StatusCode, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    Ok(StatusCode::CREATED)
}

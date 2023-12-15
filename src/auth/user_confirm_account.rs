use axum::extract::{Query, State};
use http::StatusCode;
use serde::Deserialize;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::AuthError;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Deserialize)]
pub struct UserConfirmationToken {
    token: String,
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

#[tracing::instrument(name = "Signup attempt", skip_all)]
pub async fn confirm(
    State(app_state): State<AppState>,
    Query(UserConfirmationToken { token }): Query<UserConfirmationToken>,
) -> Result<StatusCode, AuthError> {
    Ok(StatusCode::OK)
}

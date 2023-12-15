use axum::extract::State;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::AuthError;
use crate::startup::AppState;

// ───── Handlers ─────────────────────────────────────────────────────────── //

#[tracing::instrument(name = "Signup attempt", skip_all)]
pub async fn confirm(
    State(_app_state): State<AppState>,
) -> Result<StatusCode, AuthError> {
    Ok(StatusCode::OK)
}

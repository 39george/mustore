//! src/auth/mod.rs

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::error_chain_fmt;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod confirm_account;
pub mod login;
pub mod signup;
pub mod users;

// ───── Auth Types ───────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials: {0}")]
    InvalidCredentialsError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Internal error")]
    InternalError(#[source] anyhow::Error),
    #[error("User creation failed: {0}")]
    SignupFailed(#[source] anyhow::Error),
    #[error("Account confirmation failed: {0}")]
    AccountConfirmationFailed(#[source] anyhow::Error),
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        match self {
            AuthError::UnexpectedError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AuthError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AuthError::InvalidCredentialsError(_) => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body("Json auth is required".into())
                .unwrap(),
            AuthError::SignupFailed(_) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Failed to signup".into())
                .unwrap(),
            AuthError::AccountConfirmationFailed(_) => {
                axum::response::Redirect::to(
                    // FIXME: replace this to the real path
                    "react-router/accountconfirmationfailed",
                )
                .into_response()
            }
        }
    }
}

//! src/auth/mod.rs

use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::impl_debug;
use crate::service_providers::captcha_verifier::RecaptchaError;
use crate::service_providers::object_storage::ObjectStorageError;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod confirm_account;
pub mod login;
pub mod signup;
pub mod users;

// ───── Auth Types ───────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
pub enum AuthError {
    #[error(transparent)]
    ObjectStorageError(#[from] ObjectStorageError),
    #[error("Failed to parse credentials: {0}")]
    ValidationError(#[source] anyhow::Error),
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
    #[error("Recaptcha verification failed: {0}")]
    RecaptchaFailed(#[from] RecaptchaError),
}

impl_debug!(AuthError);

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        match self {
            AuthError::ValidationError(e) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(e.to_string()))
                .unwrap_or(StatusCode::BAD_REQUEST.into_response()),
            AuthError::UnexpectedError(_)
            | AuthError::ObjectStorageError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AuthError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AuthError::InvalidCredentialsError(_) => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header("Content-Type", "application/json")
                .body(Body::from("{\"caused_by\": \"Auth is required\"}"))
                .unwrap_or(StatusCode::UNAUTHORIZED.into_response()),
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
            AuthError::RecaptchaFailed(_) => {
                StatusCode::FORBIDDEN.into_response()
            }
        }
    }
}

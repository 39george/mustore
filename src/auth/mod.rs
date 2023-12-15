//! src/auth/mod.rs

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use http::header::WWW_AUTHENTICATE;
use http::HeaderValue;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::error_chain_fmt;

// ───── Submodules ───────────────────────────────────────────────────────── //

mod admins;
pub mod user_confirm_acc;
pub mod user_login;
pub mod user_signup;
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
                .header(
                    WWW_AUTHENTICATE,
                    HeaderValue::from_static(
                        r#"Basic realm="Mustore User Access""#,
                    ),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
            AuthError::SignupFailed(_) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Failed to signup".into())
                .unwrap(),
        }
    }
}

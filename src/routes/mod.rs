//! We specify 400 BadRequest response in openapi documentation
//! only when we can particularly return that. For example, on validation error.

use axum::body::Body;
use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;

use crate::auth::AuthError;
use crate::error_chain_fmt;
use crate::routes::protected::user::MAX_SIZES;
use crate::service_providers::object_storage::ObjectStorageError;

pub mod development;
pub mod health_check;
pub mod open;
pub mod protected;

#[derive(thiserror::Error)]
pub enum ResponseError {
    #[error(transparent)]
    ObjectStorageError(#[from] ObjectStorageError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Internal error")]
    InternalError(#[source] anyhow::Error),
    #[error("Bad request")]
    BadRequest(#[source] anyhow::Error),
    #[error("Validation failed")]
    ValidationError(#[from] garde::Report),
    #[error("Can't process that input")]
    UnsupportedMediaTypeError,
    #[error("No such user")]
    UnauthorizedError(#[source] anyhow::Error),
    #[error("Too many uploads for that user")]
    TooManyUploadsError,
    #[error("Authentication error")]
    AuthError(#[from] AuthError),
    /// Source error is for internal use, and static str is for response
    #[error("Can't resolve given object key as upload")]
    NotFoundError(#[source] anyhow::Error, &'static str),
    #[error("Have no access")]
    ForbiddenError(#[source] anyhow::Error),
    #[error("Conflict error")]
    ConflictError(#[source] anyhow::Error),
}

impl std::fmt::Debug for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        match self {
            ResponseError::UnexpectedError(_)
            | ResponseError::ObjectStorageError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ResponseError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ResponseError::BadRequest(e) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(e.to_string()))
                .unwrap_or(StatusCode::BAD_REQUEST.into_response()),
            ResponseError::UnauthorizedError(_) => {
                StatusCode::UNAUTHORIZED.into_response()
            }
            ResponseError::UnsupportedMediaTypeError => Response::builder()
                .status(StatusCode::UNSUPPORTED_MEDIA_TYPE)
                .header("Content-Type", "application/json")
                .body(Body::from(format!(
                    "{{\"allowed_mediatypes\":{}}}",
                    serde_json::to_string(
                        &MAX_SIZES
                            .keys()
                            .map(|media| media.as_str())
                            .collect::<Vec<&str>>(),
                    )
                    .unwrap(),
                )))
                .unwrap_or(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response()),
            ResponseError::TooManyUploadsError => {
                StatusCode::TOO_MANY_REQUESTS.into_response()
            }
            ResponseError::ValidationError(e) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(e.to_string()))
                .unwrap_or(StatusCode::BAD_REQUEST.into_response()),
            ResponseError::AuthError(e) => e.into_response(),
            ResponseError::NotFoundError(_, param) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Body::from(format!("{{\"param\":{}}}", param)))
                .unwrap_or(StatusCode::NOT_FOUND.into_response()),
            ResponseError::ForbiddenError(_) => {
                StatusCode::FORBIDDEN.into_response()
            }
            ResponseError::ConflictError(_) => {
                StatusCode::CONFLICT.into_response()
            }
        }
    }
}

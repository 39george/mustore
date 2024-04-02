//! We specify 400 BadRequest response in openapi documentation
//! only when we can particularly return that. For example, on validation error.

use axum::body::Body;
use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;

use crate::auth::AuthError;
use crate::impl_debug;
use crate::routes::protected::user::MAX_SIZES;
use crate::service_providers::object_storage::ObjectStorageError;

pub mod development;
pub mod health_check;
pub mod notifications;
pub mod open;
pub mod protected;

#[derive(thiserror::Error)]
pub enum ErrorResponse {
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

impl_debug!(ErrorResponse);

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        match self {
            ErrorResponse::UnexpectedError(_)
            | ErrorResponse::ObjectStorageError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ErrorResponse::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ErrorResponse::BadRequest(e) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(e.to_string()))
                .unwrap_or(StatusCode::BAD_REQUEST.into_response()),
            ErrorResponse::UnauthorizedError(_) => {
                StatusCode::UNAUTHORIZED.into_response()
            }
            ErrorResponse::UnsupportedMediaTypeError => Response::builder()
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
            ErrorResponse::TooManyUploadsError => {
                StatusCode::TOO_MANY_REQUESTS.into_response()
            }
            ErrorResponse::ValidationError(e) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(e.to_string()))
                .unwrap_or(StatusCode::BAD_REQUEST.into_response()),
            ErrorResponse::AuthError(e) => e.into_response(),
            ErrorResponse::NotFoundError(_, param) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(Body::from(format!("{{\"param\":{}}}", param)))
                .unwrap_or(StatusCode::NOT_FOUND.into_response()),
            ErrorResponse::ForbiddenError(_) => {
                StatusCode::FORBIDDEN.into_response()
            }
            ErrorResponse::ConflictError(_) => {
                StatusCode::CONFLICT.into_response()
            }
        }
    }
}

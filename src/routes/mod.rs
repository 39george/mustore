use axum::response::IntoResponse;
use axum::response::Response;
use http::StatusCode;

use crate::error_chain_fmt;

pub mod health_check;
pub mod open;
pub mod protected;

#[derive(thiserror::Error)]
pub enum ResponseError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Internal error")]
    InternalError(#[source] anyhow::Error),
    #[error("Bad request")]
    BadRequest(#[source] anyhow::Error),
    #[error("Can't process that input")]
    NotAcceptableError,
    #[error("No such user")]
    UnauthorizedError(#[source] anyhow::Error),
    #[error("Too many uploads for that user")]
    TooManyUploadsError,
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
            ResponseError::UnexpectedError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ResponseError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ResponseError::BadRequest(_) => {
                StatusCode::BAD_REQUEST.into_response()
            }
            ResponseError::UnauthorizedError(_) => {
                StatusCode::UNAUTHORIZED.into_response()
            }
            ResponseError::NotAcceptableError => {
                StatusCode::NOT_ACCEPTABLE.into_response()
            }
            ResponseError::TooManyUploadsError => {
                StatusCode::TOO_MANY_REQUESTS.into_response()
            }
        }
    }
}

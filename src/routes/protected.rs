use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::protected))
}

mod get {
    use super::*;

    pub async fn protected(
        auth_session: crate::auth::users::AuthSession,
    ) -> impl IntoResponse {
        match auth_session.user {
            Some(_user) => StatusCode::OK.into_response(),
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

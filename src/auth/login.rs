use anyhow::anyhow;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use http::StatusCode;
use secrecy::Secret;
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::DefaultOnRequest;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::TraceLayer;
use tower_http::LatencyUnit;
use tracing::Level;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::{users::AuthSession, AuthError};

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

// ───── Router ───────────────────────────────────────────────────────────── //

pub fn login_router() -> Router {
    Router::new()
        .route("/api/login", post(self::post::login))
        .route("/api/logout", get(self::get::logout))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        DefaultMakeSpan::new().include_headers(true),
                    )
                    .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::DEBUG)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            ),
        )
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

mod post {

    use axum::Json;

    use super::*;

    #[tracing::instrument(name = "Login attempt", skip_all)]
    pub async fn login(
        mut auth_session: AuthSession,
        Json(creds): Json<Credentials>,
    ) -> Result<StatusCode, AuthError> {
        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Ok(StatusCode::UNAUTHORIZED);
            }
            Err(e) => return Err(AuthError::UnexpectedError(anyhow!("{e}"))),
        };

        if auth_session.login(&user).await.is_err() {
            Err(AuthError::UnexpectedError(anyhow!("Internal error")))
        } else {
            Ok(StatusCode::OK)
        }
    }
}

mod get {

    use super::*;

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout().await {
            // FIX: write where to redirect to
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

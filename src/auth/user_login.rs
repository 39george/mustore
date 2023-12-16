use anyhow::anyhow;
use anyhow::Context;
use axum::response::IntoResponse;
use axum::response::Redirect;
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use base64::Engine;
use http::HeaderMap;
use http::StatusCode;
use secrecy::Secret;
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

#[derive(Clone)]
pub struct UserCredentials {
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
                    ), // on so on for `on_eos`, `on_body_chunk`, and `on_failure`
            ),
        )
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

mod post {

    use super::*;

    #[tracing::instrument(name = "Login attempt", skip_all)]
    pub async fn login(
        mut auth_session: AuthSession,
        headers: HeaderMap,
    ) -> Result<StatusCode, AuthError> {
        let creds = basic_authentication(&headers)
            .map_err(AuthError::UnexpectedError)?;

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
        match auth_session.logout() {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

// ───── Helpers ──────────────────────────────────────────────────────────── //

fn basic_authentication(
    headers: &HeaderMap,
) -> Result<UserCredentials, anyhow::Error> {
    let header_value = headers
        .get("Authorization")
        .context("The 'Authorization' header was missing")?
        .to_str()
        .context("The 'Authorization' header was not a valid UTF8 string.")?;
    let base64encoded_segment = header_value
        .strip_prefix("Basic")
        .context("The authorization scheme was not 'Basic")?
        .trim();
    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(base64encoded_segment)
        .context("The decoded credential string is not a valid UTF 8.")?;
    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF8")?;

    // Split into two segments using ':' as delimiter
    let mut credentials = decoded_credentials.splitn(2, ':');
    let username = credentials
        .next()
        .ok_or_else(|| {
            anyhow::anyhow!("A username must be provided in 'Basic' auth.")
        })?
        .to_string();
    let password = credentials
        .next()
        .ok_or_else(|| {
            anyhow::anyhow!("A password must be provided in 'Basic' auth.")
        })?
        .to_string();
    Ok(UserCredentials {
        username,
        password: Secret::new(password),
    })
}

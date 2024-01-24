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
use utoipa::IntoParams;
use utoipa::ToSchema;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::{users::AuthSession, AuthError};
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct Credentials {
    #[schema(
        min_length = 3,
        max_length = 256,
        pattern = r#"[^/()"<>\\{};:]*"#,
        example = "user1"
    )]
    pub username: String,
    #[schema(min_length = 8, max_length = 32, format = Password)]
    pub password: Secret<String>,
}

#[derive(Clone, Deserialize, ToSchema, IntoParams)]
pub struct Username {
    #[param(
        min_length = 3,
        max_length = 256,
        pattern = r#"[^/()"<>\\{};:]*"#,
        example = "user1"
    )]
    username: String,
}

// ───── Router ───────────────────────────────────────────────────────────── //

pub fn login_router(state: AppState) -> Router {
    Router::new()
        .route("/api/login", post(self::post::login))
        .route("/api/logout", get(self::get::logout))
        .route("/api/username_status", get(self::get::username_status))
        .with_state(state)
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

pub mod post {

    use axum::Json;
    use secrecy::ExposeSecret;

    use crate::{
        domain::{user_name::UserName, user_password::UserPassword},
        startup::api_doc::{
            BadRequestResponse, InternalErrorResponse,
            UnauthorizedErrorResponse,
        },
    };

    use super::*;

    /// Login account
    ///
    /// Username is logged.
    #[utoipa::path(
        post,
        path = "/api/login",
        request_body(
            content = Credentials,
            content_type = "application/json",
        ),
        responses(
            (status = 200, description = "Login success",
                headers(
                    ("set-cookie", description = "Set auth cookie token")
                )
            ),
            (status = 401, response = UnauthorizedErrorResponse),
            (status = 400, response = BadRequestResponse),
            (status = 500, response = InternalErrorResponse)
        ),
        tag = "open"
    )]
    #[tracing::instrument(name = "Login attempt", skip(auth_session, creds), fields(username = %creds.username))]
    pub async fn login(
        mut auth_session: AuthSession,
        Json(creds): Json<Credentials>,
    ) -> Result<StatusCode, AuthError> {
        let _ = UserName::parse(&creds.username)
            .map_err(AuthError::ValidationError)?;
        UserPassword::parse(creds.password.expose_secret(), &[])
            .map_err(AuthError::ValidationError)?;

        let user = match auth_session.authenticate(creds.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return Ok(StatusCode::UNAUTHORIZED);
            }
            Err(e) => {
                return Err(AuthError::InvalidCredentialsError(anyhow!("{e}")))
            }
        };

        if auth_session.login(&user).await.is_err() {
            Err(AuthError::UnexpectedError(anyhow!("Internal error")))
        } else {
            Ok(StatusCode::OK)
        }
    }
}

pub mod get {

    use std::collections::HashMap;

    use anyhow::Context;
    use axum::{
        extract::{Query, State},
        Json,
    };

    use crate::{
        cornucopia::queries::user_auth_queries,
        startup::{
            api_doc::{
                BadRequestResponse, InternalErrorResponse, IsExistsResponse,
            },
            AppState,
        },
    };

    use super::*;

    /// Log out from account
    #[utoipa::path(
        get,
        path = "/api/logout",
        responses(
            (status = 303, description = "Logout success"),
            (status = 500, response = InternalErrorResponse)
        ),
        tag = "open"
    )]
    #[tracing::instrument(
        name = "Logout",
        skip(auth_session),
        fields(username)
    )]
    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        if let Some(ref user) = auth_session.user {
            tracing::Span::current().record("username", &user.username);
        }
        match auth_session.logout().await {
            // FIXME: write where to redirect to
            Ok(_) => Redirect::to("/").into_response(),
            Err(e) => {
                tracing::error!("Failed to logout: {e}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }

    /// Check if username is occupied
    #[utoipa::path(
        get,
        path = "/api/username_status",
        params(
            Username
        ),
        responses(
            (status = 200, response = IsExistsResponse),
            (status = 400, response = BadRequestResponse),
            (status = 500, response = InternalErrorResponse)
        ),
        tag = "open"
    )]
    pub async fn username_status(
        Query(username): Query<Username>,
        State(app_state): State<AppState>,
    ) -> Result<Json<HashMap<&'static str, bool>>, AuthError> {
        let pg_client = app_state
            .pg_pool
            .get()
            .await
            .context("Failed to get  pg connection from pool")
            .map_err(AuthError::InternalError)?;

        let _ = crate::domain::user_name::UserName::parse(&username.username)
            .map_err(AuthError::ValidationError)?;

        let exists = user_auth_queries::check_if_username_occupied()
            .bind(&pg_client, &username.username)
            .opt()
            .await
            .context("Failed to get username from pg")?;
        let mut map = HashMap::new();
        map.insert("exists", if exists.is_some() { true } else { false });
        Ok(Json(map))
    }
}

//! src/auth/user_signup.rs

use std::net::SocketAddr;

use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use askama::Template;
use axum::extract::ConnectInfo;
use axum::extract::State;
use axum::Form;
use http::StatusCode;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use utoipa::ToSchema;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::AuthError;
use crate::cornucopia::queries::user_auth_queries;
use crate::domain::signup_token::SignupToken;
use crate::domain::user_candidate::store_user_candidate_data;
use crate::domain::user_candidate::UserCandidate;
use crate::domain::user_email::UserEmail;
use crate::domain::user_name::UserName;
use crate::domain::user_password::UserPassword;
use crate::domain::user_role::UserRole;
use crate::email_client::EmailClient;
use crate::startup::api_doc::BadRequestResponse;
use crate::startup::api_doc::InternalErrorResponse;
use crate::startup::AppState;
use crate::telemetry::spawn_blocking_with_tracing;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Deserialize, Debug, ToSchema)]
pub struct UserSignupData {
    /// This is username.
    #[schema(
        min_length = 3,
        max_length = 256,
        pattern = r#"[^/()"<>\\{};:]*"#
    )]
    username: String,
    /// This is password, complexity entropy should be not less than 4 (zxcvbn).
    #[schema(min_length = 11, max_length = 32, format = Password)]
    password: String,
    /// Should be a valid email.
    #[schema(format = "email")]
    email: String,
    user_role: Option<UserRole>,
    /// If user is admin, a valid admin token should be provided.
    /// User should have only a role, or only an admin token, not both!
    admin_token: Option<uuid::Uuid>,
    recaptcha_token: String,
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

/// Create a new user account
///
/// Username and email are logged.
#[utoipa::path(
    post,
    path = "/api/signup",
    request_body(
        content = UserSignupData,
        content_type = "application/x-www-form-urlencoded",
    ),
    responses(
        (status = 201, description = "Account created"),
        (status = 400, response = BadRequestResponse),
        (status = 403, description = "Recaptcha verification failed"),
        (status = 500, response = InternalErrorResponse)
    ),
    tag = "open"
)]
#[tracing::instrument(
    name = "Signup attempt",
    skip(app_state, password, user_role, admin_token, recaptcha_token)
)]
pub async fn signup(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Form(UserSignupData {
        username,
        password,
        email,
        user_role,
        admin_token,
        recaptcha_token,
    }): Form<UserSignupData>,
) -> Result<StatusCode, AuthError> {
    if user_role.is_some() && admin_token.is_some()
        || user_role.is_none() && admin_token.is_none()
    {
        return Err(AuthError::ValidationError(anyhow::anyhow!(
            "User should have only role, or only admin token!"
        )));
    }

    let username =
        UserName::parse(&username).map_err(AuthError::ValidationError)?;
    let email = UserEmail::parse(&email).map_err(AuthError::ValidationError)?;
    let password =
        UserPassword::parse(&password, &[username.as_ref(), email.as_ref()])
            .map_err(AuthError::ValidationError)?;

    let password_hash = spawn_blocking_with_tracing(move || {
        hash_password(password.as_ref(), app_state.argon2_obj)
    })
    .await
    .context("Failed to join thread")
    .map_err(AuthError::InternalError)??;

    app_state
        .captcha_verifier
        .validate(recaptcha_token, addr.ip())
        .await?;

    let pg_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get  pg connection from pool")
        .map_err(AuthError::InternalError)?;

    let if_email_exists = user_auth_queries::check_if_email_exists_already()
        .bind(&pg_client, &email.as_ref())
        .opt()
        .await
        .context("Failed to get user data from db")
        .map_err(AuthError::InternalError)?;

    if if_email_exists.is_some() {
        return Ok(StatusCode::CREATED);
    }

    let if_username_occupied = user_auth_queries::check_if_username_occupied()
        .bind(&pg_client, &username.as_ref())
        .opt()
        .await
        .context("Failed to get user data from db")
        .map_err(AuthError::InternalError)?;

    if if_username_occupied.is_some() {
        return Err(anyhow::anyhow!("User is already exists!"))
            .map_err(AuthError::SignupFailed);
    }

    let validation_token = SignupToken::generate();

    let user_candidate = UserCandidate::new(
        username.as_ref(),
        email.as_ref(),
        &password_hash.as_str(),
        user_role,
        validation_token.as_ref(),
        admin_token,
    );

    store_user_candidate_data(&app_state.redis_pool, user_candidate)
        .await
        .context("Failed to store user candidate data in redis")
        .map_err(AuthError::InternalError)?;

    send_confirmation_email(
        &app_state.email_client,
        &username,
        &email,
        &app_state.settings.app_base_url,
        &validation_token,
    )
    .await
    .context("Failed to send confirmation email")?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(name = "Performing hashing of password", skip_all)]
fn hash_password(
    password: &Secret<String>,
    argon2: argon2::Argon2,
) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    Ok(argon2
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .context("Failed to hash password")
        .map_err(AuthError::SignupFailed)?
        .to_string())
}

#[tracing::instrument(
    name = "Send a confirmation email to a new user",
    skip_all
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    username: &UserName,
    user_email: &UserEmail,
    base_url: &str,
    signup_token: &SignupToken,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/api/confirm_user_account?email={}&token={}",
        base_url,
        user_email.as_ref(),
        signup_token.as_ref()
    );
    let html_body = crate::html_template_gen::VerifyEmailTemplate::new(
        username.as_ref(),
        &confirmation_link,
    )
    .render()
    .unwrap();
    let plain_body = format!(
        "Welcome to mustore!\nVisit {} to confirm your account.",
        confirmation_link
    );
    email_client
        .send_email(&user_email, "Welcome!", &html_body, &plain_body)
        .await
}

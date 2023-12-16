use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use askama::Template;
use axum::extract::State;
use axum::Form;
use http::StatusCode;
use redis::AsyncCommands;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde::Serialize;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::AuthError;
use super::UserCandidate;
use super::UserRole;
use crate::cornucopia::queries::user_auth_queries;
use crate::domain::user_email::UserEmail;
use crate::domain::user_name::UserName;
use crate::domain::user_password::UserPassword;
use crate::email_client::EmailClient;
use crate::startup::AppState;
use crate::telemetry::spawn_blocking_with_tracing;
use crate::validation::signup_token::SignupToken;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Clone, Serialize, Deserialize)]
pub struct UserSignupData {
    username: String,
    password: String,
    email: String,
    user_role: UserRole,
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

#[tracing::instrument(name = "Signup attempt", skip_all)]
pub async fn signup(
    State(app_state): State<AppState>,
    Form(UserSignupData {
        username,
        password,
        email,
        user_role,
    }): Form<UserSignupData>,
) -> Result<StatusCode, AuthError> {
    let username =
        UserName::parse(&username).map_err(AuthError::SignupFailed)?;
    let email = UserEmail::parse(&email).map_err(AuthError::SignupFailed)?;
    let password =
        UserPassword::parse(&password, &[username.as_ref(), email.as_ref()])
            .map_err(AuthError::SignupFailed)?;

    let password_hash = spawn_blocking_with_tracing(move || {
        hash_password(password.as_ref(), app_state.argon2_obj)
    })
    .await
    .context("Failed to join thread")
    .map_err(AuthError::InternalError)??;

    let pg_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get  pg connection from pool")
        .map_err(AuthError::InternalError)?;

    let users_count = user_auth_queries::check_if_user_exists_already()
        .bind(&pg_client, &username.as_ref(), &email.as_ref())
        .one()
        .await
        .context("Failed to get user data from db")
        .map_err(AuthError::SignupFailed)?;

    if users_count != 0 {
        return Err(anyhow::anyhow!("User is already exists!"))
            .map_err(AuthError::SignupFailed);
    }

    let mut redis_client = app_state
        .redis_pool
        .get()
        .await
        .context("Failed to get redis connection from pool")
        .map_err(AuthError::InternalError)?;

    let validation_token = SignupToken::generate();

    let user_candidate = UserCandidate::new(
        username.as_ref(),
        email.as_ref(),
        &password_hash.as_str(),
        user_role,
        validation_token.as_ref(),
    );

    set_user_candidate_data(&mut redis_client, email.as_ref(), user_candidate)
        .await
        .context("Failed to store user candidate data in redis")
        .map_err(AuthError::SignupFailed)?;

    send_confirmation_email(
        &app_state.email_client,
        &username,
        &email,
        &app_state.base_url,
        &validation_token,
    )
    .await
    .context("Failed to send confirmation email")?;

    Ok(StatusCode::CREATED)
}

/// By default if the given `user_email` already exists,
/// value will be overwritten.
async fn set_user_candidate_data(
    con: &mut redis::aio::Connection,
    user_email: &str,
    user_candidate: UserCandidate,
) -> redis::RedisResult<()> {
    let key = format!("user_candidate:{}", user_email);
    con.hset_multiple(&key, &user_candidate.to_redis_fields())
        .await?;
    con.expire(&key, 60 * 30).await?; // 30 minutes
    Ok(())
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
        "{}/confirm_user_account?email={}&token={}",
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

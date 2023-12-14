use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use axum::extract::State;
use axum::Form;
use http::StatusCode;
use secrecy::ExposeSecret;

use crate::domain::user_email::UserEmail;
use crate::domain::user_name::UserName;
use crate::domain::user_password::UserPassword;
use crate::startup::AppState;

use super::{AuthError, UserSignupData};

#[tracing::instrument(name = "Signup attempt", skip_all)]
pub async fn signup(
    State(app_state): State<AppState>,
    Form(UserSignupData {
        username,
        password,
        email,
    }): Form<UserSignupData>,
) -> Result<StatusCode, AuthError> {
    let username = UserName::parse(&username)
        .map_err(AuthError::InvalidCredentialsError)?;
    let email =
        UserEmail::parse(&email).map_err(AuthError::InvalidCredentialsError)?;
    let password =
        UserPassword::parse(&password, &[username.as_ref(), email.as_ref()])
            .map_err(AuthError::InvalidCredentialsError)?;

    let salt = SaltString::generate(&mut rand::thread_rng());

    // TODO: benchmark this function. Should we do it in a dedicated thread?
    let password_hash = app_state
        .argon2_obj
        .hash_password(password.as_ref().expose_secret().as_bytes(), &salt)
        .context("Failed to hash password")?
        .to_string();

    let client = app_state.pool.get().await.map_err(|e| {
        tracing::error!("{e}");
        AuthError::InternalError
    })?;

    let rows_count = client
        .execute(
            "INSERT INTO users (username, email, password_hash)
                    VALUES ($1, $2, $3)",
            &[&username.as_ref(), &email.as_ref(), &password_hash],
        )
        .await
        .expect("Failed to create test users");

    if rows_count != 1 {
        return Err(AuthError::UserCreationFailed);
    }

    Ok(StatusCode::CREATED)
}

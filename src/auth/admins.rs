use anyhow::Context;
use argon2::{PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use deadpool_postgres::Pool;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

use crate::{startup::AppState, telemetry::spawn_blocking_with_tracing};

use super::AuthError;
use crate::auth::user_login::UserCredentials;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    password_hash: String,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // We use the password hash as the auth
        // hash--what this means
        // is when the user changes their password the
        // auth session becomes invalid.
        self.password_hash.as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct UserBackend {
    app_state: AppState,
}

#[async_trait]
impl AuthnBackend for UserBackend {
    type User = User;
    type Credentials = UserCredentials;
    type Error = AuthError;

    #[tracing::instrument(name = "Authenticate user", skip(self, creds))]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user =
            get_user_data(&self.app_state.pg_pool, Some(&creds.username), None)
                .await;

        let provided_password = creds.password.clone();
        let argon2_obj = self.app_state.argon2_obj.clone();
        let expected_password_hash = if let Ok(ref user) = user {
            Secret::new(user.password_hash.clone())
        } else {
            // We do it to prevent hacking by hash time
            Secret::new(
                "$argon2id$v=19$m=15000,t=2,p=1$\
            gZiV/M1gPc22ElAH/Jh1Hw$\
            CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
                    .to_string(),
            )
        };
        spawn_blocking_with_tracing(move || {
            verify_password_hash(
                expected_password_hash,
                provided_password,
                argon2_obj,
            )
        })
        .await
        .context("Invalid password.")
        .map_err(AuthError::UnexpectedError)??;
        Ok(Some(user?))
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user =
            get_user_data(&self.app_state.pg_pool, None, Some(user_id)).await?;
        Ok(Some(user))
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
#[allow(dead_code)]
pub type AuthSession = axum_login::AuthSession<UserBackend>;

#[tracing::instrument(name = "Get user data from db", skip_all)]
async fn get_user_data(
    pool: &Pool,
    username: Option<&str>,
    id: Option<&i32>,
) -> Result<User, AuthError> {
    let connection = pool
        .get()
        .await
        .context("Failed to get connection from db pool")?;

    let row = match (username, id) {
        (Some(username), None) => connection
            .query_one(
                "
                SELECT id, username, password_hash
                FROM users
                WHERE username = $1
                ",
                &[&username],
            )
            .await
            .context("Failed to query user from db")
            .map_err(AuthError::InvalidCredentialsError)?,
        (None, Some(id)) => connection
            .query_one(
                "
                SELECT id, username, password_hash
                FROM users
                WHERE id = $1
                ",
                &[&id],
            )
            .await
            .context("Failed to query user from db")
            .map_err(AuthError::InvalidCredentialsError)?,
        _ => unreachable!(),
    };

    let password_hash = row
        .try_get::<&str, &str>("password_hash")
        .context("Failed to get password from row")
        .map_err(AuthError::UnexpectedError)?;

    let id = row
        .try_get("id")
        .context("Failed to get user id from row")
        .map_err(AuthError::UnexpectedError)?;

    let username = row
        .try_get("username")
        .context("Failed to get username from row")
        .map_err(AuthError::UnexpectedError)?;

    Ok(User {
        id,
        username,
        password_hash: password_hash.to_string(),
    })
}

#[tracing::instrument(name = "Verify password hash", skip_all)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
    argon2: argon2::Argon2,
) -> Result<(), AuthError> {
    let expected_password_hash =
        PasswordHash::new(&expected_password_hash.expose_secret())
            .context("Failed to parse hash in PHC string format.")
            .map_err(AuthError::UnexpectedError)?;
    argon2
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password")
        .map_err(AuthError::InvalidCredentialsError)
}

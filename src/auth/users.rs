use std::collections::HashSet;

use anyhow::Context;
use argon2::{PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use deadpool_postgres::Pool;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::AuthError;
use crate::auth::login::Credentials;
use crate::cornucopia::queries::user_auth_queries;
use crate::startup::AppState;
use crate::telemetry::spawn_blocking_with_tracing;

// ───── Body ─────────────────────────────────────────────────────────────── //

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    username: String,
    #[serde(
        serialize_with = "serialize_secret",
        deserialize_with = "deserialize_secret"
    )]
    password_hash: Secret<String>,
}

// Serialize the secret revealing its content
fn serialize_secret<S>(
    secret: &Secret<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

// Deserialize the secret, consuming the revealed content immediately
fn deserialize_secret<'de, D>(
    deserializer: D,
) -> Result<Secret<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Secret::new(s))
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
        self.password_hash.expose_secret().as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    app_state: AppState,
}

impl Backend {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
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
            user.password_hash.clone()
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
pub type AuthSession = axum_login::AuthSession<Backend>;

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

    let data = match (username, id) {
        (Some(username), None) => {
            user_auth_queries::get_auth_user_data_by_username()
                .bind(&connection, &username)
                .opt()
                .await
                .context("Failed to query user from db by username")
                .map_err(AuthError::InvalidCredentialsError)?
                .map(|r| (r.id, r.username, Secret::new(r.password_hash)))
        }
        (None, Some(id)) => user_auth_queries::get_auth_user_data_by_id()
            .bind(&connection, id)
            .opt()
            .await
            .context("Failed to query user from db by id")
            .map_err(AuthError::InvalidCredentialsError)?
            .map(|r| (r.id, r.username, Secret::new(r.password_hash))),
        _ => unreachable!(),
    };

    let (id, username, password_hash) = data.ok_or(
        AuthError::UnexpectedError(anyhow::anyhow!("No data for user in db")),
    )?;

    Ok(User {
        id,
        username,
        password_hash,
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Permission {
    pub name: String,
}

impl<T> From<T> for Permission
where
    T: std::fmt::Display,
{
    fn from(name: T) -> Self {
        Permission {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl AuthzBackend for Backend {
    type Permission = Permission;

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let pg_client = self
            .app_state
            .pg_pool
            .get()
            .await
            .context("Failed to get connection from db pool")?;
        let permissions = user_auth_queries::get_user_permissions()
            .bind(&pg_client, &user.id)
            .all()
            .await
            .context("Failed to get user permissions from pg")?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(permissions)
    }
}

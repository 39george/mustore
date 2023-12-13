use core::panic;

use argon2::{PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use deadpool_postgres::Pool;
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};

use super::Credentials;

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    id: i32,
    pub username: String,
    password: String,
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
        self.password.as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct UserBackend {
    db: Pool,
}

impl UserBackend {
    pub fn new(db: Pool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthnBackend for UserBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = tokio_postgres::Error;

    #[tracing::instrument(name = "Authenticate user", skip(self, creds))]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // FIXME: unwrap
        let connection = self.db.get().await.unwrap();
        let row = connection
            .query_one(
                "
                SELECT id, username, password
                FROM users
                WHERE username = $1
                ",
                &[&creds.username],
            )
            .await?;

        let password = row.try_get::<&str, &str>("password")?;
        dbg!(&password);

        let user = User {
            id: row.try_get::<&str, i32>("id")?,
            username: row.try_get("username")?,
            password: password.to_string(),
        };

        // let expected_password_hash =
        //     PasswordHash::new(&user.password).unwrap();

        // match argon2::Argon2::default().verify_password(
        //     creds.password.as_bytes(),
        //     &expected_password_hash,
        // ) {
        //     Ok(()) => Ok(Some(user)),
        //     Err(e) => panic!(),
        // }
        if password == creds.password.expose_secret() {
            Ok(Some(user))
        } else {
            panic!()
        }
    }

    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let connection = self.db.get().await.unwrap();
        let row = connection
            .query_one(
                "
                SELECT id, username, password
                FROM users
                WHERE id = $1
                ",
                &[user_id],
            )
            .await?;
        let user = User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
        };
        Ok(Some(user))
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<UserBackend>;

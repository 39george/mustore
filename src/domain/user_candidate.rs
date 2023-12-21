use fred::prelude::RedisError;
use fred::prelude::RedisErrorKind;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use super::user_role::UserRole;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserCandidate {
    pub username: String,
    pub email: String,
    // We need to serialize that, so don't use `Secret`
    pub password_hash: String,
    pub validation_token: String,
    pub role: Option<UserRole>,
    pub admin_token: Option<uuid::Uuid>,
}

impl UserCandidate {
    pub fn new(
        username: &str,
        email: &str,
        password_hash: &str,
        role: Option<UserRole>,
        validation_token: &str,
        admin_token: Option<uuid::Uuid>,
    ) -> Self {
        UserCandidate {
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            role,
            validation_token: validation_token.to_string(),
            admin_token,
        }
    }
}

impl TryFrom<HashMap<String, String>> for UserCandidate {
    type Error = RedisError;
    fn try_from(
        mut value: HashMap<String, String>,
    ) -> Result<Self, Self::Error> {
        Ok(UserCandidate {
            username: value.remove("username").ok_or_else(|| {
                RedisError::new(
                    RedisErrorKind::NotFound,
                    "Missing field: username",
                )
            })?,
            email: value.remove("email").ok_or_else(|| {
                RedisError::new(
                    RedisErrorKind::NotFound,
                    "Missing field: email",
                )
            })?,
            password_hash: value.remove("password_hash").ok_or_else(|| {
                RedisError::new(
                    RedisErrorKind::NotFound,
                    "Missing field: password_hash",
                )
            })?,
            role: value
                .remove("role")
                .and_then(|r| UserRole::try_from(r.as_str()).ok()),
            validation_token: value.remove("validation_token").ok_or_else(
                || {
                    RedisError::new(
                        RedisErrorKind::NotFound,
                        "Missing field: validation_token",
                    )
                },
            )?,
            admin_token: value
                .remove("admin_token")
                .map(|v| {
                    uuid::Uuid::parse_str(&v).map_err(|e| {
                        RedisError::new(RedisErrorKind::Parse, e.to_string())
                    })
                })
                .transpose()?,
        })
    }
}

impl From<UserCandidate> for HashMap<String, String> {
    fn from(value: UserCandidate) -> Self {
        let mut map = HashMap::new();
        map.insert("username".to_string(), value.username);
        map.insert("email".to_string(), value.email);
        map.insert("password_hash".to_string(), value.password_hash);
        if let Some(role) = value.role {
            map.insert("role".to_string(), role.to_string());
        } else if let Some(admin_token) = value.admin_token {
            map.insert(
                "admin_token".to_string(),
                admin_token.hyphenated().to_string(),
            );
        }
        map.insert("validation_token".to_string(), value.validation_token);
        map
    }
}

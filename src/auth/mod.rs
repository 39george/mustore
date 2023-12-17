//! src/auth/mod.rs

use std::collections::HashMap;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use fred::error::RedisError;
use fred::error::RedisErrorKind;
use http::header::WWW_AUTHENTICATE;
use http::HeaderValue;
use secrecy::Secret;
use serde::Deserialize;
use serde::Serialize;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::cornucopia::types::public::Userrole;
use crate::error_chain_fmt;

// ───── Submodules ───────────────────────────────────────────────────────── //

mod admins;
pub mod user_confirm_account;
pub mod user_login;
pub mod user_signup;
pub mod users;

// ───── Auth Types ───────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
pub enum AuthError {
    #[error("Invalid credentials: {0}")]
    InvalidCredentialsError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Internal error")]
    InternalError(#[source] anyhow::Error),
    #[error("User creation failed: {0}")]
    SignupFailed(#[source] anyhow::Error),
    #[error("Account confirmation failed: {0}")]
    AccountConfirmationFailed(#[source] anyhow::Error),
}

impl std::fmt::Debug for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        match self {
            AuthError::UnexpectedError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AuthError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AuthError::InvalidCredentialsError(_) => Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .header(
                    WWW_AUTHENTICATE,
                    HeaderValue::from_static(
                        r#"Basic realm="Mustore User Access""#,
                    ),
                )
                .body(axum::body::Body::empty())
                .unwrap(),
            AuthError::SignupFailed(_) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Failed to signup".into())
                .unwrap(),
            AuthError::AccountConfirmationFailed(_) => {
                axum::response::Redirect::to(
                    // FIXME: replace this to the real path
                    "react-router/accountconfirmationfailed",
                )
                .into_response()
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum UserRole {
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "fullstack")]
    Fullstack,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Creator => f.write_str("creator"),
            UserRole::Consumer => f.write_str("consumer"),
            UserRole::Fullstack => f.write_str("fullstack"),
        }
    }
}

impl TryFrom<&str> for UserRole {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "creator" => Ok(UserRole::Creator),
            "consumer" => Ok(UserRole::Consumer),
            "fullstack" => Ok(UserRole::Fullstack),
            other => {
                Err(format!("Can't create UserRole instance from {other}"))
            }
        }
    }
}

impl Into<Userrole> for UserRole {
    fn into(self) -> Userrole {
        match self {
            UserRole::Creator => Userrole::creator,
            UserRole::Consumer => Userrole::consumer,
            UserRole::Fullstack => Userrole::fullstack,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserCandidate {
    username: String,
    email: String,
    // We need to serialize that, so don't use `Secret`
    password_hash: String,
    role: UserRole,
    validation_token: String,
}

impl UserCandidate {
    pub fn new(
        username: &str,
        email: &str,
        password_hash: &str,
        role: UserRole,
        validation_token: &str,
    ) -> Self {
        UserCandidate {
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            role,
            validation_token: validation_token.to_string(),
        }
    }

    pub fn to_redis_fields(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("username".to_string(), self.username);
        map.insert("email".to_string(), self.email);
        map.insert("password_hash".to_string(), self.password_hash);
        map.insert("role".to_string(), self.role.to_string());
        map.insert("validation_token".to_string(), self.validation_token);
        map
    }

    fn from_map(
        mut fields: HashMap<String, String>,
    ) -> Result<UserCandidate, RedisError> {
        Ok(UserCandidate {
            username: fields.remove("username").ok_or_else(|| {
                RedisError::new(
                    RedisErrorKind::NotFound,
                    "Missing field: username",
                )
            })?,
            email: fields.remove("email").ok_or_else(|| {
                RedisError::new(
                    RedisErrorKind::NotFound,
                    "Missing field: email",
                )
            })?,
            password_hash: fields.remove("password_hash").ok_or_else(|| {
                RedisError::new(
                    RedisErrorKind::NotFound,
                    "Missing field: password_hash",
                )
            })?,
            role: fields
                .remove("role")
                .and_then(|r| UserRole::try_from(r.as_str()).ok())
                .ok_or_else(|| {
                    RedisError::new(
                        RedisErrorKind::NotFound,
                        "Invalid or missing field: role",
                    )
                })?,
            validation_token: fields.remove("validation_token").ok_or_else(
                || {
                    RedisError::new(
                        RedisErrorKind::NotFound,
                        "Missing field: validation_token",
                    )
                },
            )?,
        })
    }
}

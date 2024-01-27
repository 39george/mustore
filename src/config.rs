//! src/config.rs

use std::{net::Ipv4Addr, path::Path};

use anyhow::Context;
use config::FileFormat;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

use crate::domain::user_email::UserEmail;
use crate::email_client::EmailDeliveryService;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub app_port: u16,
    pub app_addr: Ipv4Addr,
    pub app_base_url: String,
    pub email_client: EmailClientSettings,
    pub email_delivery_service: EmailDeliveryService,
    pub object_storage: ObjectStorageSettings,
    pub recaptcha: RecaptchaSettings,
}

impl Settings {
    pub fn load_configuration() -> Result<Settings, anyhow::Error> {
        let config_file = std::env::var("APP_CONFIG_FILE")
            .expect("APP_CONFIG_FILE var is unset!");

        config::Config::builder()
            .add_source(config::File::new(&config_file, FileFormat::Yaml))
            .build()?
            .try_deserialize()
            .context("Failed to build config from local config file.")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(default = "pg_username")]
    pub username: String,
    #[serde(default = "pg_password")]
    pub password: Secret<String>,
    #[serde(default = "pg_db_name")]
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> secrecy::Secret<String> {
        secrecy::Secret::new(format!(
            "user={} password={} dbname={} host={} application_name={}",
            self.username,
            self.password.expose_secret(),
            self.database_name,
            self.host,
            "zero2prod"
        ))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub db_number: u16,
    #[serde(default = "redis_password")]
    pub password: Secret<String>,
}

impl RedisSettings {
    pub fn connection_string(&self) -> secrecy::Secret<String> {
        secrecy::Secret::new(format!(
            "redis://:{}@{}:{}/{}",
            self.password.expose_secret(),
            self.host,
            self.port,
            self.db_number
        ))
    }
}

/// This type describes configuration
/// for client, sending emails.
#[derive(Debug, Deserialize)]
pub struct EmailClientSettings {
    /// Email delivery service we use to relay email
    pub base_url: String,
    /// This host email address
    pub sender_email: String,
    /// Token to authorize in API
    #[serde(default = "email_token")]
    pub authorization_token: Secret<String>,
    /// `request` crate will wait until this timeout when sends emails
    timeout: u64,
}

impl EmailClientSettings {
    /// Try to parse email from `String` type to safe `UserEmail`.
    pub fn sender(&self) -> Result<UserEmail, anyhow::Error> {
        UserEmail::parse(&self.sender_email)
    }

    pub fn timeout_millis(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.timeout)
    }
}

#[derive(Debug, Deserialize)]
pub struct ObjectStorageSettings {
    pub endpoint_url: String,
    pub region: String,
    pub bucket_name: String,
    #[serde(default = "object_storage_key_id")]
    pub access_key_id: Secret<String>,
    #[serde(default = "object_storage_acces_key")]
    pub secret_access_key: Secret<String>,
}

#[derive(Debug, Deserialize)]
pub struct RecaptchaSettings {
    pub endpoint_url: String,
    #[serde(default = "recaptcha_secret_key")]
    pub secret: Secret<String>,
}

fn load_value_from_file<T: AsRef<Path>>(
    path: T,
) -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string(path)?.trim().to_string())
}

fn pg_username() -> String {
    std::env::var("POSTGRES_USER").expect("POSTGRES_USER var is unset!")
}

fn pg_db_name() -> String {
    std::env::var("POSTGRES_DB").expect("POSTGRES_DB var is unset!")
}

fn pg_password() -> Secret<String> {
    Secret::new(
        load_value_from_file(
            std::env::var("POSTGRES_PASSWORD_FILE")
                .expect("POSTGRES_PASSWORD_FILE var is unset!"),
        )
        .expect("Can't read postgres password file!"),
    )
}

fn redis_password() -> Secret<String> {
    Secret::new(
        load_value_from_file(
            std::env::var("REDIS_PASSWORD_FILE")
                .expect("REDIS_PASSWORD_FILE var is unset!"),
        )
        .expect("Can't read redis password file!"),
    )
}

fn email_token() -> Secret<String> {
    Secret::new(
        load_value_from_file(
            std::env::var("EMAIL_AUTHORIZATION_TOKEN_FILE")
                .expect("EMAIL_AUTHORIZATION_TOKEN_FILE var is unset!"),
        )
        .expect("Can't read email token file!"),
    )
}

fn object_storage_key_id() -> Secret<String> {
    Secret::new(
        load_value_from_file(
            std::env::var("OBJECT_STORAGE_KEY_ID_FILE")
                .expect("OBJECT_STORAGE_KEY_ID_FILE var is unset!"),
        )
        .expect("Can't read object-storage-key-id file!"),
    )
}

fn object_storage_acces_key() -> Secret<String> {
    Secret::new(
        load_value_from_file(
            std::env::var("OBJECT_STORAGE_ACCESS_KEY_FILE")
                .expect("OBJECT_STORAGE_ACCESS_KEY_FILE var is unset!"),
        )
        .expect("Can't read object-storage-access-key file!"),
    )
}

fn recaptcha_secret_key() -> Secret<String> {
    Secret::new(
        load_value_from_file(
            std::env::var("RECAPTCHA_SECRET_KEY_FILE")
                .expect("RECAPTCHA_SECRET_KEY_FILE var is unset!"),
        )
        .expect("Can't read email token file!"),
    )
}

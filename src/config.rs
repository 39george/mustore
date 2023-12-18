//! src/config.rs

use std::{net::Ipv4Addr, path::Path};

use anyhow::Context;
use config::FileFormat;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

use crate::domain::user_email::UserEmail;
use crate::email_client::EmailDeliveryService;

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.", other)),
        }
    }
}

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
}

impl Settings {
    pub fn load_configuration() -> Result<Settings, anyhow::Error> {
        let base_path = std::env::current_dir()
            .expect("Failed to determine the current directory");
        let configuration_directory = base_path.join("config");
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .expect("Failed to parse APP_ENVIRONMENT.");

        match environment {
            Environment::Local => config::Config::builder()
                .add_source(config::File::new(
                    configuration_directory
                        .join(environment.as_str())
                        .to_str()
                        .unwrap(),
                    FileFormat::Yaml,
                ))
                .build()?
                .try_deserialize()
                .context("Failed to build config from local config file."),
            Environment::Production => {
                let path = std::env::var("APP_CONFIG")?;
                let config = load_config_from_file(path)?;
                serde_yaml::from_str(&config)
                    .context("Failed to build config from env variable")
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub host: String,
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
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub db_number: u16,
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
    pub access_key_id: Secret<String>,
    pub secret_access_key: Secret<String>,
}

fn load_config_from_file<T: AsRef<Path>>(
    path: T,
) -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string(path)?.trim().to_string())
}

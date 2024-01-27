use std::net::IpAddr;

use reqwest::Url;
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::error_chain_fmt;

#[derive(thiserror::Error, Debug, Deserialize)]
pub enum GoogleCaptchaError {
    #[serde(rename = "missing-input-secret")]
    #[error("The secret parameter is missing.")]
    MissingInputSecret,
    #[serde(rename = "invalid-input-secret")]
    #[error("The secret parameter is invalid or malformed.")]
    InvalidInputSecret,
    #[serde(rename = "missing-input-response")]
    #[error("The response parameter is missing.")]
    MissingInputResponse,
    #[serde(rename = "invalid-input-response")]
    #[error("The response parameter is invalid or malformed.")]
    InvalidInputResponse,
    #[serde(rename = "bad-request")]
    #[error("The request is invalid or malformed.")]
    BadRequest,
    #[serde(rename = "timeout-or-duplicate")]
    #[error("The response is no longer valid: either is too old or has been used previously.")]
    TimeoutOrDuplicate,
}

#[derive(thiserror::Error)]
pub enum CaptchaError {
    #[error(transparent)]
    ClientError(#[from] reqwest::Error),
    #[error("Service response says that success is false")]
    VerificationFailed,
}

impl std::fmt::Debug for CaptchaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Clone, Debug)]
pub struct CaptchaVerifier {
    service_addr: Url,
    client: reqwest::Client,
    secret: Secret<String>,
}

#[derive(Deserialize)]
struct CaptchaResponse {
    success: bool,
    /// Timestamp of the challenge load (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
    challenge_ts: Option<OffsetDateTime>,
    /// The hostname of the site where the reCAPTCHA was solved
    hostname: String,
    #[serde(rename = "error-codes")]
    error_codes: Option<Vec<String>>,
}

impl CaptchaVerifier {
    pub fn new(service_addr: Url, secret: Secret<String>) -> Self {
        CaptchaVerifier {
            service_addr,
            client: reqwest::Client::new(),
            secret,
        }
    }

    pub async fn validate(&self, token: String, ip: IpAddr) -> Result<(), CaptchaError> {
        let query = &[
            ("secret", self.secret.expose_secret()),
            ("response", &token),
            ("remoteip", &ip.to_string()),
        ];
        let response: CaptchaResponse = self
            .client
            .post(self.service_addr.clone())
            .query(query)
            .send()
            .await?
            .json()
            .await?;

        if !response.success {
            tracing::warn!("Captcha was not passed: {:?}", response.error_codes);
            Err(CaptchaError::VerificationFailed)
        } else {
            Ok(())
        }
    }
}

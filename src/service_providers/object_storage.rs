//! src/service_providers/object_storage.rs

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{config::Credentials, Client};
use secrecy::ExposeSecret;

use crate::config::ObjectStorageSettings;

/// Handle to work with Yandex object storage.
/// Client internally uses Arc, so clone is ok.
#[derive(Clone, Debug)]
pub struct YandexObjectStorage {
    client: Client,
}

impl YandexObjectStorage {
    pub async fn new(settings: ObjectStorageSettings) -> Self {
        let yandex_endpoint = "https://storage.yandexcloud.net";

        // Create Credentials object directly.
        let credentials = Credentials::new(
            settings.access_key_id.expose_secret(),
            settings.secret_access_key.expose_secret(),
            None, // Token, if you are using temporary credentials you can set it here
            None, // Expiry, this would be used for temporary credentials
            "provided-statically", // This is an arbitrary name for the credential provider
        );

        // Yandex doesn't require a specific region, this is a dummy region.
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(settings.region))
            .credentials_provider(credentials)
            .endpoint_url(yandex_endpoint)
            .load()
            .await;

        // Construct a client for Yandex Object Storage using the custom endpoint.
        let client = Client::new(&config);

        YandexObjectStorage { client }
    }
}

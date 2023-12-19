//! src/service_providers/object_storage.rs

use std::time::Duration;

use anyhow::Context;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::primitives::SdkBody;
use aws_sdk_s3::Client;
use secrecy::ExposeSecret;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::config::ObjectStorageSettings;

// ───── Body ─────────────────────────────────────────────────────────────── //

/// Handle to work with Yandex object storage.
/// Client internally uses Arc, so clone is ok.
#[derive(Clone, Debug)]
pub struct YandexObjectStorage {
    client: Client,
    bucket_name: String,
}

impl YandexObjectStorage {
    /// Constructs a new instance of `YandexObjectStorage`.
    ///
    /// This method initializes a new `YandexObjectStorage` client using the provided
    /// `ObjectStorageSettings`. It directly sets credentials for use with Yandex and configures
    /// the client to use Yandex's Object Storage endpoint.
    ///
    /// # Arguments
    ///
    /// * `settings` - An instance of `ObjectStorageSettings` struct containing configuration
    ///   settings such as access keys, secret keys, and bucket name.
    ///
    /// # Returns
    ///
    /// Returns an instance of `YandexObjectStorage`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mustore::config::ObjectStorageSettings;
    /// use mustore::service_providers::object_storage::YandexObjectStorage;
    ///
    /// let storage_settings = ObjectStorageSettings::new("access_key", "secret_key", "bucket_name", "region");
    /// let yandex_storage = YandexObjectStorage::new(storage_settings).await;
    /// ```
    pub async fn new(settings: ObjectStorageSettings) -> Self {
        // Create Credentials object directly.
        let credentials = Credentials::new(
            settings.access_key_id.expose_secret(),
            settings.secret_access_key.expose_secret(),
            None, // Token, if you are using temporary credentials you can set it here
            None, // Expiry, this would be used for temporary credentials
            "provided-statically", // This is an arbitrary name for the credential provider
        );

        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(Region::new(settings.region))
            .credentials_provider(credentials)
            .endpoint_url(settings.endpoint_url)
            .load()
            .await;

        // Construct a client for Yandex Object Storage using the custom endpoint.
        let client = Client::new(&config);

        YandexObjectStorage {
            client,
            bucket_name: settings.bucket_name,
        }
    }

    /// Uploads a file to Yandex Object Storage.
    ///
    /// This method takes a file name and bytes, uploads them to the configured bucket, and
    /// returns the URI of the newly uploaded object.
    pub async fn put(
        &self,
        key: &str,
        bytes: Vec<u8>,
    ) -> Result<(), anyhow::Error> {
        let _put_response = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(
                ByteStream::try_from(SdkBody::from(bytes))
                    .context("Failed to create ByteStream from bytes")?,
            )
            .send()
            .await
            .context("Failed to upload file to the object storage")?;

        Ok(())
    }

    /// Generates a pre-signed URL for accessing an object stored in Yandex Object Storage.
    ///
    /// This method creates a pre-signed URL which clients can use to directly access an object in
    /// the bucket for a limited duration, without needing further authentication.
    pub async fn generate_presigned_url(
        &self,
        key: &str,
        expiration: Duration,
    ) -> Result<String, anyhow::Error> {
        // Construct a presigning config with the desired expiration time for the link.
        let presigning_config = PresigningConfig::builder()
            .expires_in(expiration)
            .build()
            .context("Failed to build presigning config")?;

        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// Deletes an object from the bucket specified by the object's URI.
    pub async fn delete_object_by_key(
        &self,
        key: &str,
    ) -> Result<(), anyhow::Error> {
        // Make sure something was actually extracted to prevent erroneous deletions
        if key.is_empty() || key == "/" {
            return Err(anyhow::Error::msg(
                "Invalid object URI provided for deletion",
            ));
        }

        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .with_context(|| format!("Failed to delete object: {}", key))?;

        Ok(())
    }
}

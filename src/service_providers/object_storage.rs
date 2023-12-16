//! src/service_providers/object_storage.rs

use std::time::Duration;

use anyhow::Context;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{
    config::Credentials,
    presigning::PresigningConfig,
    primitives::{ByteStream, SdkBody},
    Client,
};
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
    /// let storage_settings = ObjectStorageSettings::new("access_key", "secret_key", "bucket_name", "region");
    /// let yandex_storage = YandexObjectStorage::new(storage_settings).await;
    /// ```
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

        let config = aws_config::defaults(BehaviorVersion::latest())
            .credentials_provider(credentials)
            .endpoint_url(yandex_endpoint)
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
    ///
    /// # Arguments
    ///
    /// * `file_name` - A reference to a string slice that holds the name of the file to upload.
    /// * `bytes` - A vector of bytes that constitute the file content to upload.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is either an `Ok` variant with the object URI as a `String`
    /// if the operation is successful, or an `Err` variant with an `anyhow::Error` if the
    /// upload fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the byte stream cannot be created from the bytes
    /// provided or if the S3 client fails to upload the file to object storage.
    pub async fn put(
        &self,
        file_name: &str,
        bytes: Vec<u8>,
    ) -> Result<String, anyhow::Error> {
        let _put_response = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .body(
                ByteStream::try_from(SdkBody::from(bytes))
                    .context("Failed to create ByteStream from bytes")?,
            )
            .send()
            .await
            .context("Failed to upload file to the object storage")?;

        let object_uri = format!(
            "https://{}.storage.yandexcloud.net/{}",
            &self.bucket_name, file_name
        );
        Ok(object_uri)
    }

    /// Generates a pre-signed URL for accessing an object stored in Yandex Object Storage.
    ///
    /// This method creates a pre-signed URL which clients can use to directly access an object in
    /// the bucket for a limited duration, without needing further authentication.
    ///
    /// # Arguments
    ///
    /// * `key` - A reference to a string slice that holds the object's key within the storage bucket.
    /// * `expiration` - A `Duration` specifying how long until the pre-signed URL expires.
    ///
    /// # Returns
    ///
    /// Returns a `Result` which is either an `Ok` variant with the pre-signed URL as a `String`
    /// if the operation succeeds, or an `Err` variant with an `anyhow::Error` if the operation
    /// fails to generate the URL.
    ///
    /// # Errors
    ///
    /// This function will return an error if the pre-signing configuration cannot be built, or if
    /// the SDK encounters a problem creating the pre-signed URL.
    ///
    /// # Examples
    ///
    /// ```
    /// let object_key = "path/to/my/file.txt";
    /// let expiration = Duration::from_secs(3600); // 1 hour
    /// let presigned_url = yandex_storage.generate_presigned_url(object_key, expiration).await?;
    /// println!("Use this URL to access the object for the next hour: {}", presigned_url);
    /// ```
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
    ///
    /// # Arguments
    ///
    /// * `object_uri` - The URI of the object to delete.
    ///
    /// # Returns
    ///
    /// A result that, when successful, returns (), or Err(anyhow::Error) in case of failure.
    pub async fn delete_object_by_uri(
        &self,
        object_uri: &str,
    ) -> Result<(), anyhow::Error> {
        // Extract the object key from the URI.
        // Assuming the object_uri is of the format "https://bucketname.storage.yandexcloud.net/filename"
        let object_key = object_uri
            .trim_start_matches(&format!(
                "https://{}.storage.yandexcloud.net/",
                &self.bucket_name
            ))
            .to_string();

        // Make sure something was actually extracted to prevent erroneous deletions
        if object_key.is_empty() || object_key == "/" {
            return Err(anyhow::Error::msg(
                "Invalid object URI provided for deletion",
            ));
        }

        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(&object_key)
            .send()
            .await
            .with_context(|| {
                format!("Failed to delete object: {}", object_key)
            })?;

        Ok(())
    }
}

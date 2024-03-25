//! src/service_providers/object_storage.rs

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use aws_config::BehaviorVersion;
use aws_config::Region;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::operation::head_object::HeadObjectOutput;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::primitives::SdkBody;
use aws_sdk_s3::Client;
use secrecy::ExposeSecret;

use crate::config::ObjectStorageSettings;
use crate::domain::object_key::ObjectKey;
use crate::impl_debug;

use self::presigned_post_form::PresignedPostData;

pub mod presigned_post_form;

#[derive(thiserror::Error)]
pub enum ObjectStorageError {
    #[error(transparent)]
    SdkError(#[from] anyhow::Error),
    #[error(transparent)]
    PresignedPostFormError(#[from] presigned_post_form::Error),
    #[error("Key is wrong: {0}")]
    BadObjectKeyError(String),
}

impl_debug!(ObjectStorageError);

/// Handle to work with object storage.
/// Client internally uses Arc, so clone is ok.
#[derive(Clone, Debug)]
pub struct ObjectStorage {
    client: Client,
    settings: Arc<ObjectStorageSettings>,
}

impl ObjectStorage {
    pub async fn new(settings: ObjectStorageSettings) -> Self {
        // I don't know why without this it doesn't work
        let c = reqwest::Client::new();
        let _response = c.get(&settings.endpoint_url).send().await;

        // Create Credentials object directly.
        let credentials = Credentials::new(
            settings.access_key_id.expose_secret(),
            settings.secret_access_key.expose_secret(),
            None, // Token, if you are using temporary credentials you can set it here
            None, // Expiry, this would be used for temporary credentials
            "provided-statically", // This is an arbitrary name for the credential provider
        );

        let config = aws_sdk_s3::Config::builder()
            .region(Region::new(settings.region.clone()))
            .behavior_version(BehaviorVersion::latest())
            .credentials_provider(credentials)
            .endpoint_url(settings.endpoint_url.clone())
            .force_path_style(true)
            .build();
        let client = Client::from_conf(config);

        match client
            .head_bucket()
            .bucket(&settings.bucket_name)
            .send()
            .await
        {
            Ok(_) => {
                println!("Bucket '{}' already exists!", settings.bucket_name)
            }
            Err(e) => {
                println!("Can't get bucket: {:?}", e.into_source());
                println!("Try to create bucket..");
                match client
                    .create_bucket()
                    .bucket(&settings.bucket_name)
                    .send()
                    .await
                {
                    Ok(_) => println!(
                        "Bucket '{}' created successfully.",
                        settings.bucket_name
                    ),
                    Err(err) => panic!("Failed to create bucket: {}", err),
                }
            }
        }

        ObjectStorage {
            client,
            settings: Arc::new(settings),
        }
    }

    /// Uploads a file to Object Storage.
    ///
    /// This method takes a file name and bytes, uploads them to the configured bucket, and
    /// returns the URI of the newly uploaded object.
    pub async fn put<'a>(
        &self,
        key: &ObjectKey,
        bytes: Vec<u8>,
        mediatype: mediatype::MediaType<'a>,
    ) -> Result<(), ObjectStorageError> {
        let _put_response = self
            .client
            .put_object()
            .bucket(&self.settings.bucket_name)
            .key(key.as_ref())
            .content_encoding(mediatype.to_string())
            .body(
                ByteStream::try_from(SdkBody::from(bytes))
                    .context("Failed to create ByteStream from bytes")?,
            )
            .send()
            .await
            .context("Failed to upload file to the object storage")?;

        Ok(())
    }

    /// Moves a file to `received` folder from 'upload', returns a new key.
    pub async fn receive(
        &self,
        key: &ObjectKey,
    ) -> Result<ObjectKey, ObjectStorageError> {
        if key.directory() != ("upload") {
            return Err(ObjectStorageError::BadObjectKeyError(format!(
                "Key starts not with 'upload/', cant receive it: {}",
                key
            )));
        }
        let new_key = key.clone().moved("received");
        let _copy_response = self
            .client
            .copy_object()
            .bucket(&self.settings.bucket_name)
            .copy_source(format!("{}/{key}", &self.settings.bucket_name))
            .key(new_key.as_ref())
            .send()
            .await
            .context("Failed to copy object to the 'received/' directory")?;

        let _delete_response = self
            .client
            .delete_object()
            .bucket(&self.settings.bucket_name)
            .key(key.as_ref())
            .send()
            .await
            .context("Failed to delete old object from 'upload/' directory")?;

        Ok(new_key)
    }

    /// Retrieves object meta info and returns it.
    pub async fn get_object_meta(
        &self,
        key: &ObjectKey,
    ) -> Result<HeadObjectOutput, ObjectStorageError> {
        let head_response = self
            .client
            .head_object()
            .bucket(&self.settings.bucket_name)
            .key(key.as_ref())
            .send()
            .await
            .context("Failed to retrieve object meta by key")?;
        Ok(head_response)
    }

    /// Generates a pre-signed URL for accessing an object stored in Yandex Object Storage.
    ///
    /// This method creates a pre-signed URL which clients can use to directly access an object in
    /// the bucket for a limited duration, without needing further authentication.
    pub async fn generate_presigned_url(
        &self,
        key: &ObjectKey,
        expiration: Duration,
    ) -> Result<String, ObjectStorageError> {
        // Construct a presigning config with the desired expiration time for the link.
        let presigning_config = PresigningConfig::builder()
            .expires_in(expiration)
            .build()
            .context("Failed to build presigning config")?;

        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.settings.bucket_name)
            .key(key.as_ref())
            .presigned(presigning_config)
            .await
            .context("Failed to generate presigned url")?;

        Ok(presigned_request.uri().to_string())
    }

    pub fn generate_presigned_post_form(
        &self,
        object_key: &ObjectKey,
        mime: mediatype::MediaTypeBuf,
        max: u64,
    ) -> Result<PresignedPostData, ObjectStorageError> {
        let form = PresignedPostData::builder(
            &self.settings.secret_access_key.expose_secret(),
            &self.settings.access_key_id.expose_secret(),
            &self.settings.endpoint_url,
            &self.settings.region,
            &self.settings.bucket_name,
            object_key.as_ref(),
        )
        .with_content_length_range(0, max)
        .with_mime(mime.to_ref());

        Ok(form.build()?)
    }

    /// Deletes an object from the bucket specified by the object's URI.
    pub async fn delete_object_by_key(
        &self,
        key: &ObjectKey,
    ) -> Result<(), ObjectStorageError> {
        self.client
            .delete_object()
            .bucket(&self.settings.bucket_name)
            .key(key.as_ref())
            .send()
            .await
            .with_context(|| format!("Failed to delete object: {}", key))?;

        Ok(())
    }
}

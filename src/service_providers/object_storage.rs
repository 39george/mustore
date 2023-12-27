//! src/service_providers/object_storage.rs

use std::sync::Arc;
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

use self::presigned_post::MimeType;
use self::presigned_post::PresignedPostData;
use self::presigned_post::PresignedPostForm;

// ───── Body ─────────────────────────────────────────────────────────────── //

/// Handle to work with object storage.
/// Client internally uses Arc, so clone is ok.
#[derive(Clone, Debug)]
pub struct ObjectStorage {
    client: Client,
    settings: Arc<ObjectStorageSettings>,
}

impl ObjectStorage {
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
            .region(Region::new(settings.region.clone()))
            .credentials_provider(credentials)
            .endpoint_url(settings.endpoint_url.clone())
            .load()
            .await;

        // Construct a client for Yandex Object Storage using the custom endpoint.
        let client = Client::new(&config);

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
                println!(
                    "Bucket does not exist: {:?}, try to create it",
                    e.into_source()
                );
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
            .bucket(&self.settings.bucket_name)
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
            .bucket(&self.settings.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    pub fn generate_presigned_post_form(
        &self,
        object_key: &str,
        mime_type: MimeType,
    ) -> Result<PresignedPostForm, anyhow::Error> {
        let form = PresignedPostData::new(
            &self.settings.secret_access_key.expose_secret(),
            &self.settings.access_key_id.expose_secret(),
            &self.settings.endpoint_url,
            &self.settings.bucket_name,
            &self.settings.region,
            object_key,
            "filename",
            mime_type,
            "s3",
        );

        form.generate()
            .context("Failed to generate presigned post form")
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
            .bucket(&self.settings.bucket_name)
            .key(key)
            .send()
            .await
            .with_context(|| format!("Failed to delete object: {}", key))?;

        Ok(())
    }
}

mod presigned_post {
    use std::collections::HashMap;

    use base64::Engine;
    use hmac::digest::InvalidLength;
    use hmac::{Hmac, Mac};
    use serde::Serialize;
    use sha2::Sha256;
    use time::macros::format_description;
    use time::OffsetDateTime;

    use crate::error_chain_fmt;

    type HmacSha256 = Hmac<Sha256>;

    #[derive(thiserror::Error)]
    pub enum Error {
        #[error("Date parsing error: {0}")]
        DateParsingError(#[from] time::error::Format),
        #[error("HMAC signing error: {0}")]
        HmacError(#[from] InvalidLength),
        #[error("JSON serialization error: {0}")]
        JsonSerError(#[from] serde_json::Error),
    }

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            error_chain_fmt(self, f)
        }
    }

    #[derive(Serialize, Debug)]
    pub struct PresignedPostForm {
        pub url: String,
        pub fields: HashMap<String, String>,
    }

    pub enum MimeType {
        Mp3Audio,
        WaveAudio,
        Mp4Video,
        ZipMultitrack,
        JpegImage,
        PngImage,
    }

    impl MimeType {
        fn as_str(&self) -> &'static str {
            match self {
                MimeType::Mp3Audio => "audio/mpeg",
                MimeType::WaveAudio => "audio/wav",
                MimeType::Mp4Video => "video/mp4",
                MimeType::ZipMultitrack => "application/zip",
                MimeType::JpegImage => "image/jpeg",
                MimeType::PngImage => "image/png",
            }
        }

        fn min(&self) -> u32 {
            0
        }

        fn max(&self) -> u64 {
            match self {
                MimeType::Mp3Audio => 15_728_640,
                MimeType::ZipMultitrack => 5_242_880_000,
                MimeType::Mp4Video | MimeType::WaveAudio => 52_428_800,
                MimeType::JpegImage | MimeType::PngImage => 5_242_880,
            }
        }

        fn expiration(&self) -> (time::Duration, time::Duration) {
            use time::Duration;

            let (expiration, response_expiration) = match self {
                MimeType::Mp3Audio => {
                    (Duration::minutes(10), Duration::minutes(10))
                }
                MimeType::WaveAudio => {
                    (Duration::minutes(10), Duration::minutes(10))
                }
                MimeType::Mp4Video => {
                    (Duration::minutes(10), Duration::minutes(10))
                }
                MimeType::ZipMultitrack => {
                    (Duration::minutes(10), Duration::minutes(10))
                }
                MimeType::JpegImage => {
                    (Duration::minutes(10), Duration::minutes(10))
                }
                MimeType::PngImage => {
                    (Duration::minutes(10), Duration::minutes(10))
                }
            };
            (expiration, response_expiration)
        }
    }

    pub struct PresignedPostData<'a> {
        access_key: &'a str,
        access_key_id: &'a str,
        date: OffsetDateTime,
        object_storage_endpoint_url: &'a str,
        bucket: &'a str,
        region_name: &'a str,
        object_key: &'a str,
        content_disposition: &'a str,
        expiration: OffsetDateTime,
        response_expires: OffsetDateTime,
        mime_type: MimeType,
        service_name: &'a str,
    }

    impl<'a> PresignedPostData<'a> {
        pub fn new(
            access_key: &'a str,
            access_key_id: &'a str,
            object_storage_endpoint_url: &'a str,
            bucket: &'a str,
            region_name: &'a str,
            object_key: &'a str,
            content_disposition: &'a str,
            mime_type: MimeType,
            service_name: &'a str,
        ) -> Self {
            let now = OffsetDateTime::now_utc();
            let (expiration, response_expiration) = mime_type.expiration();

            let expiration = now + expiration;
            let response_expires = now + response_expiration;
            PresignedPostData {
                access_key,
                access_key_id,
                date: now,
                object_storage_endpoint_url,
                bucket,
                object_key,
                content_disposition,
                expiration,
                response_expires,
                mime_type,
                region_name,
                service_name,
            }
        }

        pub fn generate(&self) -> Result<PresignedPostForm, Error> {
            let format = format_description!("[year][month][day]");
            let date =
                self.date.format(&format).map_err(Error::DateParsingError)?;
            let policy = self.create_policy_document()?;
            let signing_key = get_signing_key(
                self.access_key,
                &date,
                self.region_name,
                self.service_name,
            )?;
            let policy_signature = get_policy_signature(&signing_key, &policy)?;

            let format = format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z"
            );

            let date =
                self.date.format(&format).map_err(Error::DateParsingError)?;

            let x_amz_credential = format!(
                "{}/{}/{}/{}/aws4_request",
                self.access_key_id, date, self.region_name, self.service_name
            );

            let mut map: HashMap<String, String> = HashMap::new();
            map.insert("X-Amz-Algorithm".into(), "AWS4-HMAC-SHA256".into());
            map.insert("X-Amz-Date".into(), date.into());
            map.insert("success_action_status".into(), "200".into());
            map.insert("X-Amz-Signature".into(), policy_signature.into());
            map.insert("key".into(), self.object_key.into());
            map.insert("acl".into(), "private".into());
            map.insert("policy".into(), policy.into());
            map.insert("X-Amz-Credential".into(), x_amz_credential.into());
            map.insert("Content-Type".into(), self.mime_type.as_str().into());
            map.insert(
                "Expires".into(),
                get_rfc_1123_date(self.response_expires).unwrap(),
            );
            map.insert(
                "Content-Disposition".into(),
                format!(
                    "attachment; filename=\"{}\"",
                    self.content_disposition
                ),
            );

            Ok(PresignedPostForm {
                url: format!(
                    "{}/{}",
                    self.object_storage_endpoint_url, self.bucket
                ),
                fields: map,
            })
        }

        fn create_policy_document(&self) -> Result<String, Error> {
            let policy = serde_json::json!({
                "expiration": self.expiration.format(&time::format_description::well_known::Rfc3339).map_err(Error::DateParsingError)?,
                "conditions": [
                    {"bucket": self.bucket},
                    {"key": self.object_key},
                    {"acl": "private"},
                    {"Content-Disposition": format!("attachment; filename=\"{}\"", self.content_disposition)},
                    {"Content-Type": self.mime_type.as_str() },
                    {"Expires": get_rfc_1123_date(self.response_expires).unwrap()},
                    ["content-length-range", self.mime_type.min(), self.mime_type.max()]
                ]
            });
            let policy =
                serde_json::to_string(&policy).map_err(Error::JsonSerError)?;

            // Base64 encode the policy document
            Ok(base64::engine::general_purpose::STANDARD.encode(policy))
        }
    }

    fn sign(key: &[u8], msg: &[u8]) -> Result<Vec<u8>, Error> {
        let mut mac =
            HmacSha256::new_from_slice(key).map_err(Error::HmacError)?;
        mac.update(msg);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    fn get_signing_key(
        access_key: &str,
        date: &str,
        region_name: &str,
        service_name: &str,
    ) -> Result<Vec<u8>, Error> {
        let k_date =
            sign(format!("AWS4{}", access_key).as_bytes(), date.as_bytes())?;
        let k_region = sign(&k_date, region_name.as_bytes())?;
        let k_service = sign(&k_region, service_name.as_bytes())?;
        sign(&k_service, b"aws4_request")
    }

    fn get_policy_signature(
        signing_key: &[u8],
        policy_document_base64: &str,
    ) -> Result<String, Error> {
        // Sign the policy document
        let signature = sign(signing_key, policy_document_base64.as_bytes())?;
        Ok(hex::encode(signature))
    }

    fn get_rfc_1123_date(time: OffsetDateTime) -> Result<String, time::Error> {
        // Define RFC 1123 format
        let format = format_description!(
            "[weekday repr:short case_sensitive:true], [day] [month repr:short case_sensitive:true] [year] [hour]:[minute]:[second] GMT"
        );
        // Format the `OffsetDateTime` to a string that matches the RFC 1123 format.
        let expires_header_value = time.format(&format)?;
        Ok(expires_header_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use presigned_post::{MimeType, PresignedPostData};
    use secrecy::{ExposeSecret, Secret};
    use std::collections::HashMap;

    #[test]
    fn test_generate_presigned_post_form() {
        let post_data = PresignedPostData::new(
            "wJalrXUtnFEMI/K7MDENG+bPxRfiCYEXAMPLEKEY",
            "AKIDEXAMPLE",
            "https://example.com",
            "my-bucket",
            "us-east-1",
            "test-image.png",
            "test-image.png",
            MimeType::PngImage,
            "s3",
        );

        let post_form = post_data.generate().unwrap();

        assert_eq!(post_form.url, "https://example.com/my-bucket");

        dbg!(post_form);

        // Expected fields based on the implementation
        // let expected_fields: HashMap<String, String> = [
        //     ("X-Amz-Algorithm", "AWS4-HMAC-SHA256"),
        //     ("X-Amz-Date" /* date placeholder */,),
        //     ("success_action_status", "200"),
        //     ("X-Amz-Signature" /* signature placeholder */,),
        //     ("X-Amz-Credential" /* credential placeholder */,),
        //     ("policy" /* policy placeholder */,),
        //     ("key", object_key),
        // ]
        // .iter()
        // .map(|(k, v)| (k.to_string(), v.to_string()))
        // .collect();

        // for (key, expected_value) in &expected_fields {
        //     match post_form.fields.get(key) {
        //         Some(actual_value) => assert_eq!(actual_value, expected_value),
        //         None => panic!("Field {} not found in form fields", key),
        //     }
        // }
    }
}

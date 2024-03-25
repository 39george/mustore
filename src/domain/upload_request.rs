use std::{fmt::Display, num::ParseIntError, str::FromStr};

use anyhow::Context;
use fred::clients::RedisClient;
use fred::interfaces::KeysInterface;
use fred::prelude::RedisResult;
use fred::{clients::RedisPool, types::Scanner};
use futures::TryStreamExt;
use time::OffsetDateTime;

use crate::impl_debug;

use super::object_key::{ObjectKey, ObjectKeyError};

#[derive(thiserror::Error)]
pub enum UploadRequestError {
    #[error(transparent)]
    ObjectStorageError(#[from] ParseIntError),
    #[error("Some part of type not found in given str, can't parse into UploadRequest: {0}")]
    PartNotFoundInStrError(String),
    #[error(transparent)]
    ObjectKeyError(#[from] ObjectKeyError),
}

impl_debug!(UploadRequestError);

#[derive(Debug)]
pub struct UploadRequest {
    user_id: i32,
    object_key: ObjectKey,
}

impl UploadRequest {
    pub fn new(user_id: i32, object_key: ObjectKey) -> Self {
        UploadRequest {
            user_id,
            object_key,
        }
    }
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
    pub fn object_key(&self) -> &ObjectKey {
        &self.object_key
    }
}

impl Display for UploadRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "upload_request:{}:{}",
            self.user_id, self.object_key
        ))
    }
}

impl FromStr for UploadRequest {
    type Err = UploadRequestError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements = s.splitn(3, ':').collect::<Vec<_>>();
        Ok(UploadRequest {
            user_id: elements
                .get(1)
                .ok_or(UploadRequestError::PartNotFoundInStrError(
                    s.to_owned(),
                ))?
                .parse()?,
            object_key: elements
                .get(2)
                .ok_or(UploadRequestError::PartNotFoundInStrError(
                    s.to_owned(),
                ))?
                .parse()?,
        })
    }
}

/// Check that slice with obj keys (upload requests) exists in redis.
#[tracing::instrument(
    name = "Check that upload requests for a given music product exists",
    skip_all
)]
pub async fn verify_upload_request_data_in_redis(
    con: &RedisPool,
    obj_keys: &[&ObjectKey],
    user_id: i32,
) -> RedisResult<()> {
    for obj_key in obj_keys.into_iter() {
        let upload_request = UploadRequest::new(user_id, (*obj_key).clone());
        let _created_at: String = con.get(&upload_request.to_string()).await?;
    }
    Ok(())
}

#[tracing::instrument(
    name = "Delete upload requests for a given music product",
    skip_all
)]
pub async fn delete_upload_request_data_from_redis(
    con: &RedisPool,
    obj_keys: &[&ObjectKey],
    user_id: i32,
) -> RedisResult<()> {
    for obj_key in obj_keys.into_iter() {
        let upload_request = UploadRequest::new(user_id, (*obj_key).clone());
        con.del(&upload_request.to_string()).await?;
    }
    Ok(())
}

#[tracing::instrument(
    name = "Store upload request data in the redis",
    skip_all
)]
pub async fn store_upload_request_data(
    con: &RedisPool,
    object_key: &ObjectKey,
    user_id: i32,
) -> RedisResult<()> {
    let created_at = OffsetDateTime::now_utc()
        .format(&crate::DEFAULT_TIME_FORMAT)
        .unwrap();
    let upload_request = UploadRequest::new(user_id, object_key.clone());
    con.set(&upload_request.to_string(), &created_at, None, None, false)
        .await?;
    Ok(())
}

#[tracing::instrument(name = "Check current user uploads redis", skip_all)]
pub async fn check_current_user_uploads(
    con: &RedisPool,
    user_id: i32,
) -> Result<(), crate::routes::ResponseError> {
    let pattern = format!("upload_request:{}*", user_id);
    let mut scan = con.next().scan(pattern, None, None);
    while let Ok(Some(mut page)) = scan.try_next().await {
        if let Some(keys) = page.take_results() {
            if keys.len() > 15 {
                tracing::error!(
                    "User {} already have 15 current uploads",
                    user_id
                );
                return Err(crate::routes::ResponseError::TooManyUploadsError);
            }
            if keys.len() > 5 {
                tracing::warn!(
                    "User {} already have 5 current uploads",
                    user_id
                );
            }
        }
        page.next()
            .context("Failed to move on to the next page of results from the SCAN operation")?;
    }
    Ok(())
}

/// Returns removed keys
pub async fn remove_outdated_uploads_from_redis<F>(
    con: &RedisClient,
    is_outdated: F,
) -> Vec<String>
where
    F: Fn(OffsetDateTime) -> bool,
{
    let pattern = "upload_request*";
    let mut scan = con.scan(pattern, None, None);
    let mut outdated_keys = Vec::new();
    while let Ok(Some(mut page)) = scan.try_next().await {
        if let Some(keys) = page.take_results() {
            for key in keys.into_iter() {
                let date: String = match con.get(&key).await {
                    Ok(d) => d,
                    Err(e) => {
                        tracing::error!(
                            "Failed to get timestamp from upload request: {e}"
                        );
                        continue;
                    }
                };
                let created_at =
                    OffsetDateTime::parse(&date, &crate::DEFAULT_TIME_FORMAT)
                        .unwrap();
                if is_outdated(created_at) {
                    match con.del::<u32, &fred::types::RedisKey>(&key).await {
                        Ok(count) => {
                            tracing::info!(
                                "{:?} is outdated, and deleted",
                                key
                            );
                            if count != 1 {
                                tracing::error!("Strange deletion result, should be 1, but got {count}");
                            }
                            if let Some(key) = key.into_string() {
                                outdated_keys.push(key);
                            } else {
                                tracing::error!(
                                    "Failed to convert previous RedisKey into string, look ↑"
                                );
                            }
                        }
                        Err(e) => {
                            tracing::error!(
                                "Failed to delete key from redis: {e}"
                            );
                            continue;
                        }
                    }
                }
            }
        }
        if let Err(e) = page.next() {
            tracing::error!("Failed to get next page: {e}");
            break;
        }
    }
    outdated_keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upload_request_parsed_correctly() {
        let upload_req =
            format!("upload_request:12:received/Carl:7FEE8A91-2620-4B9A-98E0-E7D13DA97A62:file.png").parse::<UploadRequest>();
        assert!(upload_req.is_ok());
    }
}

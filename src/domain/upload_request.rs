use std::{fmt::Display, num::ParseIntError, str::FromStr};

use crate::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum UploadRequestError {
    #[error(transparent)]
    ObjectStorageError(#[from] ParseIntError),
    #[error("Some part of type not found in given str, can't parse into UploadRequest")]
    PartNotFoundInStrError,
}

impl std::fmt::Debug for UploadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

pub struct UploadRequest {
    pub user_id: i32,
    pub object_key: String,
}

impl UploadRequest {
    pub fn new(user_id: i32, object_key: &str) -> Self {
        UploadRequest {
            user_id,
            object_key: object_key.to_string(),
        }
    }
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
    pub fn get_object_key(&self) -> &str {
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
        let elements = s.split(':').collect::<Vec<_>>();
        Ok(UploadRequest {
            user_id: elements
                .get(1)
                .ok_or(UploadRequestError::PartNotFoundInStrError)?
                .parse()?,
            object_key: elements
                .get(2)
                .ok_or(UploadRequestError::PartNotFoundInStrError)?
                .to_string(),
        })
    }
}

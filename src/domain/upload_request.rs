use std::{fmt::Display, num::ParseIntError, str::FromStr};

use crate::error_chain_fmt;

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

impl std::fmt::Debug for UploadRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testme() {
        let upload_req =
            format!("upload_request:12:received/Carl:7FEE8A91-2620-4B9A-98E0-E7D13DA97A62:file.png").parse::<UploadRequest>();
        assert!(upload_req.is_ok());
    }
}

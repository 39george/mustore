use serde::{de::Visitor, Serialize, Serializer};

use crate::impl_debug;
use std::{fmt::Display, str::FromStr};

use super::{
    contains_no_control_characters, forbidden_characters, OBJ_KEY_MAX_LEN,
    OBJ_KEY_MIN_LEN,
};

#[derive(thiserror::Error)]
pub enum ObjectKeyError {
    #[error(
        "Some part of type not found in given str, can't parse into ObjectKey"
    )]
    PartNotFoundInStrError,
    #[error(transparent)]
    InvalidUuidError(#[from] uuid::Error),
    #[error("Can't parse into ObjectKey: key has wrong length")]
    OutOfBoundsError,
    #[error("Can't parse into ObjectKey: key contains control chars")]
    ContainsControlCharsError,
    #[error("Can't parse into ObjectKey: key contains forbidden chars")]
    ContainsForbiddenCharsError,
}

impl_debug!(ObjectKeyError);

#[derive(Debug, Clone)]
pub struct ObjectKey {
    directory: String,
    owner: String,
    uuid: uuid::Uuid,
    filename: String,

    /// We store it parsed to avoid memory allocation
    /// when we need to get ObjectKey as str.
    parsed: String,
}

impl ObjectKey {
    pub fn new(
        directory: &str,
        username: &str,
        uuid: uuid::Uuid,
        filename: &str,
    ) -> Result<Self, ObjectKeyError> {
        let s = format!("{}{}{}{}", directory, username, uuid, filename);
        // We mean there also /,:,: symbols.
        if s.len() < OBJ_KEY_MIN_LEN + 3 || s.len() > OBJ_KEY_MAX_LEN - 3 {
            return Err(ObjectKeyError::OutOfBoundsError);
        }
        if contains_no_control_characters(&s, &()).is_err() {
            return Err(ObjectKeyError::ContainsControlCharsError);
        }
        if forbidden_characters(&s, &()).is_err() {
            return Err(ObjectKeyError::ContainsForbiddenCharsError);
        }
        Ok(ObjectKey {
            directory: directory.to_string(),
            owner: username.to_string(),
            uuid,
            filename: filename.to_string(),
            parsed: format!("{}/{}:{}:{}", directory, username, uuid, filename),
        })
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
    pub fn directory(&self) -> &str {
        &self.directory
    }
    pub fn moved(mut self, new_dir: &str) -> Self {
        self.directory = new_dir.to_string();
        self.parsed = self.to_string();
        self
    }
}

impl Display for ObjectKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}/{}:{}:{}",
            self.directory, self.owner, self.uuid, self.filename
        ))
    }
}

impl FromStr for ObjectKey {
    type Err = ObjectKeyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < OBJ_KEY_MIN_LEN || s.len() > OBJ_KEY_MAX_LEN {
            return Err(ObjectKeyError::OutOfBoundsError);
        }
        let parts = s.split('/').collect::<Vec<_>>();
        let directory =
            parts.get(0).ok_or(ObjectKeyError::PartNotFoundInStrError)?;

        let file_parts: Vec<&str> = parts[1].split(':').collect();
        let owner =
            file_parts.get(0).ok_or(ObjectKeyError::PartNotFoundInStrError)?;
        let uuid_str =
            file_parts.get(1).ok_or(ObjectKeyError::PartNotFoundInStrError)?;
        let uuid = uuid::Uuid::from_str(uuid_str)?;
        let filename =
            file_parts.get(2).ok_or(ObjectKeyError::PartNotFoundInStrError)?;
        Ok(ObjectKey {
            directory: directory.to_string(),
            owner: owner.to_string(),
            uuid,
            filename: filename.to_string(),
            parsed: s.to_string(),
        })
    }
}

impl AsRef<str> for ObjectKey {
    fn as_ref(&self) -> &str {
        &self.parsed
    }
}

impl Serialize for ObjectKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.parsed)
    }
}

struct ObjectKeyVisitor;

impl<'de> Visitor<'de> for ObjectKeyVisitor {
    type Value = ObjectKey;
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.parse::<ObjectKey>().map_err(|e| serde::de::Error::custom(e))
    }

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(formatter, "a string containing folder, owner, uuid, filename")
    }
}

impl<'de> serde::Deserialize<'de> for ObjectKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ObjectKeyVisitor)
    }
}

impl std::cmp::PartialEq for ObjectKey {
    fn eq(&self, other: &Self) -> bool {
        self.parsed.eq(&other.parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::ObjectKey;

    #[test]
    fn object_key_parsing_success() {
        let object_key = "received/Josianne Koepp:1efe0ab0-9a85-4f94-ae62-237aa8b31c8e:song.mp3".parse::<ObjectKey>();
        assert!(object_key.is_ok());
        let object_key_str = object_key.unwrap().to_string();
        assert!(object_key_str.parse::<ObjectKey>().is_ok());
    }

    #[test]
    fn object_key_serialization_success() {
        let object_key = serde_json::from_str::<Vec<ObjectKey>>("[\"received/Josianne Koepp:1efe0ab0-9a85-4f94-ae62-237aa8b31c8e:song.mp3\"]");
        assert!(object_key.is_ok());
        let object_key_str = serde_json::to_string(&object_key.unwrap());
        assert!(object_key_str.is_ok());
    }
}

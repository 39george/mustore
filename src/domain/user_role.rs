use serde::Deserialize;
use serde::Serialize;

use crate::cornucopia::types::public::Userrole;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum UserRole {
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "consumer")]
    Consumer,
    #[serde(rename = "fullstack")]
    Fullstack,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Creator => f.write_str("creator"),
            UserRole::Consumer => f.write_str("consumer"),
            UserRole::Fullstack => f.write_str("fullstack"),
        }
    }
}

impl TryFrom<&str> for UserRole {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "creator" => Ok(UserRole::Creator),
            "consumer" => Ok(UserRole::Consumer),
            "fullstack" => Ok(UserRole::Fullstack),
            other => {
                Err(format!("Can't create UserRole instance from {other}"))
            }
        }
    }
}

impl Into<Userrole> for UserRole {
    fn into(self) -> Userrole {
        match self {
            UserRole::Creator => Userrole::creator,
            UserRole::Consumer => Userrole::consumer,
            UserRole::Fullstack => Userrole::fullstack,
        }
    }
}

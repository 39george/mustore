use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum UserRole {
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "consumer")]
    Consumer,
}

impl UserRole {
    pub fn to_permission_string(&self) -> String {
        match self {
            UserRole::Creator => "group.creators".to_string(),
            UserRole::Consumer => "group.consumers".to_string(),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::Creator => f.write_str("creator"),
            UserRole::Consumer => f.write_str("consumer"),
        }
    }
}

impl TryFrom<&str> for UserRole {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "creator" => Ok(UserRole::Creator),
            "consumer" => Ok(UserRole::Consumer),
            other => {
                Err(format!("Can't create UserRole instance from {other}"))
            }
        }
    }
}

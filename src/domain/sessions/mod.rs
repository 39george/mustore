use fred::clients::RedisClient;
use fred::error::RedisError;
use fred::interfaces::KeysInterface;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{impl_debug, SESSION_EXPIRATION_SECS};

type UserId = i32;

#[derive(thiserror::Error)]
pub enum SessionError {
    #[error("Redis error")]
    RedisError(#[from] RedisError),
    #[error("Json serialization/deserialization error")]
    JsonError(#[from] serde_json::Error),
    #[error("Card token session not found")]
    SessionNotFound,
    #[error("Wrong session kind")]
    WrongKind,
}

impl_debug!(SessionError);

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum Status {
    #[default]
    Active,
    Failed,
    Cancelled,
    Successed,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Kind {
    AcceptingOffer { offer_id: i32 },
    CardTokenRegistration,
}

/// Session instance is stored in redis, and is available for 1 hour.
#[derive(Debug, Serialize, Deserialize)]
pub struct Session<'a> {
    pub kind: Kind,
    #[serde(skip)]
    redis_client: Option<&'a RedisClient>,
    #[serde(skip)]
    session_id: Option<Uuid>,
    user_id: UserId,
    status: Status,
}

impl<'a> Session<'a> {
    pub async fn new(
        client: &'a RedisClient,
        id: Uuid,
        user_id: i32,
        kind: Kind,
    ) -> Result<(), SessionError> {
        let s = Session {
            redis_client: Some(client),
            session_id: Some(id),
            user_id,
            status: Status::default(),
            kind,
        };
        s.set().await?;
        Ok(())
    }
    pub async fn from_redis_by_session_id(
        client: &'a RedisClient,
        session_id: Uuid,
    ) -> Result<Self, SessionError> {
        let key = Session::redis_key(session_id);
        Ok(client
            .get::<Option<String>, _>(key)
            .await?
            .map(|s| serde_json::from_str::<Self>(&s))
            .ok_or(SessionError::SessionNotFound)?
            .map(|mut s| {
                s.redis_client = Some(client);
                s.session_id = Some(session_id);
                s
            })?)
    }
    pub fn status(&self) -> Status {
        self.status.clone()
    }
    pub fn user_id(&self) -> i32 {
        self.user_id
    }
    pub async fn update_status(
        &mut self,
        new: Status,
    ) -> Result<(), SessionError> {
        self.status = new;
        self.set().await?;
        Ok(())
    }
    pub async fn remove(&self) -> Result<(), SessionError> {
        self.redis_client
            .unwrap()
            .del::<(), _>(Self::redis_key(self.session_id.unwrap()))
            .await?;
        Ok(())
    }
    async fn set(&self) -> Result<(), SessionError> {
        let val = serde_json::to_string(&self)?;
        let key = Self::redis_key(self.session_id.unwrap());
        self.redis_client
            .unwrap()
            .set(&key, val, None, None, false)
            .await?;
        self.redis_client
            .unwrap()
            .expire(key, SESSION_EXPIRATION_SECS)
            .await?;
        Ok(())
    }
    fn redis_key(session_id: Uuid) -> String {
        format!("card_token:{}", session_id)
    }
}

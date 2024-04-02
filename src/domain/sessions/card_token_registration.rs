use fred::clients::RedisClient;
use fred::error::RedisError;
use fred::interfaces::KeysInterface;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::impl_debug;

type UserId = i32;

#[derive(thiserror::Error)]
pub enum CardTokenSessionError {
    #[error("Redis error")]
    RedisError(#[from] RedisError),
    #[error("Json serialization/deserialization error")]
    JsonError(#[from] serde_json::Error),
    #[error("Card token session not found")]
    SessionNotFound,
}

impl_debug!(CardTokenSessionError);

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum Status {
    #[default]
    Active,
    Failed,
    Cancelled,
    Successed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardTokenSession<'a> {
    #[serde(skip)]
    redis_client: Option<&'a RedisClient>,
    session_id: Uuid,
    user_id: UserId,
    status: Status,
}

impl<'a> CardTokenSession<'a> {
    pub async fn new(
        client: &'a RedisClient,
        id: Uuid,
        user_id: i32,
    ) -> Result<(), CardTokenSessionError> {
        let s = CardTokenSession {
            redis_client: Some(client),
            session_id: id,
            user_id,
            status: Status::default(),
        };
        s.set().await?;
        Ok(())
    }
    pub async fn from_redis_by_session_id(
        client: &'a RedisClient,
        session_id: Uuid,
    ) -> Result<Self, CardTokenSessionError> {
        let key = CardTokenSession::redis_key(session_id);
        Ok(client
            .get::<Option<String>, _>(key)
            .await?
            .map(|s| serde_json::from_str::<Self>(&s))
            .ok_or(CardTokenSessionError::SessionNotFound)?
            .map(|mut s| {
                s.redis_client = Some(client);
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
    ) -> Result<(), CardTokenSessionError> {
        self.status = new;
        self.set().await?;
        Ok(())
    }
    pub async fn remove(&self) -> Result<(), CardTokenSessionError> {
        self.redis_client
            .unwrap()
            .del::<(), _>(Self::redis_key(self.session_id))
            .await?;
        Ok(())
    }
    async fn set(&self) -> Result<(), CardTokenSessionError> {
        let val = serde_json::to_string(&self)?;
        let key = Self::redis_key(self.session_id);
        self.redis_client
            .unwrap()
            .set(&key, val, None, None, false)
            .await?;
        self.redis_client.unwrap().expire(key, 60 * 60).await?; // 1 hour
        Ok(())
    }
    fn redis_key(session_id: Uuid) -> String {
        format!("card_token:{}", session_id)
    }
}

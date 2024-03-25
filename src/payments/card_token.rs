use fred::clients::RedisClient;
use fred::error::RedisError;
use fred::interfaces::KeysInterface;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::impl_debug;

use super::kopeck::Kopeck;

type UserId = u32;

#[derive(thiserror::Error)]
pub enum Error {
    #[error("Redis error")]
    RedisError(#[from] RedisError),
    #[error("Json serialization/deserialization error")]
    JsonError(#[from] serde_json::Error),
}

impl_debug!(Error);

#[derive(Debug, Serialize, Deserialize)]
pub struct CardToken {
    id: Uuid,
    user_id: UserId,
}

impl CardToken {
    pub fn new(id: Uuid, user_id: u32) -> Self {
        CardToken { id, user_id }
    }
    fn redis_body(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self)
    }
    fn redis_key(id: Uuid) -> String {
        format!("card_token:{}", id)
    }
}

// pub async fn store(client: &RedisClient, p: &Payment) -> Result<(), Error> {
//     let key = Payment::redis_key(p.id);
//     let val = p.redis_body()?;
//     let () = client.set(key, val, None, None, false).await?;
//     Ok(())
// }

// pub async fn fetch_by_id(
//     client: &RedisClient,
//     id: Uuid,
// ) -> Result<Option<Payment>, Error> {
//     let key = Payment::redis_key(id);
//     let json: Option<String> = client.get(key).await?;
//     if let Some(s) = json {
//         Ok(serde_json::from_str(&s)?)
//     } else {
//         Ok(None)
//     }
// }

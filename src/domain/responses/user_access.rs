use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

use crate::{
    cornucopia::queries::user_access::ListConversationById, error_chain_fmt,
    service_providers::object_storage::ObjectStorage,
};

trait UnpackOption {
    type Output;
    type Error;
    fn unpack(
        self,
        message: impl std::fmt::Display,
    ) -> Result<Self::Output, Self::Error>;
}

impl<T> UnpackOption for Option<T> {
    type Output = T;
    type Error = ConversationDataError;
    fn unpack(
        self,
        message: impl std::fmt::Display,
    ) -> Result<Self::Output, Self::Error> {
        match self {
            Some(d) => Ok(d),
            None => {
                tracing::error!("Application design error: {message}");
                return Err(ConversationDataError::NoRelatedDataError);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interlocutor {
    pub username: String,
    pub id: i32,
    pub avatar_url: String,
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug)]
pub struct ServiceData {
    pub service_name: String,
    pub service_cover_url: String,
    pub service_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i32,
    pub interlocutor_id: i32,
    pub text: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub service: Option<ServiceData>,
    pub reply_message_id: Option<i32>,
    pub attachments: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Offer {
    pub text: String,
    pub offer_id: i32,
    pub interlocutor_id: i32,
    pub service: ServiceData,
    pub price: Decimal,
    pub delivery_date: OffsetDateTime,
    pub free_revisions: i32,
    pub revision_price: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "entry_type", rename_all = "camelCase")]
pub enum Entry {
    Message(Message),
    Offer(Offer),
}

impl Entry {
    pub fn message(self) -> Message {
        match self {
            Entry::Message(m) => m,
            Entry::Offer(_) => panic!("Trying unwrap offer from message"),
        }
    }
    pub fn offer(self) -> Offer {
        match self {
            Entry::Message(_) => panic!("Trying unwrap message from offer"),
            Entry::Offer(o) => o,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConversationDataResponse {
    pub self_user_id: i32,
    pub interlocutors: HashMap<i32, Interlocutor>,
    pub entries: Vec<Entry>,
}

#[derive(thiserror::Error)]
pub enum ConversationDataError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("No related data presented")]
    NoRelatedDataError,
}

impl std::fmt::Debug for ConversationDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ConversationDataResponse {
    #[tracing::instrument(name = "Create conversation list", skip_all)]
    pub async fn new(
        conversation_data: Vec<ListConversationById>,
        object_storage: &ObjectStorage,
        self_user_id: i32,
    ) -> Result<Self, ConversationDataError> {
        let mut interlocutors: HashMap<i32, Interlocutor> = HashMap::new();
        let mut entries: Vec<Entry> = Vec::new();

        let expiration = std::time::Duration::from_secs(30 * 60);

        // Asynchronously collect presigned URLs
        let mut presigned_urls = HashMap::new();
        let mut message_attachments = HashMap::new();
        for data in &conversation_data {
            let presigned_avatar_key = object_storage
                .generate_presigned_url(
                    &data.participant_avatar_key,
                    expiration,
                )
                .await?;
            presigned_urls.insert(
                data.participant_avatar_key.clone(),
                presigned_avatar_key,
            );
            if let Some(ref cover_key) = data.service_cover_key {
                let url = object_storage
                    .generate_presigned_url(cover_key, expiration)
                    .await?;
                presigned_urls.insert(cover_key.clone(), url);
            }
            if let Some(message_id) = data.message_id {
                if let Some(ref data_attachments) = data.message_attachments {
                    let mut attachments_unpacked = Vec::new();
                    for key in data_attachments {
                        let url = object_storage
                            .generate_presigned_url(&key, expiration)
                            .await?;
                        attachments_unpacked.push(url);
                    }
                    message_attachments
                        .insert(message_id, attachments_unpacked);
                }
            }
        }

        for data in conversation_data.into_iter() {
            let avatar_url =
                presigned_urls.remove(&data.participant_avatar_key).unpack(
                    "failed to get avatar url from hash for interlocutor",
                )?;

            interlocutors
                .entry(data.participant_user_id)
                .or_insert_with(|| Interlocutor {
                    username: data.participant_username,
                    id: data.participant_user_id,
                    avatar_url,
                });

            let service = data
                .service_id
                .map(|service_id| -> Result<ServiceData, ConversationDataError> {
                    Ok(ServiceData {
                        service_name: data.service_name.unpack(
                            "service id is represented, but name is not",
                        )?,
                        service_cover_url: presigned_urls
                            .remove(
                                &data.service_cover_key
                                     .unpack("service id is represented, but cover key is not")?,
                            )
                            .unpack("failed to get service cover from hash")?,
                        service_id,
                    })
                })
                .transpose()?;

            if let Some(message_id) = data.message_id {
                entries.push(Entry::Message(Message {
                    message_id,
                    interlocutor_id: data.participant_user_id,
                    text: data
                        .message_text
                        .unpack("message id is represented, but text is not")?,
                    created_at: data.message_created_at.unpack(
                        "message id is represented, but created_at is not",
                    )?,
                    updated_at: data.message_updated_at.unpack(
                        "message id is represented, but updated_at is not",
                    )?,
                    service,
                    reply_message_id: data.reply_message_id,
                    attachments: message_attachments.remove(&message_id),
                }));
            } else if let Some(offer_id) = data.offer_id {
                entries.push(Entry::Offer(Offer {
                    text: data
                        .offer_text
                        .unpack("offer id is represented, but text is not")?,
                    offer_id,
                    interlocutor_id: data.participant_user_id,
                    service: service.unpack(
                        "offer id is represented, but service is not",
                    )?,
                    price: data
                        .offer_price
                        .unpack("offer id is represented, but price is not")?,
                    delivery_date: data.offer_delivery_date.unpack(
                        "offer id is represented, but delivery date is not",
                    )?,
                    free_revisions: data.offer_free_revisions.unpack(
                        "offer id is represented, but free revisions are not",
                    )?,
                    revision_price: data.offer_revision_price.unpack(
                        "offer id is represented, but revision price is not",
                    )?,
                }));
            }
        }

        Ok(ConversationDataResponse {
            self_user_id,
            interlocutors,
            entries,
        })
    }
}

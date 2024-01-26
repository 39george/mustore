use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use time::OffsetDateTime;
use utoipa::{ToResponse, ToSchema};

use crate::{
    cornucopia::queries::user_access::ListConversationById,
    domain::object_key::{ObjectKey, ObjectKeyError},
    error_chain_fmt,
    service_providers::object_storage::{ObjectStorage, ObjectStorageError},
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

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct DialogId {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, std::hash::Hash)]
pub struct Interlocutor {
    #[schema(example = "someuser123")]
    pub username: String,
    pub id: i32,
    #[schema(example = "https://storage.com/someimage.png")]
    pub avatar_url: String,
}

impl std::cmp::PartialEq for Interlocutor {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl std::cmp::Eq for Interlocutor {}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Attachment {
    #[schema(example = "image.png")]
    filename: String,
    key: ObjectKey,
}

impl Attachment {
    fn from_str(s: &str) -> Result<Attachment, ObjectKeyError> {
        let key: ObjectKey = s.parse()?;
        let filename = key.filename().to_string();
        Ok(Attachment { filename, key })
    }
}

impl std::cmp::PartialEq for Attachment {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

#[derive(Eq, PartialEq, Serialize, Deserialize, Debug, ToSchema)]
pub struct ServiceData {
    #[schema(example = "Mixing")]
    pub service_name: String,
    #[schema(example = "https://objectstorage.com/cover.png")]
    pub service_cover_url: String,
    pub service_id: i32,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Message {
    #[schema(example = 123)]
    pub message_id: i32,
    #[schema(example = 77)]
    pub interlocutor_id: i32,
    #[schema(example = "Hello, how are you?")]
    pub text: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    #[schema(inline = true)]
    pub service: Option<ServiceData>,
    #[schema(example = 727)]
    pub reply_message_id: Option<i32>,
    /// These are just file keys and filenames.
    /// To get url, call the special endpoint using given key.
    pub attachments: Option<Vec<Attachment>>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Offer {
    #[schema(example = "I can mix that with these conditions")]
    pub text: String,
    pub offer_id: i32,
    pub interlocutor_id: i32,
    #[schema(inline = true)]
    pub service: ServiceData,
    #[schema(
        value_type = f32,
        example = 18.50
    )]
    pub price: Decimal,
    #[serde(with = "time::serde::iso8601")]
    pub delivery_date: OffsetDateTime,
    pub free_revisions: i32,
    #[schema(
        value_type = f32,
        example = 2.0
    )]
    pub revision_price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(tag = "entry_type", rename_all = "camelCase")]
pub enum Entry {
    Message(#[schema(inline = true)] Message),
    Offer(#[schema(inline = true)] Offer),
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

#[derive(Serialize, Deserialize, Debug, ToResponse)]
pub struct ConversationDataResponse {
    pub self_user_id: i32,
    #[response(inline = true)]
    pub interlocutors: HashSet<Interlocutor>,
    #[response(inline = true)]
    pub entries: Vec<Entry>,
}

#[derive(thiserror::Error)]
pub enum ConversationDataError {
    #[error(transparent)]
    ObjectStorageError(#[from] ObjectStorageError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("No related data presented")]
    NoRelatedDataError,
    #[error(transparent)]
    ObjectKeyError(#[from] ObjectKeyError),
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
        let mut interlocutors: HashSet<Interlocutor> = HashSet::new();
        let mut entries: Vec<Entry> = Vec::new();

        let expiration = std::time::Duration::from_secs(30 * 60);

        // Asynchronously collect presigned URLs
        let mut presigned_urls = HashMap::new();
        for data in &conversation_data {
            let presigned_avatar_key = object_storage
                .generate_presigned_url(
                    &data.participant_avatar_key.parse()?,
                    expiration,
                )
                .await?;
            presigned_urls.insert(
                data.participant_avatar_key.clone(),
                presigned_avatar_key,
            );
            if let Some(ref cover_key) = data.service_cover_key {
                let url = object_storage
                    .generate_presigned_url(&cover_key.parse()?, expiration)
                    .await?;
                presigned_urls.insert(cover_key.clone(), url);
            }
        }

        for data in conversation_data.into_iter() {
            let avatar_url =
                presigned_urls.remove(&data.participant_avatar_key).unpack(
                    "failed to get avatar url from hash for interlocutor",
                )?;

            interlocutors.insert(Interlocutor {
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
                    attachments: data
                        .message_attachments
                        .map(|v| v.into_iter()
                            .map(|s| Attachment::from_str(&s))
                            .collect::<Result<Vec<Attachment>, ObjectKeyError>>()
                        ).transpose()?,
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

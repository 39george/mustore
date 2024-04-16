use garde::Validate;
use serde::Deserialize;
use serde::Serialize;
use utoipa::IntoParams;
use utoipa::ToSchema;

use crate::domain::object_key::ObjectKey;
use crate::domain::*;

#[derive(Deserialize, Debug, Validate, ToSchema, IntoParams)]
pub struct UploadFileRequest {
    #[garde(skip)]
    pub media_type: mediatype::MediaTypeBuf,
    #[garde(
        length(chars, min = MIN_FILENAME_LEN, max = MAX_FILENAME_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    #[param(min_length = 2, max_length = 50, pattern = r#"[^/()"<>\\{};:]*"#)]
    pub file_name: String,
}

/// We don't use `ToSchema` here, because we specify it manually above handler
/// in `params` section.
#[derive(Deserialize, Debug)]
pub struct GetConversationRequest {
    pub with_user: String,
}

#[derive(Deserialize, Debug, ToSchema, IntoParams)]
pub struct CreateConversationRequest {
    #[param(example = "jack123")]
    pub with_username: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
#[garde(allow_unvalidated)]
pub struct SendMessageRequest {
    /// Id of conversation
    pub conversation_id: i32,
    /// Text of a message
    #[garde(length(chars, min = MIN_MESSAGE_LEN, max = MAX_MESSAGE_LEN))]
    #[schema(min_length = 1, max_length = 2500)]
    pub text: String,
    /// Optional service id
    pub service_id: Option<i32>,
    /// Optional attachments
    #[schema(max_items = 10)]
    #[garde(length(min = 0, max = MAX_ATTACHMENTS_COUNT))]
    pub attachments: Vec<ObjectKey>,
    /// Optional reply message
    pub reply_message_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, IntoParams)]
pub struct ListConversationRequest {
    pub conversation_id: i32,
    pub offset: i64,
}

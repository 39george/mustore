use garde::Validate;
use serde::Deserialize;
use serde::Serialize;

use crate::domain::*;

#[derive(Deserialize, Debug, Validate)]
pub struct UploadFileRequest {
    #[garde(skip)]
    pub media_type: mediatype::MediaTypeBuf,
    #[garde(
        length(min = MIN_FILENAME_LEN, max = MAX_FILENAME_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    pub file_name: String,
}

#[derive(Deserialize, Debug)]
pub struct GetConversationRequest {
    pub with_user_id: i32,
}

#[derive(Deserialize, Debug)]
pub struct CreateConversationRequest {
    pub with_user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[garde(allow_unvalidated)]
pub struct SendMessageRequest {
    pub conversation_id: i32,
    #[garde(length(min = MIN_MESSAGE_LEN, max = MAX_MESSAGE_LEN))]
    pub text: String,
    pub service_id: Option<i32>,
    #[garde(length(min = 0, max = MAX_ATTACHMENTS_COUNT))]
    pub attachments: Vec<String>,
    pub reply_message_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListConversationRequest {
    pub conversation_id: i32,
    pub offset: i64,
}

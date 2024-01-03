use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct UploadFileRequest {
    pub media_type: mediatype::MediaTypeBuf,
    #[validate(
        length(min = 2, max = 50),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
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
pub struct SendMessageRequest {
    pub conversation_id: i32,
    #[validate(length(min = 1, max = 2500))]
    pub text: String,
    pub service_id: Option<i32>,
    pub attachments: Vec<String>,
    pub reply_message_id: Option<i32>,
}

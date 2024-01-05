use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;
use validator::Validate;

use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::domain::validate_slice_bounds_characters;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct SubmitSongRequest {
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub song_master_object_key: String,
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub song_master_tagged_object_key: Option<String>,
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub song_multitrack_object_key: String,
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub song_cover_object_key: String,
    #[validate(
        length(min = 2, max = 30),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub name: String,
    #[validate(
        length(min = 15, max = 400),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub description: Option<String>,
    #[validate(custom(
        function = "validate_slice_bounds_characters",
        arg = "(usize, usize)"
    ))]
    pub moods: Vec<String>,
    #[validate(
        length(min = 1, max = 50),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub primary_genre: String,
    #[validate(
        length(min = 1, max = 50),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub secondary_genre: Option<String>,
    #[validate(range(min = 40, max = 320))]
    pub tempo: i16,
    #[validate(range(min = 15, max = 600))]
    pub duration: i16,
    #[validate(
        length(min = 1, max = 1000),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub lyric: String,
    pub price: Decimal,
    pub sex: Sex,
    pub key: MusicKey,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateOfferRequest {
    pub conversation_id: i32,
    pub service_id: i32,
    #[validate(length(min = 1, max = 2500))]
    pub text: String,
    pub price: Decimal,
    pub delivery_date: OffsetDateTime,
    pub free_revisions: i32,
    pub revision_price: Decimal,
}

use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;
use validator::Validate;
use validator::ValidateArgs;
use validator::ValidationError;
use validator::ValidationErrors;

use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct MusicProduct {
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub master_object_key: String,
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub master_tagged_object_key: Option<String>,
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub multitrack_object_key: String,
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub cover_object_key: String,
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
    #[validate(custom(function = "validate_moods_slice_bounds_characters"))]
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
    pub price: Decimal,
    pub key: MusicKey,
}

pub fn validate_moods_slice_bounds_characters(
    moods: &[String],
) -> Result<(), ValidationError> {
    for mood in moods.iter() {
        crate::domain::forbidden_characters(mood)?;
        if mood.len() < 1 {
            return Err(ValidationError::new("Mood is too short"));
        } else if mood.len() > 50 {
            return Err(ValidationError::new("Mood is too long"));
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct SongMusicProduct {
    #[validate(
        length(min = 1, max = 1000),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub lyric: String,
    pub sex: Sex,
    #[validate]
    pub music_product: MusicProduct,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubmitMusicProductRequest {
    Beat(MusicProduct),
    Song(SongMusicProduct),
}

impl Validate for SubmitMusicProductRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            SubmitMusicProductRequest::Beat(m) => m.validate(),
            SubmitMusicProductRequest::Song(s) => s.validate(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct OtherProduct {
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
    #[validate(
        length(min = 10, max = 500),
        non_control_character,
        custom = "crate::domain::forbidden_characters"
    )]
    pub cover_object_key: String,
    #[validate(custom(function = "validate_moods_slice_bounds_characters"))]
    pub moods: Vec<String>,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SubmitOtherProductRequest {
    Lyric(OtherProduct, String),
    Cover(OtherProduct),
}

impl Validate for SubmitOtherProductRequest {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            SubmitOtherProductRequest::Lyric(p, _) => p.validate(),
            SubmitOtherProductRequest::Cover(p) => p.validate(),
        }
    }
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

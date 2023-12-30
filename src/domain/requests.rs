use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;
use validator::ValidationError;

use super::music_parameters::MusicKey;
use super::music_parameters::Sex;
use super::music_parameters::SortBy;
use super::validate_slice_bounds_characters;

#[derive(Deserialize, Debug, Validate)]
pub struct GetSongsListRequest {
    pub sex: Option<Sex>,
    #[validate(custom(
        function = "validate_tempo_bounds",
        arg = "(i16, i16)"
    ))]
    pub tempo: Option<Vec<i16>>,
    pub key: Option<Vec<MusicKey>>,
    pub genres: Option<Vec<String>>,
    pub vibes: Option<Vec<String>>,
    pub sort_by: SortBy,
    #[validate(range(min = 1, max = 50))]
    pub amount: i64,
    pub offset: i64,
}

fn validate_tempo_bounds(
    tempos: &[i16],
    (min, max): (i16, i16),
) -> Result<(), ValidationError> {
    if tempos.len() != 2 {
        Err(ValidationError::new("Tempo array should contain 2 values"))
    } else if tempos[0] < min
        || tempos[0] > max
        || tempos[1] < min
        || tempos[1] > max
    {
        Err(ValidationError::new("Tempo is out of bounds"))
    } else {
        Ok(())
    }
}

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
    pub tags: Vec<String>,
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

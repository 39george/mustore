use serde::Deserialize;
use validator::Validate;
use validator::ValidationError;

use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::domain::music_parameters::SortBy;

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

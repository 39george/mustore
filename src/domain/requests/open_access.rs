use garde::Validate;

use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::domain::music_parameters::SortBy;
use crate::domain::*;

use serde::Deserialize;

#[derive(Deserialize, Debug, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
#[garde(allow_unvalidated)]
pub struct GetSongsListRequest {
    /// Filter by sex
    pub sex: Option<Sex>,
    /// Filter by tempo range
    #[serde(default)]
    #[garde(custom(validate_tempo_bounds))]
    pub tempo: Vec<i16>,
    /// Filter by music key
    #[serde(default)]
    pub key: Vec<MusicKey>,
    /// Filter by genres
    #[serde(default)]
    #[garde(inner(
        length(min=MOOD_MIN_LEN, max=MOOD_MAX_LEN), 
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    ))]
    pub genres: Vec<String>,
    /// Filter by moods (vibes)
    #[serde(default)]
    #[garde(inner(
        length(min=MOOD_MIN_LEN, max=MOOD_MAX_LEN), 
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    ))]
    pub vibes: Vec<String>,
    /// Set sorting strategy
    pub sort_by: SortBy,
    #[garde(range(min = 1, max = 50))]
    /// Amount of songs
    pub amount: i64,
    /// Songs list offset
    pub offset: i64,
}

fn validate_tempo_bounds(tempos: &[i16], _: &()) -> garde::Result {
    if tempos.len() != 2 {
        Err(garde::Error::new("Tempo array should contain 2 values"))
    } else if tempos[0] < MIN_TEMPO
        || tempos[0] > MAX_TEMPO
        || tempos[1] < MIN_TEMPO
        || tempos[1] > MAX_TEMPO
    {
        Err(garde::Error::new("Tempo is out of bounds"))
    } else if tempos[0] > tempos[1] {
        Err(garde::Error::new("First tempo marker can't be less than second"))
    } else {
        Ok(())
    }
}

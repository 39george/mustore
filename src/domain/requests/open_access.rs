use garde::Validate;
use serde::Deserialize;

use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::domain::music_parameters::SortBy;
use crate::domain::*;

#[derive(Deserialize, Debug, Validate, utoipa::ToSchema, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
#[garde(allow_unvalidated)]
pub struct GetSongsListRequest {
    pub sex: Option<Sex>,
    #[garde(inner(custom(validate_tempo_bounds)))]
    #[param(value_type = Vec<i16>)]
    pub tempo: Option<Vec<i16>>,
    pub key: Option<Vec<MusicKey>>,
    #[garde(inner(inner(
        length(min=MOOD_MIN_LEN, max=MOOD_MAX_LEN), 
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )))]
    pub genres: Option<Vec<String>>,
    #[garde(inner(inner(
        length(min=MOOD_MIN_LEN, max=MOOD_MAX_LEN), 
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )))]
    pub vibes: Option<Vec<String>>,
    pub sort_by: SortBy,
    #[garde(range(min = 1, max = 50))]
    pub amount: i64,
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
        Err(garde::Error::new("First tempo markers can't be reversed"))
    } else {
        Ok(())
    }
}

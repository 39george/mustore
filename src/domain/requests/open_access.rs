use std::collections::HashMap;

use anyhow::Context;
use garde::Validate;
use serde::Serialize;
use utoipa::IntoParams;
use utoipa::ToSchema;

use crate::cornucopia::queries::open_access;
use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::domain::music_parameters::SortBy;
use crate::domain::*;

use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Stats {
    #[schema(example = "55")]
    beats: u32,
    #[schema(example = "12")]
    songs: u32,
    #[schema(example = "77")]
    lyrics: u32,
    #[schema(example = "71")]
    covers: u32,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SongsAmount {
    /// Amount of songs
    #[param(minimum = 1, maximum = 50)]
    #[garde(range(min = 1, max = 50))]
    pub amount: i64,
}

impl TryFrom<Vec<open_access::GetStats>> for Stats {
    type Error = anyhow::Error;
    fn try_from(
        value: Vec<open_access::GetStats>,
    ) -> Result<Self, Self::Error> {
        let hash_map: HashMap<String, u32> = value
            .into_iter()
            .map(|v| (v.table_name, v.count as u32))
            .collect();
        Ok(Stats {
            beats: *hash_map
                .get("beats")
                .context("Failed to get beats count")?,
            songs: *hash_map
                .get("songs")
                .context("Failed to get songs count")?,
            lyrics: *hash_map
                .get("lyrics")
                .context("Failed to get lyrics count")?,
            covers: *hash_map
                .get("covers")
                .context("Failed to get covers count")?,
        })
    }
}

#[derive(Deserialize, Debug, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
#[garde(allow_unvalidated)]
pub struct GetSongsListRequest {
    /// Filter by sex
    pub sex: Option<Sex>,
    /// Filter by tempo range, min is 40, max is 320, should be 2 values!
    #[serde(default)]
    #[param(minimum = 40, maximum = 320, max_items = 2, min_items = 2, example = json!([120, 150]))]
    #[garde(custom(validate_tempo_bounds))]
    pub tempo: Vec<i16>,
    /// Filter by music key
    #[serde(default)]
    pub key: Vec<MusicKey>,
    /// Filter by genres
    #[serde(default)]
    #[garde(inner(
        length(chars, min=MOOD_MIN_LEN, max=MOOD_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    ))]
    pub genres: Vec<String>,
    /// Filter by moods (vibes)
    #[serde(default)]
    #[garde(inner(
        length(chars, min=MOOD_MIN_LEN, max=MOOD_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    ))]
    pub vibes: Vec<String>,
    /// Set sorting strategy
    pub sort_by: SortBy,
    /// Amount of songs
    #[param(minimum = 1, maximum = 50, example = 30)]
    #[garde(range(min = 1, max = 50))]
    pub amount: i64,
    /// Songs list offset
    #[param(example = 0)]
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
        Err(garde::Error::new(
            "First tempo marker can't be less than second",
        ))
    } else {
        Ok(())
    }
}

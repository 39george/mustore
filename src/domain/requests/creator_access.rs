use garde::Validate;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;
use utoipa::ToSchema;

use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::domain::*;

use self::object_key::ObjectKey;

/// Lyric (text for song).
#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
#[garde(transparent)]
pub struct Lyric(
    #[garde(
        length(min = MIN_LYRIC_LEN, max = MAX_LYRIC_LEN),
        custom(contains_no_control_characters)
    )]
    /// Should contain no control characters
    #[schema(
        example = "Some lyrics",
        min_length = 1,
        max_length = 5000,
    )]
    String,
);

impl AsRef<str> for Lyric {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T> From<T> for Lyric
where T: std::fmt::Display
{
    fn from(value: T) -> Self {
        Lyric(value.to_string())
    }
}

/// Fields related with product.
#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct Product {
    /// Should contain no control characters
    #[garde(
        length(min = PRDCT_NAME_MIN_LEN, max = PRDCT_NAME_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    #[schema(min_length = 2, max_length = 30, pattern = r#"[^/()"<>\\{};:]*"#)]
    pub name: String,
    /// Should contain no control characters
    #[garde(
        length(min = PRDCT_DESC_MIN_LEN, max = PRDCT_DESC_MAX_LEN),
        inner(
            custom(forbidden_characters),
            custom(contains_no_control_characters)
        )
    )]
    #[schema(min_length = 15, max_length = 400, pattern = r#"[^/()"<>\\{};:]*"#)]
    pub description: Option<String>,
    #[garde(inner(
        length(min=MOOD_MIN_LEN, max=MOOD_MAX_LEN), 
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    ))]
    #[schema(pattern = r#"[^/()"<>\\{};:]*"#)]
    pub moods: Vec<String>,
    #[garde(skip)]
    pub cover_object_key: ObjectKey,
    #[garde(skip)]
    #[schema(
        value_type = f32,
        example = 18.50
    )]
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
#[garde(allow_unvalidated)]
pub struct MusicProduct {
    pub master_object_key: ObjectKey,
    pub master_tagged_object_key: Option<ObjectKey>,
    pub multitrack_object_key: ObjectKey,
    #[garde(
        length(min=GENRE_MIN_LEN, max=GENRE_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    #[schema(
        example = "pop",
        pattern = r#"[^/()"<>\\{};:]*"#
    )]
    pub primary_genre: String,
    #[garde(
        length(min=GENRE_MIN_LEN, max=GENRE_MAX_LEN),
        inner(
            custom(forbidden_characters),
            custom(contains_no_control_characters)
        )
    )]
    #[schema(pattern = r#"[^/()"<>\\{};:]*"#)]
    pub secondary_genre: Option<String>,
    #[garde(range(min = MIN_TEMPO, max = MAX_TEMPO))]
    #[schema(minimum = 40, maximum = 320)]
    pub tempo: i16,
    #[garde(range(min = MIN_AUDIO_DURATION_SEC, max = MAX_AUDIO_DURATION_SEC))]
    #[schema(minimum = 15, maximum = 600)]
    pub duration: i16,
    pub music_key: MusicKey,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub enum SubmitProductRequest {
    Beat {
        #[garde(dive)]
        product: Product,
        #[garde(dive)]
        music_product: MusicProduct,
    },
    Song {
        #[garde(dive)]
        product: Product,
        #[garde(dive)] 
        music_product: MusicProduct,
        #[garde(dive)]
        lyric: Lyric,
        #[garde(skip)]
        sex: Sex
    },
    Lyric {
        #[garde(dive)]
        product: Product,
        #[garde(dive)]
        lyric: Lyric,
        #[garde(skip)] 
        sex: Option<Sex>
    },
    Cover {
        #[garde(dive)]
        product: Product,
    }
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct Service {
    #[garde(
        length(min = PRDCT_NAME_MIN_LEN, max = PRDCT_NAME_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    #[schema(example = "Mixing")]
    pub name: String,
    #[garde(inner(
        length(min = PRDCT_DESC_MIN_LEN, max = PRDCT_DESC_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    ))]
    pub description: Option<String>,
    #[garde(skip)]
    pub cover_object_key: ObjectKey,
    #[garde(skip)]
    #[schema(
        value_type = f32,
        example = 18.50
    )]
    pub display_price: Decimal,
    #[garde(
        // Checked, length works as expected here
        length(min = 1, max = 3),
    )]
    #[schema(max_items = 3)]
    pub credits_object_keys: Option<Vec<ObjectKey>>,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct MusicService {
    #[garde(dive)]
    pub service: Service,
    #[garde(inner(inner(
        custom(contains_no_control_characters)
    )))]
    pub genres: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub enum SubmitServiceRequest {
    Mixing(#[garde(dive)] MusicService),
    SongWriting(#[garde(dive)] MusicService),
    BeatWriting(#[garde(dive)] MusicService),

    GhostWriting {
        #[garde(dive)]
        service: Service,
        // FIXME: check that it works correctly
        #[garde(inner(
            length(min = MIN_LYRIC_COUNT, max = MAX_LYRIC_COUNT),
            inner(
                length(min = MIN_LYRIC_LEN, max = MAX_LYRIC_LEN),
                custom(contains_no_control_characters)
            )
        ))]
        credits: Option<Vec<String>>
    },
    CoverDesign(#[garde(dive)] Service),
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[garde(allow_unvalidated)]
pub struct CreateOfferRequest {
    pub conversation_id: i32,
    pub service_id: i32,
    #[garde(length(min = MIN_MESSAGE_LEN, max = MAX_MESSAGE_LEN))]
    pub text: String,
    pub price: Decimal,
    pub delivery_date: OffsetDateTime,
    pub free_revisions: i32,
    pub revision_price: Decimal,
}

#[cfg(test)]
mod tests{
    use super::SubmitProductRequest;

    #[test]
    fn testme() {
        let s = r#"
            {
              "Beat": {
                "music_product": {
                  "duration": 0,
                  "master_object_key": "received/Lisa:21C960E7-5CA8-4974-98D7-6501DCCCAFD7:master.wav",
                  "master_tagged_object_key": "received/Lisa:21C960E7-5CA8-4974-98D7-6501DCCCAFD7:tagged.wav",
                  "multitrack_object_key": "received/Lisa:21C960E7-5CA8-4974-98D7-6501DCCCAFD7:multitrack.zip",
                  "music_key": "a_minor",
                  "primary_genre": "pop",
                  "secondary_genre": "string",
                  "tempo": 0
                },
                "product": {
                  "cover_object_key": "received/Lisa:21C960E7-5CA8-4974-98D7-6501DCCCAFD7:image.png",
                  "description": "string",
                  "moods": [
                    "string"
                  ],
                  "name": "string",
                  "price": 18.5
                }
              }
            }
        "#;
        let abc: SubmitProductRequest = serde_json::from_str(&s).unwrap();
        dbg!(abc);
    }
}

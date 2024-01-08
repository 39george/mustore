use garde::Validate;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::domain::*;

#[derive(Serialize, Deserialize, Debug, Validate)]
#[garde(transparent)]
pub struct ObjKey(
    #[garde(
        length(min = OBJ_KEY_MIN_LEN, max = OBJ_KEY_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    String,
);

impl AsRef<str> for ObjKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<T> From<T> for ObjKey 
where T: std::fmt::Display
{
    fn from(value: T) -> Self {
        ObjKey(value.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[garde(transparent)]
pub struct Lyric(
    #[garde(
        length(min = MIN_LYRIC_LEN, max = MAX_LYRIC_LEN),
        custom(contains_no_control_characters)
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

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Product {
    #[garde(
        length(min = PRDCT_NAME_MIN_LEN, max = PRDCT_NAME_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    pub name: String,
    #[garde(
        length(min = PRDCT_DESC_MIN_LEN, max = PRDCT_DESC_MAX_LEN),
        inner(
            custom(forbidden_characters),
            custom(contains_no_control_characters)
        )
    )]
    pub description: Option<String>,
    #[garde(inner(
        length(min=MOOD_MIN_LEN, max=MOOD_MAX_LEN), 
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    ))]
    pub moods: Vec<String>,
    #[garde(dive)]
    pub cover_object_key: ObjKey,
    #[garde(skip)]
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct MusicProduct {
    #[garde(dive)]
    pub master_object_key: ObjKey,
    #[garde(dive)]
    pub master_tagged_object_key: Option<ObjKey>,
    #[garde(dive)]
    pub multitrack_object_key: ObjKey,
    #[garde(
        length(min=GENRE_MIN_LEN, max=GENRE_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    pub primary_genre: String,
    #[garde(
        length(min=GENRE_MIN_LEN, max=GENRE_MAX_LEN),
        inner(
            custom(forbidden_characters),
            custom(contains_no_control_characters)
        )
    )]
    pub secondary_genre: Option<String>,
    #[garde(range(min = MIN_TEMPO, max = MAX_TEMPO))]
    pub tempo: i16,
    #[garde(range(min = MIN_AUDIO_DURATION_SEC, max = MAX_AUDIO_DURATION_SEC))]
    pub duration: i16,
    #[garde(skip)]
    pub key: MusicKey,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
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
        sex :Sex
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

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Service {
    #[garde(
        length(min = PRDCT_NAME_MIN_LEN, max = PRDCT_NAME_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )]
    pub name: String,
    #[garde(inner(
        length(min = PRDCT_DESC_MIN_LEN, max = PRDCT_DESC_MAX_LEN),
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    ))]
    pub description: Option<String>,
    #[garde(dive)]
    pub cover_object_key: ObjKey,
    #[garde(skip)]
    pub display_price: Decimal,
    // FIXME: check that lenght works correcly here
    #[garde(
        dive,
        length(min = 1, max = 3),
    )]
    pub credits_object_keys: Option<Vec<ObjKey>>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct MusicService {
    #[garde(dive)]
    pub service: Service,
    #[garde(inner(inner(
        custom(forbidden_characters),
        custom(contains_no_control_characters)
    )))]
    pub genres: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub enum SubmitServiceRequest {
    Mixing(#[garde(dive)] MusicService),
    SongWriting(#[garde(dive)] MusicService),
    BeatWriting(#[garde(dive)] MusicService),

    GhostWriting {
        #[garde(dive)]
        service: Service,
        // FIXME: check that it works correctly
        #[garde(inner(
            length(min = MIN_LYRIC_LEN, max = MAX_LYRIC_LEN),
            inner(custom(contains_no_control_characters))
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

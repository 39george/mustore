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

// mod inner {
//     use garde::Validate;
//     use rust_decimal::Decimal;
//     use serde::{Deserialize, Serialize};

//     const MIN: usize = 1;
//     const MAX: usize = 1000;

//     #[derive(Serialize, Deserialize, Debug, Validate)]
//     pub struct Service {
//         // #[validate(
//         //     length(min = 2, max = 30),
//         //     non_control_character,
//         //     custom = "crate::domain::forbidden_characters"
//         // )]
//         #[garde(length(min = MIN, max = MAX))]
//         pub name: String,
//         // #[validate(
//         //     length(min = 15, max = 400),
//         //     non_control_character,
//         //     custom = "crate::domain::forbidden_characters"
//         // )]
//         // pub description: Option<String>,
//         // #[validate(
//         //     length(min = 10, max = 500),
//         //     non_control_character,
//         //     custom = "crate::domain::forbidden_characters"
//         // )]
//         // pub cover_object_key: String,
//         // #[validate(custom = "validate_credits_object_keys")]
//         // pub credits_object_keys: Option<Vec<String>>,
//         // pub display_price: Decimal,
//     }

//     #[derive(Serialize, Deserialize, Debug)]
//     pub enum SubmitServiceRequest {
//         /// Vec with genres list for all musical services
//         Mixing(Service, Vec<String>),
//         SongWriting(Service, Vec<String>),
//         BeatWriting(Service, Vec<String>),

//         /// Vec with credits
//         GhostWriting(Service, Vec<String>),
//         CoverDesign(Service),
//     }

//     // impl Validate for SubmitServiceRequest {
//     //     fn validate(&self) -> Result<(), ValidationErrors> {
//     //         let mut errors = ValidationErrors::new();
//     //         let mut result = Result::Ok(());
//     //         let service = match self {
//     //             SubmitServiceRequest::Mixing(service, genres) => {
//     //                 if let Err(e) = validate_moods_genres(genres) {
//     //                     errors.add("mixing genres", e);
//     //                 }
//     //                 service
//     //             }
//     //             SubmitServiceRequest::SongWriting(service, genres) => {
//     //                 if let Err(e) = validate_moods_genres(genres) {
//     //                     errors.add("song writing genres", e);
//     //                 }
//     //                 service
//     //             }
//     //             SubmitServiceRequest::BeatWriting(service, genres) => {
//     //                 if let Err(e) = validate_moods_genres(genres) {
//     //                     errors.add("beat writing genres", e);
//     //                 }
//     //                 service
//     //             }
//     //             SubmitServiceRequest::GhostWriting(service, credits) => {
//     //                 if credits.len() > 5 {
//     //                     errors.add(
//     //                         "ghost writing",
//     //                         ValidationError::new(
//     //                             "Maximum credits for ghost writing is 5",
//     //                         ),
//     //                     );
//     //                 }
//     //                 for credit in credits.iter() {
//     //                     if credit.len() > 5000 {
//     //                         errors.add(
//     //                     "ghost writing",
//     //                     ValidationError::new(
//     //                         "Maximum length for ghost writing credits is 5000",
//     //                     ),
//     //                 );
//     //                     }
//     //                     break;
//     //                 }
//     //                 service
//     //             }
//     //             SubmitServiceRequest::CoverDesign(service) => service,
//     //         };
//     //         result = ValidationErrors::merge(
//     //             result,
//     //             "mixing service",
//     //             service.validate(),
//     //         );
//     //         result
//     //     }
//     // }
// }

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

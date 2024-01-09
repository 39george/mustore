use garde::Validate;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use time::OffsetDateTime;

use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::domain::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AcceptOffer {
    pub offer_id: i32,
}

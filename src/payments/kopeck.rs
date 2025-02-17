use std::str::FromStr;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::impl_debug;

impl_debug!(KopeckError);

#[derive(thiserror::Error)]
pub enum KopeckError {
    #[error("Wrong scale")]
    WrongScale(#[from] rust_decimal::Error),
    #[error("Number can't be negative for Kopeck")]
    NumberIsNegativeError,
    #[error("Number is too big")]
    OverflowError,
    #[error("Failed to parse string")]
    ParseError(#[source] rust_decimal::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Kopeck(u32);

impl Kopeck {
    /// Scale should be equal 2, and mantissa length should be <= 10 symbols.
    pub fn from_rub(mut rub: Decimal) -> Result<Kopeck, KopeckError> {
        if rub.scale() != 2 {
            tracing::warn!(
                "Given rub decimal scale is not 2, but {}",
                rub.scale()
            );
            rub.set_scale(2)?;
        }
        if rub.is_sign_negative() {
            return Err(KopeckError::NumberIsNegativeError);
        }
        let mantissa = rub.mantissa();
        if mantissa > u32::MAX as i128 {
            return Err(KopeckError::OverflowError);
        }
        let kopeck = mantissa as u32;
        Ok(Kopeck(kopeck))
    }

    /// Panics, if can't parse str as decimal
    pub fn from_rub_str(rub: &str) -> Result<Kopeck, KopeckError> {
        let parsed = rub.try_into().unwrap();
        Self::from_rub(parsed)
    }

    pub fn raw(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for Kopeck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

impl FromStr for Kopeck {
    type Err = KopeckError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let number = s.parse().map_err(KopeckError::ParseError)?;
        Kopeck::from_rub(number)
    }
}

//! src/lib.rs

use std::time::Duration;

use time::{
    format_description::well_known::{
        iso8601::{self, TimePrecision},
        Iso8601,
    },
    macros::format_description,
};

mod examples;

pub mod auth;
pub mod config;
pub mod cornucopia;
pub mod domain;
pub mod email_client;
pub mod html_template_gen;
pub mod middleware;
pub mod payments;
pub mod routes;
pub mod service_providers;
pub mod startup;
pub mod telemetry;
pub mod types;

// ───── Helpers ──────────────────────────────────────────────────────────── //

lazy_static::lazy_static! {
    pub static ref DEFAULT_TIME_FORMAT: &'static [time::format_description::FormatItem<'static>] = format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]");
}

const SIMPLE_ISO: Iso8601<6651332276402088934156738804825718784> = Iso8601::<
    {
        iso8601::Config::DEFAULT
            .set_year_is_six_digits(false)
            .set_time_precision(TimePrecision::Second {
                decimal_digits: None,
            })
            .encode()
    },
>;

time::serde::format_description!(iso_format, OffsetDateTime, SIMPLE_ISO);

pub const PRESIGNED_IMAGE_EXP: Duration = Duration::from_secs(10);

pub const MAX_MP3_SIZE_MB: u64 = 15;
pub const MAX_MULTITRACK_SIZE_GB: u64 = 5;
pub const MAX_WAV_SIZE_MB: u64 = 50;
pub const MAX_VIDEO_SIZE_MB: u64 = 50;
pub const MAX_IMAGE_SIZE_MB: u64 = 5;
pub const MAX_DEFAULT_SIZE_MB: u64 = 2;
pub const MAX_DOCUMENT_SIZE_MB: u64 = 5;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

#[macro_export]
macro_rules! impl_debug {
    ($type:ident) => {
        use crate::error_chain_fmt;
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                error_chain_fmt(self, f)
            }
        }
    };
}

/// This macro is for tracing error and returning Result if there are some
/// meaningful Ok() case, and returning () if there are no meaningful result.
/// It is useful to simply trace error message on fallible operations which doesn't
/// return anything in the Ok() branch.
#[macro_export]
macro_rules! trace_err {
    ($exp:expr) => {
        match $exp {
            Ok(v) => Ok(v),
            Err(e) => {
                tracing::error!("{e}");
                Err(e)
            }
        }
    };
    ($exp:expr, ()) => {
        match $exp {
            Ok(()) => (),
            Err(e) => {
                tracing::error!("{e}");
                ()
            }
        }
    };
}

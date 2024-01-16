//! src/lib.rs

use time::macros::format_description;

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

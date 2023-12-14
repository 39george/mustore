//! src/lib.rs

mod examples;

pub mod auth;
pub mod config;
pub mod domain;
pub mod email_client;
pub mod middleware;
pub mod routes;
pub mod startup;
pub mod telemetry;

// ───── Helpers ──────────────────────────────────────────────────────────── //

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

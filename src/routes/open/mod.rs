use std::collections::HashMap;

use anyhow::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing;
use axum::Json;
use axum::Router;
use http::StatusCode;
use serde::Deserialize;
use serde::Serialize;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::cornucopia::queries::open_access::get_stats;
use crate::cornucopia::queries::open_access::GetStats;
use crate::error_chain_fmt;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
pub enum ResponseError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Internal error")]
    InternalError(#[source] anyhow::Error),
}

impl std::fmt::Debug for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self);
        match self {
            ResponseError::UnexpectedError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            ResponseError::InternalError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Stats {
    beats: u32,
    songs: u32,
    lyrics: u32,
    covers: u32,
}

impl TryFrom<Vec<GetStats>> for Stats {
    type Error = anyhow::Error;
    fn try_from(value: Vec<GetStats>) -> Result<Self, Self::Error> {
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

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn open_router() -> Router<AppState> {
    Router::new()
        .route("/stats", routing::get(stats))
        .route("/genres", routing::get(genres))
        .route("/tags", routing::get(tags))
}

#[tracing::instrument(name = "Get products counts (stats)", skip_all)]
async fn stats(
    State(app_state): State<AppState>,
) -> Result<Json<Stats>, ResponseError> {
    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;
    let stats: Stats = get_stats()
        .bind(&pg_pool)
        .all()
        .await
        .context("Failed to get stats from pg")?
        .try_into()?;

    Ok(Json::from(stats))
}

async fn genres() -> StatusCode {
    StatusCode::OK
}

async fn tags() -> StatusCode {
    StatusCode::OK
}

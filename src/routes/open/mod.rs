use std::collections::HashMap;

use anyhow::Context;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::routing;
use axum::Json;
use axum::Router;
use serde::Deserialize;
use serde::Serialize;
use validator::ValidateArgs;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::cornucopia::queries::open_access;
use crate::cornucopia::queries::open_access::GetNewSongs;
use crate::cornucopia::queries::open_access::GetRecommendedSongs;
use crate::cornucopia::queries::open_access::GetSongs;
use crate::domain::requests::GetSongsListRequest;
use crate::startup::AppState;

use super::ResponseError;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Debug, Deserialize, Serialize)]
struct Stats {
    beats: u32,
    songs: u32,
    lyrics: u32,
    covers: u32,
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

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn open_router() -> Router<AppState> {
    Router::new()
        .route("/stats", routing::get(stats))
        .route("/:what", routing::get(get_values_list))
        .route("/songs", routing::get(get_songs))
        .route("/new_songs", routing::get(get_new_songs))
        .route("/recommended_songs", routing::get(get_recommended_songs))
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
    let stats: Stats = open_access::get_stats()
        .bind(&pg_pool)
        .all()
        .await
        .context("Failed to get stats from pg")?
        .try_into()?;

    Ok(Json::from(stats))
}

/// We should return json list
#[tracing::instrument(name = "Get values list", skip(app_state))]
async fn get_values_list(
    Path(path): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<String>>, ResponseError> {
    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;

    match path.as_str() {
        "genres" => Ok(Json(
            open_access::get_genres_list()
                .bind(&pg_pool)
                .all()
                .await
                .context("Failed to get genres list from pg")?,
        )),
        "tags" => Ok(Json(
            open_access::get_tags_list()
                .bind(&pg_pool)
                .all()
                .await
                .context("Failed to get genres list from pg")?,
        )),
        _ => Err(ResponseError::BadRequest(anyhow::anyhow!(
            "Can't send values of {path}!"
        ))),
    }
}

#[tracing::instrument(name = "Get songs query", skip(app_state))]
async fn get_songs(
    State(app_state): State<AppState>,
    Json(params): Json<GetSongsListRequest>,
) -> Result<Json<Vec<GetSongs>>, ResponseError> {
    params.validate_args((40, 320))?;

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;
    let songs = open_access::get_songs()
        .bind(
            &pg_pool,
            &params.sex.map(|s| s.to_string()),
            &params.tempo,
            &params
                .key
                .map(|v| v.into_iter().map(|el| el.into()).collect::<Vec<_>>()),
            &params.genres,
            &params.vibes,
            &params.sort_by.to_string(),
            &params.amount,
            &params.offset,
        )
        .all()
        .await
        .context("Failed to fetch songs data from pg")?;
    Ok(Json(songs))
}

#[tracing::instrument(name = "Get new songs query", skip(app_state))]
async fn get_new_songs(
    State(app_state): State<AppState>,
    Query(amount): Query<i64>,
) -> Result<Json<Vec<GetNewSongs>>, ResponseError> {
    if amount > 50 || amount < 1 {
        return Err(ResponseError::BadRequest(anyhow::anyhow!(
            "Amount of tracks should be between 1 and 50, requested: {}",
            amount
        )));
    }

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;

    let songs = open_access::get_new_songs()
        .bind(&pg_pool, &amount)
        .all()
        .await
        .context("Failed to fetch songs data from pg")?;

    Ok(Json(songs))
}

#[tracing::instrument(name = "Get recommended songs query", skip(app_state))]
async fn get_recommended_songs(
    State(app_state): State<AppState>,
    Query(amount): Query<i64>,
) -> Result<Json<Vec<GetRecommendedSongs>>, ResponseError> {
    if amount > 50 || amount < 1 {
        return Err(ResponseError::BadRequest(anyhow::anyhow!(
            "Amount of tracks should be between 1 and 50, requested: {}",
            amount
        )));
    }

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;

    let songs = open_access::get_recommended_songs()
        .bind(&pg_pool, &amount)
        .all()
        .await
        .context("Failed to fetch songs data from pg")?;

    Ok(Json(songs))
}

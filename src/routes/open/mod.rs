use std::collections::HashMap;

use anyhow::Context;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::routing;
use axum::Json;
use axum::Router;
use garde::Validate;
use serde::Deserialize;
use serde::Serialize;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::open_access;
use crate::cornucopia::queries::open_access::GetNewSongs;
use crate::cornucopia::queries::open_access::GetRecommendedSongs;
use crate::cornucopia::queries::open_access::GetSongsListResponse;
use crate::domain::requests::open_access::GetSongsListRequest;
use crate::startup::AppState;

use super::ResponseError;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct Stats {
    #[schema(example = "55")]
    beats: u32,
    #[schema(example = "12")]
    songs: u32,
    #[schema(example = "77")]
    lyrics: u32,
    #[schema(example = "71")]
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
        .route("/get_songs", routing::post(get_songs))
        .route("/new_songs", routing::get(get_new_songs))
        .route("/recommended_songs", routing::get(get_recommended_songs))
}

#[utoipa::path(
    get,
    path = "/api/open/stats",
    responses(
        (status = 200, description = "Got stats successfully", body = Stats),
        (status = 500, description = "Some internal error")
    )
)]
#[tracing::instrument(name = "Get products counts (stats)", skip_all)]
pub async fn stats(
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

/// Returns a json array with genres or moods
#[utoipa::path(
    get,
    path = "/api/open/{what}",
    params(
        ("what" = String, Path, description = "Which values would you get")
    ),
    responses(
        (status = 200, description = "Got values successfully", body = [String], content_type = "application/json",
            example = json!(["genre1", "genre2", "genre3"])
        ),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Some internal error")
    )
)]
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
        "moods" => Ok(Json(
            open_access::get_moods_list()
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

/// It's post request for songs list retrieval.
#[utoipa::path(
    post,
    path = "/api/open/get_songs",
    request_body = GetSongsListRequest,
    responses(
        (status = 200, description = "Got songs successfully", body = [GetSongsListResponse], content_type = "application/json"),
        (status = 400, description = "Bad request error"),
        (status = 417, description = "Validation error",
            content_type = "application/json",
            example = json!({"Validation error": "Tempo is out of bounds"})
        ),
        (status = 500, description = "Some internal error")
    )
)]
#[tracing::instrument(name = "Get songs post request", skip_all)]
async fn get_songs(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(params): Json<GetSongsListRequest>,
) -> Result<Json<Vec<GetSongsListResponse>>, ResponseError> {
    params.validate(&())?;

    let user_id = auth_session.user.map(|u| u.id);

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;

    let songs = open_access::get_songs()
        .bind(
            &pg_pool,
            &user_id,
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
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(amount): Query<i64>,
) -> Result<Json<Vec<GetNewSongs>>, ResponseError> {
    if amount > 50 || amount < 1 {
        return Err(ResponseError::BadRequest(anyhow::anyhow!(
            "Amount of tracks should be between 1 and 50, requested: {}",
            amount
        )));
    }

    let user_id = auth_session.user.map(|u| u.id);

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;

    let songs = open_access::get_new_songs()
        .bind(&pg_pool, &user_id, &amount)
        .all()
        .await
        .context("Failed to fetch songs data from pg")?;

    Ok(Json(songs))
}

#[tracing::instrument(name = "Get recommended songs query", skip(app_state))]
async fn get_recommended_songs(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(amount): Query<i64>,
) -> Result<Json<Vec<GetRecommendedSongs>>, ResponseError> {
    if amount > 50 || amount < 1 {
        return Err(ResponseError::BadRequest(anyhow::anyhow!(
            "Amount of tracks should be between 1 and 50, requested: {}",
            amount
        )));
    }

    let user_id = auth_session.user.map(|u| u.id);

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;

    let songs = open_access::get_recommended_songs()
        .bind(&pg_pool, &user_id, &amount)
        .all()
        .await
        .context("Failed to fetch songs data from pg")?;

    Ok(Json(songs))
}

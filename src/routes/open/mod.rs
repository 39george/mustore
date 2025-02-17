use anyhow::Context;
use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::routing;
use axum::Json;
use axum::Router;
use futures::future::try_join_all;
use garde::Validate;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::open_access;
use crate::cornucopia::queries::open_access::GetNewSongs;
use crate::cornucopia::queries::open_access::GetRecommendedSongs;
use crate::cornucopia::queries::open_access::GetSongsList;
use crate::domain::requests::open_access::GetSongsListRequest;
use crate::domain::requests::open_access::SongsAmount;
use crate::domain::requests::open_access::Stats;
use crate::startup::AppState;

use crate::startup::api_doc::BadRequestResponse;
use crate::startup::api_doc::InternalErrorResponse;
use crate::PRESIGNED_IMAGE_EXP;

use super::ErrorResponse;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn open_router() -> Router<AppState> {
    Router::new()
        .route("/stats", routing::get(stats))
        .route("/:what", routing::get(get_values_list))
        .route("/songs", routing::get(get_songs))
        .route("/new_songs", routing::get(get_new_songs))
        .route("/recommended_songs", routing::get(get_recommended_songs))
}

/// Request count of all main types contents
#[utoipa::path(
    get,
    path = "/api/open/stats",
    responses(
        (status = 200, description = "Got stats successfully", body = Stats),
        (status = 500, response = InternalErrorResponse)
    )
)]
#[tracing::instrument(name = "Get products counts (stats)", skip_all)]
pub async fn stats(
    State(app_state): State<AppState>,
) -> Result<Json<Stats>, ErrorResponse> {
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

/// Retrieve a json array with genres or moods
#[utoipa::path(
    get,
    path = "/api/open/{what}",
    params(
        ("what" = String, Path,
            description = "Which values would you get, can be 'genres', or 'moods'",
            example = "genres"
        )
    ),
    responses(
        (status = 200, description = "Got values successfully",
            body = [String], content_type = "application/json",
            example = json!(["classic", "pop", "folk"])
        ),
        (status = 400, response = BadRequestResponse),
        (status = 500, response = InternalErrorResponse)
    )
)]
#[tracing::instrument(name = "Get values list", skip(app_state))]
async fn get_values_list(
    Path(path): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<String>>, ErrorResponse> {
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
        _ => Err(ErrorResponse::BadRequest(anyhow::anyhow!(
            "Can't send values of {path}! Only 'genres' and 'moods' available!"
        ))),
    }
}

/// Retrieve filtered list of songs
#[utoipa::path(
    get,
    path = "/api/open/songs",
    params(
        GetSongsListRequest
    ),
    responses(
        (status = 200, description = "Got songs successfully",
            body = [GetSongsList],
            content_type = "application/json"
        ),
        (status = 400, response = BadRequestResponse),
        (status = 500, response = InternalErrorResponse)
    )
)]
#[tracing::instrument(name = "Get songs request", skip_all)]
async fn get_songs(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    axum_extra::extract::Query(params): axum_extra::extract::Query<
        GetSongsListRequest,
    >,
) -> Result<Json<Vec<GetSongsList>>, ErrorResponse> {
    params.validate(&())?;

    let user_id = auth_session.user.map(|u| u.id);

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;
    let s3 = app_state.object_storage;

    let songs: Vec<_> = open_access::get_songs()
        .bind(
            &pg_pool,
            &user_id,
            &params.sex.map(|s| s.to_string()),
            &params.tempo,
            &params
                .key
                .into_iter()
                .map(|el| el.into())
                .collect::<Vec<_>>(),
            &params.genres,
            &params.vibes,
            &params.sort_by.to_string(),
            &params.amount,
            &params.offset,
        )
        .all()
        .await
        .context("Failed to fetch songs data from pg")?
        .into_iter()
        .map(|mut song| async {
            song.cover_url = s3
                .generate_presigned_url(
                    &song
                        .cover_url
                        .parse()
                        .context("Failed to parse object key")?,
                    PRESIGNED_IMAGE_EXP,
                )
                .await?;
            Ok::<open_access::GetSongsList, ErrorResponse>(song)
        })
        .collect();
    let songs = try_join_all(songs).await?;
    Ok(Json(songs))
}

/// Retrieve certain amount of new songs
#[utoipa::path(
    get,
    path = "/api/open/new_songs",
    params(
        SongsAmount
    ),
    responses(
        (status = 200, description = "Got songs successfully",
            body = [GetNewSongs],
            content_type = "application/json"
        ),
        (status = 400, response = BadRequestResponse),
        (status = 500, response = InternalErrorResponse)
    )
)]
#[tracing::instrument(
    name = "Get new songs request",
    skip(auth_session, app_state)
)]
async fn get_new_songs(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(amount): Query<SongsAmount>,
) -> Result<Json<Vec<GetNewSongs>>, ErrorResponse> {
    amount.validate(&())?;

    let user_id = auth_session.user.map(|u| u.id);

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;

    let songs = open_access::get_new_songs()
        .bind(&pg_pool, &user_id, &amount.amount)
        .all()
        .await
        .context("Failed to fetch songs data from pg")?;

    Ok(Json(songs))
}

/// Retrieve certain amount of recommended songs
#[utoipa::path(
    get,
    path = "/api/open/recommended_songs",
    params(
        SongsAmount
    ),
    responses(
        (status = 200, description = "Got songs successfully",
            body = [GetRecommendedSongs],
            content_type = "application/json"
        ),
        (status = 400, response = BadRequestResponse),
        (status = 500, response = InternalErrorResponse)
    )
)]
#[tracing::instrument(
    name = "Get recommended songs request",
    skip(app_state, auth_session)
)]
async fn get_recommended_songs(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(amount): Query<SongsAmount>,
) -> Result<Json<Vec<GetRecommendedSongs>>, ErrorResponse> {
    amount.validate(&())?;

    let user_id = auth_session.user.map(|u| u.id);

    let pg_pool = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get pool from pg")?;

    let songs = open_access::get_recommended_songs()
        .bind(&pg_pool, &user_id, &amount.amount)
        .all()
        .await
        .context("Failed to fetch songs data from pg")?;

    Ok(Json(songs))
}

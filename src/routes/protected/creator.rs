use anyhow::Context;
use axum::extract::DefaultBodyLimit;
use axum::extract::State;
use axum::routing;
use axum::Json;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;
use rust_decimal::Decimal;
use serde::Deserialize;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::creator_access;
use crate::domain::music_parameters::MusicKey;
use crate::domain::music_parameters::Sex;
use crate::routes::ResponseError;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

// pub enum UserResponseError {
//     #[error(transparent)]
//     Common(#[from] ResponseError),
//     // User-specific errors here
// }

enum UploadLinkType {
    // Images
    NewAvatarImage,
    CoverCreditsDesign,
    ServiceCover,
    ProductCover,
    // Audio
    SongMasterFile,
    //SongMasterFileTagged?,
    SongMultitrackArchive,
    BeatMasterFile,
    //BeatMasterFileTagged?,
    BeatMultitrackArchive,
    MixingCreditsTrack,
    SongCreditsTrack,
    BeatCreditsTrack,
    // Other
    ServiceVideoDescription,
    MessageAttachment,
}

#[derive(Deserialize, Debug)]
struct RequestUploadLinkQuery {}

// FIXME validate this query, maybe implement domain validation?
#[derive(Deserialize, Debug)]
struct UploadSongQuery {
    name: String,
    description: String,
    price: Decimal,
    tags: Vec<String>,
    primary_genre: String,
    secondary_genre: Option<String>,
    sex: Sex,
    tempo: i16,
    duration: i16,
    key: MusicKey,
    lyric: String,
}

// ───── Body ─────────────────────────────────────────────────────────────── //

pub fn creator_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route("/upload_song", routing::post(upload_song))
        .layer(permission_required!(crate::auth::users::Backend, "creator",))
}

#[tracing::instrument(name = "Creator's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tracing::instrument(name = "Upload a new song", skip_all)]
async fn request_obj_storage_upload_link(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(params): Json<UploadSongQuery>,
) -> Result<StatusCode, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Upload a new song", skip_all)]
#[axum::debug_handler]
async fn upload_song(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(params): Json<UploadSongQuery>,
) -> Result<StatusCode, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")?;

    let product_id = creator_access::insert_product_and_get_product_id()
        .bind(
            &transaction,
            &user.id,
            &params.name,
            &params.description,
            &params.price,
        )
        .one()
        .await
        .context("Failed to insert song (product part) into the pg")?;

    let song_id = creator_access::insert_song_and_get_song_id().bind(
        &transaction,
        &product_id,
        &params.primary_genre,
        &params.secondary_genre,
        &params.sex.to_string(),
        &params.tempo,
        &params.key.into(),
        &params.duration,
        &params.lyric,
    );

    if let Err(e) = transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")
    {
        // app_state
        //     .object_storage
        //     .delete_object_by_key(&avatar_key)
        //     .await?;

        return Err(e.into());
    }

    Ok(StatusCode::CREATED)
}

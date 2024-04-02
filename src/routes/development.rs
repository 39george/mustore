use anyhow::Context;
use axum::extract::DefaultBodyLimit;
use axum::extract::Multipart;
use axum::extract::Query;
use axum::extract::State;
use axum::routing;
use axum::Router;
use fred::interfaces::KeysInterface;
use fred::interfaces::ServerInterface;
use fred::types::Scanner;
use futures::TryStreamExt;
use http::StatusCode;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use serde::Deserialize;
use utoipa::IntoParams;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::domain::upload_request::remove_outdated_uploads_from_redis;
use crate::domain::upload_request::UploadRequest;
use crate::service_providers::object_storage::presigned_post_form::PresignedPostData;
use crate::startup::AppState;
use crate::types::data_size::DataSizes;

use super::ErrorResponse;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Deserialize, Debug, utoipa::ToSchema)]
pub struct InputWithFiles {
    /// In OpenAPI Specification 3.0, files are defined as binary strings,
    /// that is, type: string + format: binary
    #[schema(value_type = String, format = Binary)]
    file: Vec<u8>,
    presigned_post_form: PresignedPostData,
}

#[derive(Deserialize, IntoParams)]
pub struct DbNumber {
    /// Number of redis database
    number: u8,
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn dev_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/upload",
            routing::post(upload_file)
                .layer(DefaultBodyLimit::max(5.gb_to_bytes())),
        )
        .route("/cleanup", routing::post(cleanup))
        .route("/reset_ban", routing::post(reset_ban))
        .with_state(state)
}

/// Upload a file using presigned post form.
#[utoipa::path(
    post,
    path = "/api/upload",
    request_body(
        content = InputWithFiles,
        content_type = "multipart/form-data"  
    ),
    responses(
        (
            status = 200,
            body = String,
            description = "Successfull uploaded file"
        ),
        (status = 500, description = "Some internall error"),
    ),
    tag = "development"
)]
#[tracing::instrument(name = "Upload file to object storage", skip_all)]
async fn upload_file(
    mut multipart: Multipart,
) -> Result<(StatusCode, String), ErrorResponse> {
    let mut file = None;
    let mut presigned_post_form = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().is_some_and(|f| f.eq("file")) {
            let data: Vec<u8> = field
                .bytes()
                .await
                .context("Failed to get bytes from multipart field")?
                .into();
            file = Some(data);
        } else if field.name().is_some_and(|f| f.eq("presigned_post_form")) {
            let data: PresignedPostData = serde_json::from_str(
                &field
                    .text()
                    .await
                    .context("Failed to get presigned post form as text")?,
            )
            .context("Failed to deserialize text into PresignedPostData")?;
            presigned_post_form = Some(data);
        }
    }

    let input = InputWithFiles {
        file: file.context("No file in input!")?,
        presigned_post_form: presigned_post_form
            .context("No presigned post form data!")?,
    };
    let client = reqwest::Client::builder().build().unwrap();
    let url = input.presigned_post_form.url;
    let mut multipart = Form::new();
    for (key, value) in input.presigned_post_form.fields.into_iter() {
        multipart = multipart.text(key, value);
    }
    multipart = multipart.part("file", Part::bytes(input.file));
    let response = client.post(url).multipart(multipart).send().await.unwrap();
    let status =
        StatusCode::from_u16(response.status().as_u16()).context("")?;
    let text = response
        .text()
        .await
        .context("Failed to get text from response")?;
    Ok((status, text))
}

/// Clear redis uploads, and object storage hanging uploads.
#[utoipa::path(
    post,
    path = "/api/cleanup",
    params(
        DbNumber
    ),
    responses(
        (status = 200, description = "Successfull cleanup"),
        (status = 500, description = "Some internall error"),
    ),
    tag = "development"
)]
#[tracing::instrument(name = "Cleanup redis && s3", skip_all)]
async fn cleanup(
    Query(DbNumber { number }): Query<DbNumber>,
    State(app_state): State<AppState>,
) -> Result<StatusCode, ErrorResponse> {
    let client = app_state.redis_pool.next();
    client.select(number).await.context("Failed to select db")?;
    let obj_storage = app_state.object_storage;
    // Delete all upload requests from redis
    let removed_keys =
        remove_outdated_uploads_from_redis(&client, |_| true).await;
    for key in removed_keys {
        match key.parse() {
            Ok(key) => {
                match obj_storage.delete_object_by_key(&key).await {
                    Ok(()) => tracing::info!("Object with key {key} is successfully deleted from obj storage"),
                    Err(e) => tracing::warn!("Failed to delete object with key {key} from object storage: {e}"),
                }
            },
            Err(e) => {
                tracing::error!("Failed to parse {key} as objet key: {e}");
            },
        }
    }
    Ok(StatusCode::OK)
}

/// Reset ban in redis.
#[utoipa::path(
    post,
    path = "/api/reset_ban",
    responses(
        (status = 200),
        (status = 500, description = "Some internall error"),
    ),
    tag = "development"
)]
#[tracing::instrument(name = "Reset ban in redis", skip_all)]
async fn reset_ban(State(state): State<AppState>) -> StatusCode {
    let con = state.redis_pool.next();
    let pattern = "username_status_req_limit*";
    let mut scan = con.scan(pattern, None, None);
    while let Ok(Some(mut page)) = scan.try_next().await {
        if let Some(keys) = page.take_results() {
            for key in keys.into_iter() {
                let _ = con.del::<(), _>(&key).await;
            }
        }
    }
    StatusCode::OK
}

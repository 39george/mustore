use anyhow::Context;
use axum::extract::DefaultBodyLimit;
use axum::extract::Multipart;
use axum::extract::State;
use axum::routing;
use axum::Json;
use axum::Router;
use http::StatusCode;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use serde::Deserialize;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::service_providers::object_storage::presigned_post_form::PresignedPostData;
use crate::startup::AppState;
use crate::types::data_size::DataSizes;

use super::ResponseError;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Deserialize, Debug, utoipa::ToSchema)]
pub struct InputWithFiles {
    /// In OpenAPI Specification 3.0, files are defined as binary strings,
    /// that is, type: string + format: binary
    #[schema(value_type = String, format = Binary)]
    file: Vec<u8>,
    presigned_post_form: PresignedPostData,
}

// ───── Handlers ─────────────────────────────────────────────────────────── //
pub fn user_router() -> Router {
    Router::new().route(
        "/upload",
        routing::post(upload_file)
            .layer(DefaultBodyLimit::max(5.gb_to_bytes())),
    )
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
        (status = 403, description = "Forbidden")
    ),
    tag = "development"
)]
#[axum::debug_handler]
#[tracing::instrument(name = "Upload file to object storage", skip_all)]
async fn upload_file(
    mut multipart: Multipart,
) -> Result<(StatusCode, String), ResponseError> {
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

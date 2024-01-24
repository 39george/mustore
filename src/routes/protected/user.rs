use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::Context;
use axum::extract::Query;
use axum::extract::State;
use axum::routing;
use axum::Json;
use axum::Router;
use axum_login::permission_required;
use axum_login::AuthzBackend;
use fred::clients::RedisPool;
use fred::interfaces::KeysInterface;
use fred::prelude::RedisResult;
use fred::types::Scanner;
use futures::TryStreamExt;
use garde::Validate;
use http::StatusCode;
use mediatype::media_type;
use mediatype::MediaTypeBuf;
use time::OffsetDateTime;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::auth::users::Permission;
use crate::cornucopia::queries::user_access;
use crate::cornucopia::queries::user_access::GetConversationsEntries;
use crate::domain::requests::user_access::CreateConversationRequest;
use crate::domain::requests::user_access::GetConversationRequest;
use crate::domain::requests::user_access::ListConversationRequest;
use crate::domain::requests::user_access::SendMessageRequest;
use crate::domain::requests::user_access::UploadFileRequest;
use crate::domain::responses::user_access::ConversationDataResponse;
use crate::domain::upload_request::UploadRequest;
use crate::routes::ResponseError;
use crate::service_providers::object_storage::presigned_post_form::PresignedPostData;
use crate::startup::api_doc::BadRequestResponse;
use crate::startup::AppState;
use crate::types::data_size::DataSizes;

// ───── Types ────────────────────────────────────────────────────────────── //

// Define the static variable MAX_SIZES for acceptable media types.
lazy_static::lazy_static! {
    pub static ref MAX_SIZES: HashMap<MediaTypeBuf, u64> = {
        let mut m = HashMap::new();
        m.insert(media_type!(IMAGE/PNG).into(), crate::MAX_IMAGE_SIZE_MB.mb_to_bytes());
        m.insert(media_type!(IMAGE/JPEG).into(), crate::MAX_IMAGE_SIZE_MB.mb_to_bytes());
        m.insert(media_type!(AUDIO/WAV).into(), crate::MAX_WAV_SIZE_MB.mb_to_bytes());
        m.insert(media_type!(AUDIO/MPEG).into(), crate::MAX_MP3_SIZE_MB.mb_to_bytes());
        m.insert(media_type!(APPLICATION/ZIP).into(), crate::MAX_MULTITRACK_SIZE_GB.gb_to_bytes());
        m.insert(media_type!(VIDEO/MP4).into(), crate::MAX_VIDEO_SIZE_MB.mb_to_bytes());
        m.insert(media_type!(APPLICATION/PDF).into(), crate::MAX_DOCUMENT_SIZE_MB.mb_to_bytes());
        m.insert(
            media_type!(APPLICATION/vnd::OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT).into(),
            crate::MAX_DOCUMENT_SIZE_MB.mb_to_bytes(),
        );
        m
    };
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route("/upload_form", routing::get(request_obj_storage_upload))
        .route("/conversations", routing::get(get_conversations))
        .route("/conversation_id", routing::get(get_conversation_id))
        .route("/new_conversation", routing::post(create_new_conversation))
        .route("/send_message", routing::post(send_message))
        .route("/list_conversation", routing::get(list_conversation))
        .route("/permissions", routing::get(user_permissions))
        .layer(permission_required!(crate::auth::users::Backend, "user"))
}

/// Check access to user's endpoint.
#[utoipa::path(
    get,
    path = "/api/protected/user/health_check",
    responses(
        (status = 200, description = "Accessed to protected health check"),
        (status = 403, description = "Forbidden")
    ),
    security(
     ("api_key" = [])
    ),
    tag = "health_checks"
)]
#[tracing::instrument(name = "User's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Get list of user permissions.
#[utoipa::path(
    get,
    path = "/api/protected/user/permissions",
    responses(
        (status = 200, response = Permission ),
        (status = 403, description = "Forbidden")
    ),
    security(
     ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(name = "Get list of user permissions", skip_all)]
async fn user_permissions(
    auth_session: AuthSession,
) -> Result<Json<HashSet<Permission>>, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    let all_permissions =
        auth_session.backend.get_all_permissions(&user).await?;
    Ok(Json(all_permissions))
}

/// Get presigned post form to upload a file to the object storage.
#[utoipa::path(
    get,
    path = "/api/protected/user/upload_form",
    params(
        UploadFileRequest
    ),
    responses(
        (status = 200, response = PresignedPostData),
        (status = 400, response = BadRequestResponse),
        (
            status = 415,
            description = "Server will not generate presigned post form for provided media type",
            content_type = "application/json",
            body = String,
            example = json!({"AllowedFormats": ["application/zip", "audio/wav", "video/mp4"]})
        ),
        (status = 403, description = "Forbidden")
    ),
    security(
     ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(
    name = "Request post form data for obj store upload.",
    skip_all,
    fields(username)
)]
async fn request_obj_storage_upload(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(params): Query<UploadFileRequest>,
) -> Result<Json<PresignedPostData>, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    params.validate(&())?;

    check_current_user_uploads(&app_state.redis_pool, user.id)
        .await
        .context("Failed to check user uploads")?;

    let max_size = match MAX_SIZES.get(&params.media_type) {
        Some(&max_size) => max_size,
        None => {
            tracing::warn!("Wrong media type: {}", params.media_type);
            return Err(ResponseError::UnsupportedMediaTypeError);
        }
    };

    let object_key = format!(
        "upload/{}-{}-{}",
        user.username,
        uuid::Uuid::new_v4(),
        params.file_name
    );

    let presigned_post_data =
        app_state.object_storage.generate_presigned_post_form(
            &object_key,
            params.media_type,
            max_size,
        )?;

    store_upload_request_data(&app_state.redis_pool, &object_key, user.id)
        .await
        .context("Failed to store upload data in the redis.")?;

    Ok(Json(presigned_post_data))
}

#[tracing::instrument(
    name = "Get ordinary conversations list",
    skip_all,
    fields(username)
)]
async fn get_conversations(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<GetConversationsEntries>>, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let entries = user_access::get_conversations_entries()
        .bind(&db_client, &user.id)
        .all()
        .await
        .context("Failed to get conversations list from pg")?;

    Ok(Json(entries))
}

#[tracing::instrument(
    name = "Get conversation id by user id",
    skip_all,
    fields(username)
)]
async fn get_conversation_id(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(GetConversationRequest { with_user_id }): Query<
        GetConversationRequest,
    >,
) -> Result<Json<Option<i32>>, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let conversation_id = user_access::get_conversation_by_user_id()
        .bind(&db_client, &user.id, &with_user_id)
        .one()
        .await
        .context("Failed to get conversation id by user id")?;

    Ok(Json(conversation_id))
}

#[tracing::instrument(
    name = "Create new conversation",
    skip_all,
    fields(username, with_user_id)
)]
async fn create_new_conversation(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(CreateConversationRequest { with_user_id }): Query<
        CreateConversationRequest,
    >,
) -> Result<(StatusCode, Json<i32>), ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")?;

    let conversation_id = user_access::create_new_conversation()
        .bind(&transaction)
        .one()
        .await
        .context("Failed to create new conversation")?;

    let count = user_access::add_participants_to_conversation()
        .bind(&transaction, &conversation_id, &user.id, &with_user_id)
        .await
        .context("Failed to add participants to the conversation")?;

    transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")?;

    if count != 2 {
        Err(ResponseError::InternalError(anyhow::anyhow!(
            "Count was equal {count}, but should be 2"
        )))
    } else {
        Ok((StatusCode::CREATED, Json(conversation_id)))
    }
}

#[tracing::instrument(name = "Send a message", skip_all, fields(username))]
async fn send_message(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(params): Json<SendMessageRequest>,
) -> Result<StatusCode, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    params.validate(&())?;

    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")?;

    let message_id = user_access::insert_new_message()
        .bind(
            &transaction,
            &params.conversation_id,
            &params.service_id,
            &user.id,
            &params.reply_message_id,
            &params.text,
        )
        .one()
        .await
        .context("Failed to insert new message to pg.")?;

    for attachment in &params.attachments {
        user_access::insert_message_attachment()
            .bind(&transaction, attachment, &message_id)
            .await
            .context("Failed to insert message attachment to pg.")?;
    }

    remove_attachments_data_from_redis(
        &app_state.redis_pool,
        &params.attachments,
        user.id,
    )
    .await
    .context(
        "Failed to remove message attachments upload information from redis.",
    )?;

    transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")?;

    Ok(StatusCode::CREATED)
}

#[tracing::instrument(name = "List conversation", skip_all, fields(username))]
async fn list_conversation(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(ListConversationRequest {
        conversation_id,
        offset,
    }): Query<ListConversationRequest>,
) -> Result<Json<ConversationDataResponse>, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let conversations = user_access::list_conversation_by_id()
        .bind(&db_client, &conversation_id, &offset)
        .all()
        .await
        .context("Failed to fetch conversations from db")?;

    let response = ConversationDataResponse::new(
        conversations,
        &app_state.object_storage,
        user.id,
    )
    .await
    .context("Failed to build conversation response")?;

    Ok(Json(response))
}

// ───── Functions ────────────────────────────────────────────────────────── //

#[tracing::instrument(
    name = "Store upload request data in the redis",
    skip_all
)]
async fn store_upload_request_data(
    con: &RedisPool,
    object_key: &str,
    user_id: i32,
) -> RedisResult<()> {
    let created_at = OffsetDateTime::now_utc()
        .format(&crate::DEFAULT_TIME_FORMAT)
        .unwrap();
    let upload_request = UploadRequest::new(user_id, object_key);
    con.set(&upload_request.to_string(), &created_at, None, None, false)
        .await?;
    Ok(())
}

#[tracing::instrument(name = "Check current user uploads redis", skip_all)]
async fn check_current_user_uploads(
    con: &RedisPool,
    user_id: i32,
) -> Result<(), ResponseError> {
    let pattern = format!("upload_request:{}*", user_id);
    let mut scan = con.next().scan(pattern, None, None);
    while let Ok(Some(mut page)) = scan.try_next().await {
        if let Some(keys) = page.take_results() {
            if keys.len() > 15 {
                tracing::error!(
                    "User {} already have 15 current uploads",
                    user_id
                );
                return Err(ResponseError::TooManyUploadsError);
            } else if keys.len() > 5 {
                tracing::warn!(
                    "User {} already have 5 current uploads",
                    user_id
                );
            }
        }
        page.next().context("Failed to move on to the next page of results from the SCAN operation")?;
    }
    Ok(())
}

#[tracing::instrument(name = "Remove upload data from redis.", skip_all)]
async fn remove_attachments_data_from_redis(
    con: &RedisPool,
    keys: &Vec<String>,
    user_id: i32,
) -> RedisResult<()> {
    for obj_key in keys.iter() {
        let upload_request = UploadRequest::new(user_id, obj_key).to_string();

        // Check that there are such upload is
        let _created_at: String = con.get(&upload_request).await?;

        con.del(&upload_request).await?;
    }
    Ok(())
}

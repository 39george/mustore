use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;

use anyhow::Context;
use axum::extract::Query;
use axum::extract::State;
use axum::routing;
use axum::Json;
use axum::Router;
use axum_login::permission_required;
use axum_login::AuthzBackend;
use futures::future::try_join_all;
use garde::Validate;
use http::StatusCode;
use mediatype::media_type;
use mediatype::MediaTypeBuf;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::auth::users::Permission;
use crate::cornucopia::queries::user_access;
use crate::cornucopia::queries::user_access::GetConversationsEntries;
use crate::domain::object_key::ObjectKey;
use crate::domain::requests::user_access::CreateConversationRequest;
use crate::domain::requests::user_access::GetConversationRequest;
use crate::domain::requests::user_access::ListConversationRequest;
use crate::domain::requests::user_access::SendMessageRequest;
use crate::domain::requests::user_access::UploadFileRequest;
use crate::domain::responses::user_access::ConversationDataResponse;
use crate::domain::responses::user_access::DialogId;
use crate::domain::upload_request::check_current_user_uploads;
use crate::domain::upload_request::delete_upload_request_data_from_redis;
use crate::domain::upload_request::store_upload_request_data;
use crate::domain::upload_request::verify_upload_request_data_in_redis;
use crate::domain::user_name::UserName;
use crate::routes::ErrorResponse;
use crate::service_providers::object_storage::presigned_post_form::PresignedPostData;
use crate::startup::api_doc::BadRequestResponse;
use crate::startup::api_doc::ConflictErrorResponse;
use crate::startup::api_doc::InternalErrorResponse;
use crate::startup::api_doc::NotFoundResponse;
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
        .route("/dialog_id", routing::get(get_dialog_id))
        .route("/new_conversation", routing::post(create_new_conversation))
        .route("/send_message", routing::post(send_message))
        .route("/list_conversation", routing::get(list_conversation))
        .route("/permissions", routing::get(user_permissions))
        .route("/avatar_username", routing::get(avatar_username))
        .layer(permission_required!(crate::auth::users::Backend, "user"))
}

#[tracing::instrument(name = "User's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// Get list of user permissions.
#[utoipa::path(
    get,
    path = "/api/protected/user/permissions",
    responses(
        (status = 200, response = Permission),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
     ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(name = "Get list of user permissions", skip_all)]
async fn user_permissions(
    auth_session: AuthSession,
) -> Result<Json<HashSet<Permission>>, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    let all_permissions =
        auth_session.backend.get_all_permissions(&user).await?;
    Ok(Json(all_permissions))
}

/// Get user's avatar url and username.
#[utoipa::path(
    get,
    path = "/api/protected/user/avatar_username",
    responses(
        (
            status = 200,
            body = GetUserAvatarUsername,
            content_type = "application/json",
            description = "Avatar url and username",
            example = json!(
                [
                  {
                     "username": "someusername",
                     "avatar": "https://someurl.png",
                  }
                ]
            )
        ),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
     ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(name = "Get avatar url and username", skip_all)]
async fn avatar_username(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
) -> Result<Json<user_access::GetUserAvatarUsername>, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let mut avatar_username = user_access::get_user_avatar_username()
        .bind(&db_client, &user.id)
        .one()
        .await
        .context("Failed to get conversations list from pg")?;
    let object_key: ObjectKey = avatar_username
        .avatar
        .parse()
        .context("Failed to parse avatar object key")?;
    let result = app_state
        .object_storage
        .generate_presigned_url(&object_key, Duration::from_secs(1))
        .await?;
    avatar_username.avatar = result;
    Ok(Json(avatar_username))
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
        (status = 403, description = "Forbidden"),
        (
            status = 415,
            description = "Server will not generate presigned post form for provided media type",
            content_type = "application/json",
            body = String,
            example = json!({"AllowedFormats": ["application/zip", "audio/wav", "video/mp4"]})
        ),
        (status = 500, response = InternalErrorResponse)
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
) -> Result<Json<PresignedPostData>, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
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
            return Err(ErrorResponse::UnsupportedMediaTypeError);
        }
    };

    let object_key = ObjectKey::new(
        "upload",
        &user.username,
        uuid::Uuid::new_v4(),
        &params.file_name,
    )
    .context("Failed to build object key")
    .map_err(ErrorResponse::BadRequest)?;

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

/// Get conversations list.
#[utoipa::path(
    get,
    path = "/api/protected/user/conversations",
    responses(
        (
            status = 200,
            body = Vec<GetConversationsEntries>,
            content_type = "application/json",
            description = "Conversations list",
            example = json!(
                [
                  {
                    "conversation_id": 1236,
                    "image_url": "https://images.com/image123.png",
                    "interlocutor": "Jack",
                    "last_message_text": "Hi, How do you do!",
                    "last_message_timestamp": "2024-01-24T09:31:39.404Z",
                    "unread_messages_count": 3
                  }
                ]
            )
        ),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(
    name = "Get ordinary conversations list",
    skip_all,
    fields(username)
)]
async fn get_conversations(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<GetConversationsEntries>>, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let futures = user_access::get_conversations_entries()
        .bind(&db_client, &user.id)
        .all()
        .await
        .context("Failed to get conversations list from pg")?
        .into_iter()
        .map(|mut entry| {
            let obj_storage = app_state.object_storage.clone();
            async move {
                let object_key: ObjectKey = entry
                    .image_url
                    .parse()
                    .context("Failed to parse object key")?;
                let result = obj_storage
                    .generate_presigned_url(&object_key, Duration::from_secs(1))
                    .await?;
                entry.image_url = result;
                Ok::<GetConversationsEntries, ErrorResponse>(entry)
            }
        });

    let entries = try_join_all(futures).await?;

    Ok(Json(entries))
}

/// Get dialog id by user id, if not found, returns NotFound: 'dialog_id'.
#[utoipa::path(
    get,
    path = "/api/protected/user/dialog_id",
    params(
        ("with_username" = String, Query, description = "Username which one you request dialog with")
    ),
    responses(
        (
            status = 200,
            body = DialogId,
            content_type = "application/json",
            description = "Dialog id",
            example = json!({
                "id": 123
            })
        ),
        (status = 403, description = "Forbidden"),
        (status = 404, response = NotFoundResponse),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(
    name = "Get dialog id by user id",
    skip(auth_session, app_state),
    fields(username)
)]
async fn get_dialog_id(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(GetConversationRequest {
        with_user: with_username,
    }): Query<GetConversationRequest>,
) -> Result<Json<DialogId>, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);
    tracing::Span::current().record("dialog_with", &with_username);
    let _ =
        UserName::parse(&with_username).map_err(ErrorResponse::BadRequest)?;

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let id =
        get_dialog_by_username(&db_client, &user.id, &with_username).await?;

    Ok(Json(DialogId { id }))
}

/// Create a new conversation. Returns 409 Conflict, if conversation already exists.
#[utoipa::path(
    post,
    path = "/api/protected/user/new_conversation",
    params(
        CreateConversationRequest
    ),
    responses(
        (
            status = 201,
            body = DialogId,
            content_type = "application/json",
            description = "Conversation id",
            example = json!({
                "id": 123
            })
        ),
        (status = 403, description = "Forbidden"),
        (status = 404, response = NotFoundResponse),
        (status = 409, response = ConflictErrorResponse),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(
    name = "Create new conversation",
    skip(auth_session, app_state),
    fields(username)
)]
async fn create_new_conversation(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(CreateConversationRequest { with_username }): Query<
        CreateConversationRequest,
    >,
) -> Result<(StatusCode, Json<DialogId>), ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);
    let _ =
        UserName::parse(&with_username).map_err(ErrorResponse::BadRequest)?;

    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")?;

    // Check that `with_user_id` exists in db
    let with_user_id = match user_access::user_exists()
        .bind(&transaction, &with_username)
        .opt()
        .await
        .context("Failed to fetch user by id from pg")?
    {
        Some(id) => id,
        None => {
            return Err(ErrorResponse::NotFoundError(
                anyhow::anyhow!("Failed to get user id by username"),
                "with_username",
            ))
        }
    };

    if get_dialog_by_username(&transaction, &user.id, &with_username)
        .await
        .is_ok()
    {
        return Err(ErrorResponse::ConflictError(anyhow::anyhow!(
            "Can't create a new dialog: it is already exists!"
        )));
    }

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
        Err(ErrorResponse::InternalError(anyhow::anyhow!(
            "Count was equal {count}, but should be 2"
        )))
    } else {
        Ok((
            StatusCode::CREATED,
            Json(DialogId {
                id: conversation_id,
            }),
        ))
    }
}

/// Send a new message.
#[utoipa::path(
    post,
    path = "/api/protected/user/send_message",
    request_body (
        content = SendMessageRequest,
        content_type = "application/Json",
        example = json!({
            "attachments": ["attachment1.wav"],
            "conversation_id": 2,
            "reply_message_id": null,
            "service_id": null,
            "text": "this is a new message"
        }),
    ),
    responses(
        (status = 201, description = "Message is created"),
        (status = 403, description = "Forbidden"),
        (status = 404, response = NotFoundResponse),
        (status = 500, description = "Something happened on the server, or provided id's were incorrect")
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(name = "Send a message", skip_all, fields(username))]
async fn send_message(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Json(params): Json<SendMessageRequest>,
) -> Result<StatusCode, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    params.validate(&())?;

    let s3 = app_state.object_storage;

    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")?;

    check_conversation_exists(&transaction, &params.conversation_id).await?;
    check_conversation_access(
        &transaction,
        &user.id,
        &user.username,
        &params.conversation_id,
    )
    .await?;

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
        .context("Failed to insert a new message into pg")?;

    let attachments = params.attachments.iter().collect::<Vec<_>>();
    verify_upload_request_data_in_redis(
        &app_state.redis_pool,
        &attachments,
        user.id,
    )
    .await
    .context("Failed to verify attachments data redis")?;
    delete_upload_request_data_from_redis(
        &app_state.redis_pool,
        &attachments,
        user.id,
    )
    .await
    .context("Failed to remove attachments from redis")?;

    for attachment in &params.attachments {
        // Move attachments into `received` folder
        let new_key = s3
            .receive(&attachment)
            .await
            .map_err(ErrorResponse::ObjectStorageError)?;

        user_access::insert_message_attachment()
            .bind(&transaction, &new_key.as_ref(), &message_id)
            .await
            .context("Failed to insert message attachment into pg.")?;
    }

    transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")?;

    Ok(StatusCode::CREATED)
}

/// List conversation by id, 30 entries returned.
/// Can return forbidden, if has no access to the provided conversation id.
#[utoipa::path(
    get,
    path = "/api/protected/user/list_conversation",
    params(
        ListConversationRequest
    ),
    responses(
        (status = 201, response = ConversationDataResponse),
        (status = 403, description = "Forbidden"),
        (status = 404, response = NotFoundResponse),
        (status = 500, description = "Something happened on the server, or provided id were incorrect")
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.users"
)]
#[tracing::instrument(name = "List conversation", skip_all, fields(username))]
async fn list_conversation(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Query(ListConversationRequest {
        conversation_id,
        offset,
    }): Query<ListConversationRequest>,
) -> Result<Json<ConversationDataResponse>, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    check_conversation_exists(&db_client, &conversation_id).await?;
    check_conversation_access(
        &db_client,
        &user.id,
        &user.username,
        &conversation_id,
    )
    .await?;

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

/// Check that user has access to the conversation
#[tracing::instrument(name = "Check conversation access", skip_all)]
async fn check_conversation_access<T: cornucopia_async::GenericClient>(
    db_client: &T,
    user_id: &i32,
    username: &str,
    conversation_id: &i32,
) -> Result<i32, ErrorResponse> {
    user_access::user_has_access_to_conversation()
        .bind(db_client, user_id, conversation_id)
        .opt()
        .await
        .context("Failed to fetch conversation access from db")?
        .ok_or(ErrorResponse::ForbiddenError(anyhow::anyhow!(
            "{} has no access to the requested conversation id: {}",
            username,
            conversation_id
        )))
}

/// Check that conversation exists
#[tracing::instrument(name = "Check conversation exists", skip_all)]
async fn check_conversation_exists<T: cornucopia_async::GenericClient>(
    db_client: &T,
    conversation_id: &i32,
) -> Result<i32, ErrorResponse> {
    user_access::conversation_exists()
        .bind(db_client, conversation_id)
        .opt()
        .await
        .context("Failed to fetch conversation access from db")?
        .ok_or(ErrorResponse::NotFoundError(
            anyhow::anyhow!(
                "Conversation with id {conversation_id} not exists",
            ),
            "conversation_id",
        ))
}

async fn get_dialog_by_username<T: cornucopia_async::GenericClient>(
    client: &T,
    user_id: &i32,
    with_username: &str,
) -> Result<i32, ErrorResponse> {
    user_access::get_dialog_by_username()
        .bind(client, user_id, &with_username)
        .opt()
        .await
        .context("Failed to get conversation id by user id")?
        .ok_or(ErrorResponse::NotFoundError(
            anyhow::anyhow!("Optional from db was none"),
            "dialog_id",
        ))
}

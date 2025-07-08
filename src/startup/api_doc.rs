//! We derive ToSchema to all types need to show their fields to frontend,
//! and derive ToResponse to all types we bind as `response = Type`.
//! We only need ToSchema derived if we set response as `body = Type`.

use utoipa::{
    openapi::{
        security::{ApiKey, ApiKeyValue, SecurityScheme},
        ServerBuilder,
    },
    Modify, OpenApi, ToResponse, ToSchema,
};

use crate::{
    auth::{self, users::Permission},
    domain::responses::user_access::DialogId,
};
use crate::{domain::general_types::ProductStatus, routes::protected};
use crate::{
    domain::requests::user_access::SendMessageRequest, routes::development,
};
use crate::{
    domain::responses::user_access::ConversationDataResponse, routes::open,
};

// ───── ErrorResponses ───────────────────────────────────────────────────── //

#[derive(ToResponse)]
#[response(description = "Something happened on the server")]
pub struct InternalErrorResponse;

#[derive(ToResponse)]
#[response(description = "You not allowed to access this method")]
pub struct ForbiddenResponse;

#[allow(dead_code)]
#[derive(ToResponse)]
#[response(
    description = "Request was formed erroneously",
    content_type = "application/json",
    example = json!({
        "caused_by":
        "Here will be the reason of a rejection"
    }),
)]
pub struct BadRequestResponse(String);

#[derive(ToResponse)]
#[response(description = "Not acceptable error")]
pub struct NotAcceptableErrorResponse;

#[allow(dead_code)]
#[derive(ToResponse)]
#[response(
    description = "Unauthorized error",
    content_type = "text/plain",
    example = json!({
        "caused_by":
        "Auth is required"
    }),
)]
pub struct UnauthorizedErrorResponse(String);

#[derive(ToResponse)]
#[response(description = "Too many uploads error")]
pub struct TooManyUploadsErrorResponse;

// We use ToSchema here, because we write manually in every case,
// inlined, description, examples etc.
#[derive(ToResponse)]
#[response(
    description = "Not found some data (param name passed)",
    content_type = "application/json",
    example = json!({
        "param": "param_name" }),
)]
pub struct NotFoundResponse {
    _param: String,
}

#[derive(ToResponse)]
#[response(description = "Conflict error")]
pub struct ConflictErrorResponse;

// ───── Responses ────────────────────────────────────────────────────────── //

#[derive(ToSchema)]
#[schema(as = GetSongsList)]
pub struct GetSongsListResponse {
    pub song_id: i32,
    pub created_at: time::OffsetDateTime,
    pub cover_url: String,
    pub name: String,
    pub author: String,
    pub likes: i64,
    pub listenings: i64,
    pub relevance_score: rust_decimal::Decimal,
    pub price: rust_decimal::Decimal,
    pub is_user_liked: Option<bool>,
}

#[derive(ToSchema)]
#[schema(as = GetNewSongs)]
pub struct GetNewSongsResponse {
    pub song_id: i32,
    pub created_at: time::OffsetDateTime,
    pub cover_url: String,
    pub name: String,
    pub author: String,
    pub likes: i64,
    pub price: rust_decimal::Decimal,
    pub is_user_liked: Option<bool>,
}

#[derive(ToSchema)]
#[schema(as = GetRecommendedSongs)]
pub struct GetRecommendedSongsResponse {
    pub song_id: i32,
    pub created_at: time::OffsetDateTime,
    pub cover_url: String,
    pub name: String,
    pub author: String,
    pub likes: i64,
    pub price: rust_decimal::Decimal,
    pub is_user_liked: Option<bool>,
}

#[allow(dead_code)]
#[derive(ToSchema, ToResponse)]
#[response(
    description = "If value exists",
    content_type = "application/json",
    example = json!({"exists": true }),
)]
pub struct IsExistsResponse(String);

#[derive(ToSchema)]
#[schema(as = GetConversationsEntries)]
pub struct GetConversationsEntriesResponse {
    pub conversation_id: i32,
    pub interlocutor: String,
    pub last_message_text: String,
    pub last_message_timestamp: time::OffsetDateTime,
    pub image_url: String,
    pub unread_messages_count: i64,
}

#[derive(ToSchema)]
#[schema(as = Products)]
pub struct Products {
    pub product_name: String,
    pub author_username: String,
    pub price: rust_decimal::Decimal,
    pub product_cover: String,
}

#[derive(ToSchema)]
#[schema(as = GetUserAvatarUsername)]
pub struct GetUserAvatarUsername {
    pub username: String,
    pub avatar: String,
}

#[derive(ToSchema)]
#[schema(as = GetCreatorMarksAvg)]
pub struct GetCreatorMarksAvg {
    pub avg: rust_decimal::Decimal,
    pub count: i64,
}

#[derive(ToResponse)]
#[response(
    description = "Get creator's songs",
    content_type = "application/json",
    example = json!({
    "song_id": "5",
    "name": "pretty song",
    "price": "1200",
    "key": "http://some_url_to_cover.png",
    "primary_genre": "jazz",
    "secondary_genre": "pop",
    "tempo": "110",
    "music_key": "a_major",
    "sex": "Female",
    "duration": "2.25",
    "lyric": "Lalalalaaaa, la",
    "moods": [
        "kind",
        "lovely"
    ],
    "likes_count": "1210",
    "listenings_count": "12346",
    }),
)]
pub struct GetCreatorSongs {
    pub song_id: i32,
    pub status: ProductStatus,
    pub name: String,
    pub price: rust_decimal::Decimal,
    pub key: String,
    pub primary_genre: i32,
    pub secondary_genre: i32,
    pub tempo: i16,
    pub music_key: Musickey,
    pub sex: String,
    pub duration: i16,
    pub lyric: String,
    pub moods: Vec<String>,
    pub likes_count: i64,
    pub listenings_count: i64,
}

// ───── TypeWrappers ─────────────────────────────────────────────────────── //

#[allow(dead_code)]
#[derive(ToSchema)]
#[schema(as = Secret)]
pub struct Password(String);

#[allow(dead_code)]
#[derive(ToSchema)]
#[schema(as = mediatype::MediaTypeBuf)]
pub struct MediaType(String);

#[allow(dead_code)]
#[derive(ToSchema)]
#[schema(
    value_type = String,
    example = "received/Lisa:21C960E7-5CA8-4974-98D7-6501DCCCAFD7:file.ext"
)]
pub struct ObjectKey(String);

#[derive(ToSchema)]
#[schema(as = MusicKey)]
pub struct Musickey;

// ───── Addons ───────────────────────────────────────────────────────────── //

/// NOTE: We use security addon also as simple marker.
/// For example, endpoint is protected by cookie auth, we will mark
/// endpoint as secured with "api_key" = [].
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("id"))),
            )
        }
    }
}

struct ServerAddon;

impl Modify for ServerAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let server = ServerBuilder::new()
            .description(Some("Development server"))
            .build();
        openapi.servers = Some(vec![server]);
    }
}

// ───── Api ──────────────────────────────────────────────────────────────── //

#[derive(OpenApi)]
#[openapi(
        paths(
            open::stats,
            open::get_values_list,
            open::get_songs,
            open::get_new_songs,
            open::get_recommended_songs,
            auth::signup::signup,
            auth::login::post::login,
            auth::login::get::logout,
            auth::login::get::username_status,
            protected::creator::submit_product,
            protected::creator::submit_service,
            protected::creator::create_offer,
            protected::creator::marks_avg,
            protected::creator::songs,
            protected::creator::connect_card,
            protected::consumer::status_bar_info,
            protected::user::user_permissions,
            protected::user::avatar_username,
            protected::user::request_obj_storage_upload,
            protected::user::get_conversations,
            protected::user::get_dialog_id,
            protected::user::create_new_conversation,
            protected::user::send_message,
            protected::user::list_conversation,
            development::upload_file,
            development::cleanup,
            development::reset_ban
        ),
        components(
            schemas(
                crate::domain::requests::open_access::Stats,
                crate::domain::requests::open_access::GetSongsListRequest,
                crate::domain::requests::open_access::SongsAmount,
                crate::domain::music_parameters::SortBy,
                crate::domain::music_parameters::Sex,
                crate::domain::music_parameters::MusicKey,
                crate::domain::general_types::ProductStatus,
                crate::domain::user_role::UserRole,
                crate::auth::signup::UserSignupData,
                crate::auth::login::Credentials,
                crate::auth::login::Username,
                crate::routes::development::InputWithFiles,
                GetSongsListResponse,
                GetNewSongsResponse,
                GetRecommendedSongsResponse,
                GetConversationsEntriesResponse,
                GetUserAvatarUsername,
                GetCreatorMarksAvg,
                Products,
                Password,
                DialogId,
                crate::service_providers::object_storage::presigned_post_form::PresignedPostObject,
                MediaType,
                SendMessageRequest,
                ObjectKey,
                Musickey,
                crate::domain::responses::user_access::Entry,
                crate::domain::responses::user_access::Interlocutor,
                crate::domain::responses::user_access::Message,
                crate::domain::responses::user_access::Offer,
                crate::domain::responses::user_access::Attachment,
                crate::domain::responses::user_access::ServiceData,
                crate::domain::requests::creator_access::Lyric,
                crate::domain::requests::creator_access::Product,
                crate::domain::requests::creator_access::MusicProduct,
                crate::domain::requests::creator_access::SubmitProductRequest,
                crate::domain::requests::creator_access::Service,
                crate::domain::requests::creator_access::MusicService,
                crate::domain::requests::creator_access::SubmitServiceRequest,
                crate::domain::requests::creator_access::CreateOfferRequest
            ),
            responses(
                InternalErrorResponse,
                BadRequestResponse,
                NotAcceptableErrorResponse,
                UnauthorizedErrorResponse,
                TooManyUploadsErrorResponse,
                IsExistsResponse,
                crate::service_providers::object_storage::presigned_post_form::PresignedPostObject,
                Permission,
                NotFoundResponse,
                ConversationDataResponse,
                ConflictErrorResponse,
                GetCreatorSongs
            )
        ),
        modifiers(&ServerAddon),
        modifiers(&SecurityAddon), // TEST IT!
        tags(
            (name = "open", description = "Open routes (no authorization)"),
            (name = "protected.users", description = "Protected routes for all users"),
            (name = "protected.creators", description = "Protected routes for creators"),
            (name = "protected.consumers", description = "Protected routes for consumers"),
            (name = "protected.admins", description = "Protected routes for admins"),
            (name = "development", description = "Routes available only in development mode")
        ),
        info(
            title = "HarmonySphere - OpenAPI 3.0",
            version = "0.1.0",
            description = "This is a swagger documentation for harmonysphere backend application.",
        )
    )]
pub(super) struct ApiDoc;

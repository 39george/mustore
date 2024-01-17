use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

use crate::routes::open;

// #[derive(utoipa::ToSchema)]
// #[schema(as = anyhow::Error)]
// struct AnyhowError {
//     message: String,
// }

// #[derive(utoipa::ToSchema)]
// #[schema(as = garde::Report)]
// struct GardeReport {
//     message: String,
// }

#[derive(utoipa::ToSchema)]
#[schema(as = GetSongsListResponse)]
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

#[derive(OpenApi)]
#[openapi(
        paths(
        open::stats,
        open::get_values_list,
        open::get_songs,
        ),
        components(
            schemas(
                crate::routes::open::Stats,
                crate::domain::requests::open_access::GetSongsListRequest,
                crate::domain::music_parameters::SortBy,
                crate::domain::music_parameters::Sex,
                crate::domain::music_parameters::MusicKey,
                GetSongsListResponse,
                // crate::routes::ResponseError,
                // AnyhowError,
                // GardeReport
            )
        ),
        tags(
            (name = "open", description = "Open routes (no authorization)")
        )
    )]
pub(super) struct ApiDoc;

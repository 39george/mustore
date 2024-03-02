use std::time::Duration;

use anyhow::Context;
use axum::extract::Path;
use axum::extract::State;
use axum::routing;
use axum::Form;
use axum::Json;
use axum::Router;
use axum_login::permission_required;
use futures::future::try_join_all;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::consumer_access;
use crate::domain::object_key::ObjectKey;
use crate::domain::requests::consumer_access::AcceptOffer;
use crate::routes::ResponseError;
use crate::startup::api_doc::BadRequestResponse;
use crate::startup::api_doc::InternalErrorResponse;
use crate::startup::AppState;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn consumer_router() -> Router<AppState> {
    Router::new()
        .route("/accept_offer", routing::post(accept_offer))
        .route("/status_bar_info/:which", routing::get(status_bar_info))
        .layer(permission_required!(
            crate::auth::users::Backend,
            "consumer"
        ))
}

/// Get status bar information
#[utoipa::path(
    get,
    path = "/api/protected/consumer/status_bar_info/{which}",
    params(
        ("which" = String, Path,
            description = "'likes', or 'orders'",
            example = "likes"
        )
    ),
    responses(
        (
            status = 200,
            body = Vec<Product>,
            content_type = "application/json",
            description = "Status bar info",
            example = json!(
                [
                  {
                    "product_name": "somesong",
                    "author_username": "someauthor",
                    "price": 123,
                    "product_cover": "someurl",
                  }
                ]
            )
        ),
        (status = 400, response = BadRequestResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 500, response = InternalErrorResponse)
    ),
    security(
        ("api_key" = [])
    ),
    tag = "protected.consumers"
)]
#[tracing::instrument(name = "Get status bar info (likes, orders)", skip_all)]
async fn status_bar_info(
    auth_session: AuthSession,
    Path(path): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<Vec<consumer_access::Products>>, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")
        .map_err(ResponseError::UnexpectedError)?;

    let retrieve_urls = |mut entry: consumer_access::Products| {
        let obj_storage = app_state.object_storage.clone();
        async move {
            let object_key: ObjectKey = entry
                .product_cover
                .parse()
                .context("Failed to parse object key")?;
            let result = obj_storage
                .generate_presigned_url(&object_key, Duration::from_secs(120)) // 2 minutes expiration
                .await?;
            entry.product_cover = result;
            Ok::<consumer_access::Products, ResponseError>(entry)
        }
    };

    let products = match path.as_str() {
        "likes" => {
            let futures = consumer_access::get_liked_products()
                .bind(&db_client, &user.id)
                .all()
                .await
                .context("Failed to get liked products for user")?
                .into_iter()
                .map(retrieve_urls);
            try_join_all(futures).await?
        }
        "orders" => {
            let futures = consumer_access::get_product_orders()
                .bind(&db_client, &user.id)
                .all()
                .await
                .context("Failed to get liked products for user")?
                .into_iter()
                .map(retrieve_urls);
            try_join_all(futures).await?
        }
        _ => {
            return Err(ResponseError::BadRequest(anyhow::anyhow!(
                "Only 'likes' or 'orders' allowed"
            )))
        }
    };

    Ok(Json(products))
}

// TODO: implement function
#[tracing::instrument(name = "Accept offer", skip_all, fields(username))]
async fn accept_offer(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Form(AcceptOffer { offer_id: _ }): Form<AcceptOffer>,
) -> Result<StatusCode, ResponseError> {
    let user = auth_session.user.ok_or(ResponseError::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    // Check that this offer is available for that user
    // We should check that user is participant of conversation
    // where that offer was posted.

    let _db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    Ok(StatusCode::OK)
}

use std::time::Duration;

use anyhow::Context;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Redirect;
use axum::routing;
use axum::Form;
use axum::Json;
use axum::Router;
use axum_login::permission_required;
use banksim_api::init_payment::InitPayment;
use banksim_api::init_payment::InitPaymentRequest;
use futures::future::try_join_all;
use http::StatusCode;
use reqwest::Url;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::cornucopia::queries::consumer_access;
use crate::cornucopia::queries::user_access;
use crate::domain::object_key::ObjectKey;
use crate::domain::requests::consumer_access::AcceptOffer;
use crate::domain::sessions::Session;
use crate::payments::kopeck::Kopeck;
use crate::routes::ErrorResponse;
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
) -> Result<Json<Vec<consumer_access::Products>>, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")
        .map_err(ErrorResponse::UnexpectedError)?;

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
            Ok::<consumer_access::Products, ErrorResponse>(entry)
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
            return Err(ErrorResponse::BadRequest(anyhow::anyhow!(
                "Only 'likes' or 'orders' allowed"
            )))
        }
    };

    Ok(Json(products))
}

#[tracing::instrument(name = "Accept offer", skip_all, fields(username))]
async fn accept_offer(
    auth_session: AuthSession,
    State(app_state): State<AppState>,
    Form(AcceptOffer { offer_id }): Form<AcceptOffer>,
) -> Result<Redirect, ErrorResponse> {
    let user = auth_session.user.ok_or(ErrorResponse::UnauthorizedError(
        anyhow::anyhow!("No such user in AuthSession!"),
    ))?;
    tracing::Span::current().record("username", &user.username);

    let db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")?;

    // Check that this offer is available for that user
    let offer_info = user_access::get_offer_info_by_id()
        .bind(&db_client, &offer_id)
        .one()
        .await
        .context("Failed to get conversation id by offer id")?;
    super::check_conversation_access(
        &db_client,
        &user.id,
        &user.username,
        &offer_info.conversations_id,
    )
    .await?;

    let base: Url = app_state.settings.app_base_url.parse().unwrap();
    let success_url = "https://www.google.com".parse().unwrap();
    let fail_url = base.join("/notification_center/bank").unwrap();
    let notification_url = base.join("/notification_center/bank").unwrap();
    let cashbox_pass = &app_state.settings.payments.cashbox_password;
    let request = InitPaymentRequest::new(
        notification_url,
        success_url,
        fail_url,
        Kopeck::from_rub(offer_info.price)
            .context("Failed to parse decimal as kopeck")?
            .raw() as i64,
        cashbox_pass,
        None,
    );
    let response = app_state
        .airactions_c
        .execute(InitPayment, request)
        .await
        .context("Failed to initiate payment for offer")?;

    let (payment_id, payment_url) = match response.status {
        banksim_api::OperationStatus::Success => (
            response.payment_id.unwrap(),
            response.payment_url.unwrap().to_string(),
        ),
        banksim_api::OperationStatus::Fail(e) => {
            return Err(ErrorResponse::InternalError(anyhow::Error::msg(e)));
        }
        banksim_api::OperationStatus::Cancel => todo!(),
    };

    let redis_client = app_state.redis_pool;
    match Session::new(
        redis_client.next_connected(),
        payment_id,
        user.id,
        crate::domain::sessions::Kind::AcceptingOffer { offer_id },
    )
    .await
    {
        Ok(()) => (),
        Err(e) => {
            return Err(ErrorResponse::InternalError(anyhow::anyhow!(
                "Failed to store payment session in redis: {e}"
            ))
            .into())
        }
    }

    tracing::info!("Redirecting to: {payment_url}");
    Ok(Redirect::to(&payment_url))
}

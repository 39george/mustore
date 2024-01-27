use anyhow::Context;
use axum::extract::State;
use axum::routing;
use axum::Form;
use axum::Router;
use axum_login::permission_required;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth::users::AuthSession;
use crate::domain::requests::consumer_access::AcceptOffer;
use crate::routes::ResponseError;
use crate::startup::AppState;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn consumer_router() -> Router<AppState> {
    Router::new()
        .route("/health_check", routing::get(health_check))
        .route("/accept_offer", routing::post(accept_offer))
        .layer(permission_required!(
            crate::auth::users::Backend,
            "consumer"
        ))
}

#[tracing::instrument(name = "Consumer's health check", skip_all)]
async fn health_check() -> StatusCode {
    StatusCode::OK
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

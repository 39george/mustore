use anyhow::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use banksim_api::notifications::Notification;
use banksim_api::notifications::PaymentNotification;
use banksim_api::notifications::TokenNotification;
use banksim_api::session::webhook::Webhook;
use banksim_api::session::webhook::WebhookRequest;
use http::StatusCode;
use uuid::Uuid;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::cornucopia::queries::internal;
use crate::domain::sessions::Kind as SessionKind;
use crate::domain::sessions::Session;
use crate::domain::sessions::SessionError;
use crate::domain::sessions::Status as SessionStatus;
use crate::impl_debug;
use crate::startup::AppState;

use super::ErrorResponse;

#[derive(thiserror::Error)]
pub enum NotificationErrorResponse {
    #[error(transparent)]
    ErrorResponse(#[from] ErrorResponse),
    #[error("Session error")]
    SessionError(#[from] SessionError),
}

impl_debug!(NotificationErrorResponse);

impl IntoResponse for NotificationErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            // TODO: Should we return OK in any error case?
            NotificationErrorResponse::ErrorResponse(e) => e.into_response(),
            NotificationErrorResponse::SessionError(_) => {
                tracing::error!("{:?}", self);
                // Return OK meaning that we handled notification
                StatusCode::OK.into_response()
            }
        }
    }
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn notification_center_router() -> Router<AppState> {
    Router::new().route("/bank", routing::post(bank_notification))
}

#[tracing::instrument(name = "Got notification", skip_all)]
async fn bank_notification(
    State(state): State<AppState>,
    Json(notification): Json<Notification>,
) -> Result<StatusCode, NotificationErrorResponse> {
    match notification {
        Notification::TokenNotification(t) => {
            handle_token_notification(&state, t).await
        }
        Notification::PaymentNotification(p) => {
            handle_payment_notification(&state, p).await
        }
    }
}

// ───── Token notification ───────────────────────────────────────────────── //

async fn handle_token_notification(
    state: &AppState,
    token_notification: TokenNotification,
) -> Result<StatusCode, NotificationErrorResponse> {
    match token_notification {
        TokenNotification::ReadyToConfirm { session_id } => {
            let session = get_session(&state, session_id).await?;
            let _ =
                check_status_and_confirm(&state, session_id, &session).await;
        }
        TokenNotification::Finished {
            card_token,
            session_id,
            status,
        } => {
            handle_finished_token_notification(
                &state, session_id, card_token, status,
            )
            .await?
        }
    }
    Ok(StatusCode::OK)
}

async fn handle_finished_token_notification(
    state: &AppState,
    session_id: Uuid,
    card_token: Option<String>,
    status: banksim_api::OperationStatus,
) -> Result<(), NotificationErrorResponse> {
    let session = get_session(&state, session_id).await?;
    match status {
        banksim_api::OperationStatus::Success => {
            handle_token_registration_success(&state, session, card_token)
                .await?
        }
        banksim_api::OperationStatus::Cancel => {
            get_session(state, session_id).await?.remove().await?;
            tracing::info!(
                "Card token registration operation was cancelled, removing it"
            );
        }
        banksim_api::OperationStatus::Fail(e) => {
            get_session(state, session_id).await?.remove().await?;
            tracing::error!(
                "Card token registration operation failed: {e}, removing it"
            );
        }
    }
    Ok(())
}

async fn handle_token_registration_success(
    state: &AppState,
    session: Session<'_>,
    card_token: Option<String>,
) -> Result<(), NotificationErrorResponse> {
    if session.status().eq(&SessionStatus::Active)
        && session.kind.eq(&SessionKind::CardTokenRegistration)
    {
        let db_client = state
            .pg_pool
            .get()
            .await
            .context("Failed to get connection from postgres pool")
            .map_err(ErrorResponse::UnexpectedError)?;
        internal::insert_card_token()
            .bind(&db_client, &session.user_id(), &card_token.unwrap())
            .await
            .context("Failed to insert card token into pg")
            .map_err(ErrorResponse::UnexpectedError)?;
        tracing::info!("Successfully created card token!");
    } else {
        tracing::error!("Got notification with token registration success, but session status is {:?}, kind is {:?}", session.status(), session.kind);
    }
    Ok(())
}

// ───── Payment notification ─────────────────────────────────────────────── //

async fn handle_payment_notification(
    state: &AppState,
    payment_notification: PaymentNotification,
) -> Result<StatusCode, NotificationErrorResponse> {
    match payment_notification {
        PaymentNotification::ReadyToConfirm { session_id } => {
            let session = get_session(state, session_id).await?;
            let _ =
                check_status_and_confirm(&state, session_id, &session).await;
        }
        PaymentNotification::ReadyToCapture { session_id } => {
            let session = get_session(state, session_id).await?;
            let _ =
                check_status_and_capture(&state, session_id, &session).await;
        }
        PaymentNotification::PaymentFinished { session_id, status } => {
            match status {
                banksim_api::OperationStatus::Success => {
                    let session = get_session(state, session_id).await?;
                    if session.status().eq(&SessionStatus::Active) {
                        match session.kind {
                            SessionKind::AcceptingOffer { offer_id } => {
                                let db_client = state
                                    .pg_pool
                                    .get()
                                    .await
                                    .context(
                                        "Failed to get connection from postgres pool",
                                    )
                                    .map_err(ErrorResponse::UnexpectedError)?;
                                {
                                    internal::update_offer_status_accepted()
                                        .bind(&db_client, &offer_id)
                                        .await
                                        .context("Failed to update offer status to accepted")
                                        .map_err(ErrorResponse::UnexpectedError)?;
                                    internal::create_service_order()
                                        .bind(&db_client, &offer_id)
                                        .await
                                        .context("Failed to create new service order")
                                        .map_err(ErrorResponse::UnexpectedError)?;
                                }
                                tracing::info!(
                                    "Successfully created service order!"
                                );
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        tracing::error!("Got notification with payment finished success, but session status is {:?}", session.status())
                    }
                }
                banksim_api::OperationStatus::Cancel => {
                    tracing::info!("Payment operation was cancelled");
                }
                banksim_api::OperationStatus::Fail(e) => {
                    tracing::error!("Payment operation failed: {e}");
                }
            }
        }
    }
    // Return OK meaning that we handled notification
    Ok(StatusCode::OK)
}

// ───── Helpers ──────────────────────────────────────────────────────────── //

fn get_session(
    state: &AppState,
    session_id: Uuid,
) -> impl futures::Future<Output = Result<Session<'_>, SessionError>> {
    Session::from_redis_by_session_id(
        state.redis_pool.next_connected(),
        session_id,
    )
}

async fn check_status_and_confirm(
    state: &AppState,
    session_id: Uuid,
    session: &Session<'_>,
) -> Result<(), ()> {
    let status = session.status();
    if status.eq(&SessionStatus::Active) {
        match state
            .airactions_c
            .execute(
                Webhook::Confirm,
                WebhookRequest::new(
                    session_id,
                    &state.settings.payments.cashbox_password,
                ),
            )
            .await
        {
            Ok(response) => {
                tracing::info!("Webhook confirmation response: {response:?}");
                return Ok(());
            }
            Err(e) => {
                tracing::error!("Webhook confirmation error: {e}");
            }
        }
    } else {
        tracing::warn!("Got ReadyToConfirm notification on session with status: {status:?}");
    }
    Err(())
}

async fn check_status_and_capture(
    state: &AppState,
    session_id: Uuid,
    session: &Session<'_>,
) -> Result<(), ()> {
    let status = session.status();
    if status.eq(&SessionStatus::Active) {
        match state
            .airactions_c
            .execute(
                Webhook::Capture,
                WebhookRequest::new(
                    session_id,
                    &state.settings.payments.cashbox_password,
                ),
            )
            .await
        {
            Ok(response) => {
                tracing::info!("Capture webhook response: {response:?}");
                return Ok(());
            }
            Err(e) => {
                tracing::error!("Capture webhook error: {e}");
            }
        }
    } else {
        tracing::warn!("Got ReadyToCapture notification on session with status: {status:?}");
    }
    Err(())
}

use anyhow::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use banksim_api::notifications::Notification;
use banksim_api::notifications::TokenNotification;
use banksim_api::session::webhook::Webhook;
use banksim_api::session::webhook::WebhookRequest;
use http::StatusCode;
use uuid::Uuid;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::cornucopia::queries::internal;
use crate::domain::sessions::card_token_registration::CardTokenSession;
use crate::domain::sessions::card_token_registration::CardTokenSessionError;
use crate::domain::sessions::card_token_registration::Status as CTStatus;
use crate::impl_debug;
use crate::startup::AppState;

use super::ErrorResponse;

#[derive(thiserror::Error)]
pub enum NotificationErrorResponse {
    #[error(transparent)]
    ErrorResponse(#[from] ErrorResponse),
    #[error("No upload info in cache")]
    CardTokenSessionError(#[from] CardTokenSessionError),
}

impl_debug!(NotificationErrorResponse);

impl IntoResponse for NotificationErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            NotificationErrorResponse::ErrorResponse(e) => e.into_response(),
            NotificationErrorResponse::CardTokenSessionError(_) => {
                tracing::error!("{:?}", self);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
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
    let get_session = |session_id| {
        CardTokenSession::from_redis_by_session_id(
            state.redis_pool.next_connected(),
            session_id,
        )
    };
    match notification {
        Notification::TokenNotification(t) => match t {
            TokenNotification::ReadyToConfirm { session_id } => {
                let session = get_session(session_id).await?;
                let _ = check_status_and_confirm(&state, session_id, &session)
                    .await;
                // Return OK meaning that we handled notification
                return Ok(StatusCode::OK);
            }
            TokenNotification::Finished {
                card_token,
                session_id,
                status,
            } => {
                match status {
                    banksim_api::OperationStatus::Success => {
                        let session = get_session(session_id).await?;
                        if session.status().eq(&CTStatus::Active) {
                            let db_client = state
                            .pg_pool
                            .get()
                            .await
                            .context(
                                "Failed to get connection from postgres pool",
                            )
                            .map_err(ErrorResponse::UnexpectedError)?;
                            internal::insert_card_token()
                                .bind(
                                    &db_client,
                                    &session.user_id(),
                                    &card_token.unwrap(),
                                )
                                .await
                                .context("Failed to insert card token into pg")
                                .map_err(ErrorResponse::UnexpectedError)?;
                        }
                        tracing::info!("Successfully created card token!");
                    }
                    banksim_api::OperationStatus::Cancel => {
                        tracing::info!(
                            "Card token registration operation was cancelled"
                        );
                    }
                    banksim_api::OperationStatus::Fail(e) => {
                        tracing::error!(
                            "Card token registration operation failed: {e}"
                        );
                    }
                }
                return Ok(StatusCode::OK);
            }
        },
        // Notification::PaymentNotification(p) =>
        // match p {
        //     PaymentNotification::ReadyToConfirm { session_id } => {
        //         let req = WebhookRequest::new(
        //             Webhook::Confirm,
        //             session_id,
        //             &state.settings.payments.cashbox_password,
        //         );
        //         let _response = state
        //             .payments_client
        //             .execute(Webhook::Confirm, req)
        //             .await
        //             .unwrap();
        //     }
        //     PaymentNotification::ReadyToCapture { session_id } => todo!(),
        //     PaymentNotification::PaymentFinished { session_id, status } => {
        //         todo!()
        //     }
        // },
        _ => unimplemented!(),
    }
}

// ───── Helpers ──────────────────────────────────────────────────────────── //

async fn check_status_and_confirm(
    state: &AppState,
    session_id: Uuid,
    session: &CardTokenSession<'_>,
) -> Result<(), ()> {
    let status = session.status();
    if status.eq(&CTStatus::Active) {
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

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

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::domain::sessions::card_token_registration::CardTokenSession;
use crate::domain::sessions::card_token_registration::CardTokenSessionError;
use crate::impl_debug;
use crate::startup::AppState;

use super::ErrorResponse;

#[derive(thiserror::Error)]
pub enum NotificationsErrorResponse {
    #[error(transparent)]
    ErrorResponse(#[from] ErrorResponse),
    #[error("No upload info in cache")]
    CardTokenSessionError(#[from] CardTokenSessionError),
}

impl_debug!(NotificationsErrorResponse);

impl IntoResponse for NotificationsErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            NotificationsErrorResponse::ErrorResponse(e) => e.into_response(),
            NotificationsErrorResponse::CardTokenSessionError(_) => {
                tracing::error!("{:?}", self);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn notification_center_router() -> Router<AppState> {
    Router::new().route("/notification", routing::post(handle_notification))
}

#[tracing::instrument(name = "Got notification", skip_all)]
async fn handle_notification(
    State(state): State<AppState>,
    Json(notification): Json<Notification>,
) -> Result<StatusCode, NotificationsErrorResponse> {
    use crate::domain::sessions::card_token_registration::Status as CTStatus;
    match notification {
        Notification::TokenNotification(t) => match t {
            TokenNotification::ReadyToConfirm { session_id } => {
                let session = CardTokenSession::from_redis_by_session_id(
                    state.redis_pool.next_connected(),
                    session_id,
                )
                .await?;
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
                        Ok(response) => tracing::info!(
                            "Webhook confirmation response: {response:?}"
                        ),
                        Err(e) => {
                            tracing::error!("Webhook confirmation error: {e}")
                        }
                    }
                } else {
                    tracing::warn!("Got ReadyToConfirm notification on session with status: {status:?}");
                }
                return Ok(StatusCode::OK);
            }
            TokenNotification::Finished { .. } => {
                //
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

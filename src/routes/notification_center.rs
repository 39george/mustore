use axum::extract::State;
use axum::routing;
use axum::Json;
use axum::Router;
use banksim_api::notifications::Notification;
use banksim_api::notifications::PaymentNotification;
use banksim_api::notifications::TokenNotification;
use banksim_api::session::webhook::Webhook;
use banksim_api::session::webhook::WebhookRequest;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::startup::AppState;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn notification_center_router() -> Router<AppState> {
    Router::new().route("/notification", routing::post(handle_notification))
}

#[tracing::instrument(name = "Got notification", skip_all)]
async fn handle_notification(
    State(state): State<AppState>,
    Json(notification): Json<Notification>,
) -> StatusCode {
    match notification {
        Notification::PaymentNotification(p) => match p {
            PaymentNotification::ReadyToConfirm { session_id } => {
                let req = WebhookRequest::new(
                    Webhook::Confirm,
                    session_id,
                    &state.settings.payments.cashbox_password,
                );
                let _response = state
                    .payments_client
                    .execute(Webhook::Confirm, req)
                    .await
                    .unwrap();
            }
            PaymentNotification::ReadyToCapture { session_id } => todo!(),
            PaymentNotification::PaymentFinished { session_id, status } => {
                todo!()
            }
        },
        Notification::TokenNotification(t) => match t {
            TokenNotification::ReadyToConfirm { session_id } => todo!(),
            TokenNotification::Finished {
                card_token,
                session_id,
                status,
            } => todo!(),
        },
    }
    StatusCode::OK
}

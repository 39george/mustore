use axum::routing;
use axum::Router;
use http::StatusCode;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::startup::AppState;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn minio_router() -> Router<AppState> {
    Router::new().route("/event", routing::post(event))
}

#[tracing::instrument(name = "Got Minio event", skip_all)]
async fn event(body: String) -> StatusCode {
    println!("{}", body);
    StatusCode::OK
}

use axum::routing;
use axum::Json;
use axum::Router;

use crate::startup::AppState;

pub fn open_router() -> Router<AppState> {
    Router::new()
        .route("/something_open", routing::get(|| async {}))
        .route("/another_open", routing::get(another_open))
}

async fn another_open() -> Result<Json<String>, String> {
    Err(String::from("hehehe"))
}

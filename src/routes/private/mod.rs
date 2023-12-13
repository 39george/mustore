use axum::routing;
use axum::Router;
use axum_login::login_required;

use crate::startup::AppState;

pub fn private_router() -> Router<AppState> {
    Router::new()
        .route("/something_private", routing::get(|| async {}))
        .route("/another_private", routing::get(|| async {}))
        .layer(login_required!(crate::auth::users::UserBackend))
}

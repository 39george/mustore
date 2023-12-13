use axum::response::IntoResponse;

use axum::body::Body;
use hyper::Request;
use hyper::StatusCode;

pub async fn my_handler() -> Result<&'static str, (StatusCode, &'static str)> {
    // Your handler logic here
    Err((StatusCode::INTERNAL_SERVER_ERROR, "Oops!"))
}

pub async fn health_check(_: Request<Body>) -> Result<StatusCode, String> {
    Err("hehehe".to_string())
}

pub async fn get_hello(_: Request<Body>) -> impl IntoResponse {
    (StatusCode::OK, "Hello from rust-backend!")
}

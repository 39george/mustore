use axum::body::Body;
use hyper::Request;
use hyper::StatusCode;

pub async fn health_check(_: Request<Body>) -> StatusCode {
    StatusCode::OK
}

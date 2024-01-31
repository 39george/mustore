use std::net::SocketAddr;

use axum::extract::ConnectInfo;
use hyper::StatusCode;

pub async fn health_check(ConnectInfo(my_connect_info): ConnectInfo<SocketAddr>) -> StatusCode {
    // println!("Connect info: {}", my_connect_info);
    StatusCode::OK
}

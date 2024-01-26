use std::net::SocketAddr;

use axum::body::Body;
use axum::extract::connect_info::Connected;
use axum::extract::ConnectInfo;
use axum::serve::IncomingStream;
use hyper::Request;
use hyper::StatusCode;

pub async fn health_check(
    ConnectInfo(my_connect_info): ConnectInfo<SocketAddr>,
) -> StatusCode {
    println!("Connect info: {}", my_connect_info);
    StatusCode::OK
}

#[derive(Clone, Debug)]
pub struct MyConnectInfo {
    // ...
}

impl Connected<IncomingStream<'_>> for MyConnectInfo {
    fn connect_info(target: IncomingStream<'_>) -> Self {
        MyConnectInfo {
            // ...
        }
    }
}

//! tests/api/upload_song.rs

use crate::helpers::{TestApp, TestUser};
use mustore::{
    config::Settings, domain::requests::creator_access::SubmitSongRequest,
};

#[tokio::test]
async fn send_message_success() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user1 = TestUser::generate_user(String::from("creator"));
    app.register_user(&test_user1).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
}

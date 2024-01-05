//! tests/api/upload_song.rs

use crate::helpers::{TestApp, TestUser};
use mustore::{
    config::Settings, domain::requests::creator_access::SubmitSongRequest,
};

#[tokio::test]
async fn send_message_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 2).await;

    let test_user1 = TestUser::generate_user(String::from("creator"), 0);
    let test_user2 = TestUser::generate_user(String::from("consumer"), 1);
    dbg!(&test_user1);
    dbg!(&test_user2);

    assert_eq!(app.register_user(&test_user1).await.as_u16(), 200);
    assert_eq!(app.register_user(&test_user2).await.as_u16(), 200);
    // let client = reqwest::Client::builder()
    //     .cookie_store(true)
    //     .build()
    //     .unwrap();
}

//! tests/api/signup.rs

use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;

#[tokio::test]
async fn signup_with_correct_data_creates_a_new_candidate() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let test_user = TestUser::generate();
    let response = test_user.store_in_db(&app.address).await.unwrap();
    assert!(response.status().is_success());

    let db_client = app.pool.get().await.unwrap();
    let row = db_client
        .query_one(
            "SELECT * FROM user_candidates
             WHERE username = $1",
            &[&test_user.username],
        )
        .await
        .unwrap();
    assert_eq!(row.get::<&str, &str>("email"), &test_user.email);
}

#[tokio::test]
async fn signup_with_uncorrect_data_rejected() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let test_user = TestUser {
        username: String::from("a"),
        password: String::from("abc"),
        email: String::from("definitely_not_email"),
    };
    let response = test_user.store_in_db(&app.address).await.unwrap();
    assert_eq!(response.status().as_u16(), 400);
}

//! tests/api/login.rs
use mustore::cornucopia::queries::tests;

// ───── Current Crate Imports ────────────────────────────────────────────── //
use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;

#[tokio::test]
async fn signup_and_confirm_email_creates_user() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate();
    let confirmation_link =
        app.reg_user_get_confirmation_link(&test_user).await;
    let response = reqwest::get(confirmation_link.0).await.unwrap();
    assert!(response.status().is_success());
    let pg_client = app.pg_pool.get().await.unwrap();
    let rows = tests::select_user_data_with_avatar_key()
        .bind(&pg_client)
        .all()
        .await
        .unwrap();
    for row in rows.into_iter() {
        println!(
            "username: {}, email: {}, avatar_key: {}",
            row.username, row.email, row.key
        );
    }
}

#[tokio::test]
async fn create_user_and_login() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate();
    let confirmation_link =
        app.reg_user_get_confirmation_link(&test_user).await;
    let response = reqwest::get(confirmation_link.0).await.unwrap();
    assert!(response.status().is_success());

    let response = reqwest::Client::new()
        .post(format!("{}/api/login", app.address))
        .json(&serde_json::json!({
            "username": test_user.username,
            "password": test_user.password
        }))
        .send()
        .await
        .unwrap();
    dbg!(response);
    // FINISH TEST
}

// login with wrong credentials should fail

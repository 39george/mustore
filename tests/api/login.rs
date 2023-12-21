//! tests/api/login.rs
use std::collections::HashSet;

use mustore::cornucopia::queries::{tests, user_auth_queries};

// ───── Current Crate Imports ────────────────────────────────────────────── //
use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;

#[tokio::test]
async fn signup_and_confirm_email_creates_user_with_correct_permissions() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate(String::from("creator"));
    let confirmation_link =
        app.reg_user_get_confirmation_link(&test_user).await;
    let response = reqwest::get(confirmation_link.0).await.unwrap();

    assert!(response.status().is_success());

    let user_data = tests::select_user_data_with_avatar_key()
        .bind(&app.pg_client, &test_user.username)
        .one()
        .await
        .unwrap();

    assert_eq!(user_data.username, test_user.username);
    assert_eq!(user_data.email, test_user.email);

    let permissions = user_auth_queries::get_user_permissions()
        .bind(&app.pg_client, &user_data.id)
        .all()
        .await
        .unwrap();

    assert_eq!(
        permissions.into_iter().collect::<HashSet<_>>(),
        vec![String::from("user"), String::from("creator")]
            .into_iter()
            .collect::<HashSet<_>>()
    );
}

#[tokio::test]
async fn access_to_protected_with_login_is_allowed() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate(String::from("consumer"));
    let confirmation_link =
        app.reg_user_get_confirmation_link(&test_user).await;
    let response = reqwest::get(confirmation_link.0).await.unwrap();
    assert!(response.status().is_success());

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let response = client
        .post(format!("{}/api/login", app.address))
        .json(&serde_json::json!({
            "username": test_user.username,
            "password": test_user.password
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);

    let response = client
        .get(format!("{}/api/protected/health_check", app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn access_to_protected_without_login_is_restricted() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let response = client
        .get(format!("{}/api/protected/health_check", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status().as_u16(), 403);
}

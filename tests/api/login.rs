//! tests/api/login.rs
use std::collections::HashSet;

use mustore::cornucopia::queries::{tests, user_auth_queries};
use reqwest::redirect::Policy;

// ───── Current Crate Imports ────────────────────────────────────────────── //
use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;

#[tokio::test]
async fn signup_and_confirm_email_creates_user_with_correct_permissions() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate_user(String::from("creator"));
    app.register_user(&test_user, 1).await;

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

    let test_user = TestUser::generate_user(String::from("consumer"));
    let status_code = app.register_user(&test_user, 1).await;
    assert_eq!(status_code.as_u16(), 200);

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let status_code = app.login_user(&test_user, &client).await;
    assert_eq!(status_code.as_u16(), 200);

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

#[tokio::test]
async fn access_to_admin_with_permission_is_given_and_token_is_used() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let admin_token = uuid::Uuid::new_v4();
    let test_user = TestUser::generate_admin(admin_token);
    assert_eq!(
        1,
        user_auth_queries::insert_a_new_admin_signup_token()
            .bind(&app.pg_client, &admin_token)
            .await
            .unwrap()
    );

    let status_code = app.register_user(&test_user, 1).await;
    assert_eq!(status_code.as_u16(), 200);

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let status_code = app.login_user(&test_user, &client).await;
    assert_eq!(status_code.as_u16(), 200);

    let response = client
        .get(format!("{}/api/protected/admin/health_check", app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);

    let token_info = user_auth_queries::get_admin_token()
        .bind(&app.pg_client, &admin_token)
        .one()
        .await
        .unwrap();

    // Assert that token is used
    assert!(token_info.used);
}

#[tokio::test]
async fn cant_register_admin_with_used_token() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let admin_token = uuid::Uuid::new_v4();

    assert_eq!(
        1,
        user_auth_queries::insert_a_new_admin_signup_token()
            .bind(&app.pg_client, &admin_token)
            .await
            .unwrap()
    );

    let test_user = TestUser::generate_admin(admin_token);
    let status_code = app.register_user(&test_user, 2).await;
    assert_eq!(status_code, 200);

    let new_test_user = TestUser::generate_admin(admin_token);

    let confirmation_link = {
        let response = new_test_user.post_signup(&app.address).await.unwrap();
        assert!(response.status().is_success());
        // WARNING: get 2 request with [1]!
        let request = &app.email_server.received_requests().await.unwrap()[1];
        app.get_confirmation_link_urlencoded(request)
    };

    let response = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap()
        .get(confirmation_link.0)
        .send()
        .await
        .unwrap();
    let status_code = response.status();
    assert_eq!(status_code.as_u16(), 303);
}

#[tokio::test]
async fn access_to_admin_api_from_non_admin_account_is_restricted() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate_user(String::from("consumer"));
    let status_code = app.register_user(&test_user, 1).await;
    assert_eq!(status_code.as_u16(), 200);

    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let status_code = app.login_user(&test_user, &client).await;
    assert_eq!(status_code.as_u16(), 200);

    let response = client
        .get(format!("{}/api/protected/admin/health_check", app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 403);
}

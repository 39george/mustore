//! tests/api/login.rs
use std::collections::HashSet;

use mustore::cornucopia::queries::{tests, user_auth_queries};

// ───── Current Crate Imports ────────────────────────────────────────────── //
use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;

#[tokio::test]
async fn signup_and_confirm_email_creates_user_with_correct_permissions() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate_user(String::from("creator"));
    app.register_user(&test_user).await;

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
    let status_code = app.register_user(&test_user).await;
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
async fn access_to_admin_with_permission_is_given() {
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

    let status_code = app.register_user(&test_user).await;
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
}

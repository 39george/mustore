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
async fn access_to_protected_with_login_is_allowed() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate();
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
    let cookie = response.cookies().collect::<Vec<_>>();
    dbg!(cookie);

    let response = client
        .get(format!(
            "{}/api/protected/health_check_protected",
            app.address
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn acces_to_protected_without_login_is_restricted() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    let response = client
        .get(format!(
            "{}/api/protected/health_check_protected",
            app.address
        ))
        .send()
        .await
        .unwrap();
    dbg!(&response);
    assert_eq!(response.status().as_u16(), 401);
}

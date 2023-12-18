//! tests/api/login.rs
use wiremock::matchers::AnyMatcher;
use wiremock::Mock;
use wiremock::ResponseTemplate;

// ───── Current Crate Imports ────────────────────────────────────────────── //
use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;

#[tokio::test]
async fn signup_and_login_creates_user() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let confirmation_link = app
        .reg_user_get_confirmation_link(TestUser::generate())
        .await;
    let response = reqwest::get(confirmation_link.0).await.unwrap();
    assert!(response.status().is_success());
    let pg_client = app.pg_pool.get().await.unwrap();
    // FIXME: add join with avatar
    let rows = pg_client.query("SELECT * FROM users", &[]).await.unwrap();
    for row in rows.into_iter() {
        let username: &str = row.get("username");
        let email: &str = row.get("email");
        println!("username: {}, email: {}", username, email);
    }
}

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
}

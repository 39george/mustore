//! tests/api/login.rs

use crate::helpers::ConfirmationLink;
use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;
use wiremock::matchers::{self, AnyMatcher};
use wiremock::Mock;
use wiremock::ResponseTemplate;

#[tokio::test]
async fn signup_and_login_creates_user() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    Mock::given(AnyMatcher)
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.object_storage_server)
        .await;

    let confirmation_link = app
        .reg_user_get_confirmation_link(TestUser::generate())
        .await;
    let response = reqwest::get(confirmation_link.0).await.unwrap();
    assert!(response.status().is_success());

    // Check that user exists
}

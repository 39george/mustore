//! tests/api/signup.rs

use crate::helpers::ConfirmationLink;
use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;
use wiremock::matchers;
use wiremock::Mock;
use wiremock::ResponseTemplate;

#[tokio::test]
async fn signup_with_correct_data_creates_a_new_candidate() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let test_user = TestUser::generate();

    Mock::given(matchers::path("/v1/smtp/send"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = test_user.post_signup(&app.address).await.unwrap();
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
    let response = test_user.post_signup(&app.address).await.unwrap();
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn signup_with_correct_data_sends_confirmation_email_with_link_smtpbz() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let _confirmation_link =
        reg_user_get_confirmation_link(TestUser::generate(), &app).await;
}

#[tokio::test]
async fn going_by_confirmation_link_confirmes_candidate_account() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let confirmation_link =
        reg_user_get_confirmation_link(TestUser::generate(), &app).await;
    dbg!(&confirmation_link);
    let response = reqwest::get(confirmation_link.0).await.unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

// ───── Helpers ──────────────────────────────────────────────────────────── //

async fn reg_user_get_confirmation_link(
    user: TestUser,
    app: &TestApp,
) -> ConfirmationLink {
    Mock::given(matchers::path("/v1/smtp/send"))
        .and(matchers::method("POST"))
        .and(matchers::header_exists("Authorization"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    let response = user.post_signup(&app.address).await.unwrap();
    assert!(response.status().is_success());

    let request = &app.email_server.received_requests().await.unwrap()[0];

    app.get_confirmation_link_urlencoded(request)
}

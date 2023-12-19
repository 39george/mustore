//! tests/api/signup.rs

use std::collections::HashMap;

use crate::helpers::{TestApp, TestUser};
use fred::interfaces::HashesInterface;
use mustore::config::Settings;
use mustore::domain::user_candidate::UserCandidate;
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

    let key = format!("user_candidate:{}", test_user.email);
    let candidate: HashMap<String, String> =
        app.redis_client.hgetall(&key).await.unwrap();

    let candidate = UserCandidate::try_from(candidate).unwrap();

    assert_eq!(&candidate.email, &test_user.email);
    assert_eq!(&candidate.username, &test_user.username);
}

#[tokio::test]
async fn signup_with_uncorrect_data_rejected() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let test_user = TestUser {
        username: String::from("a"),
        password: String::from("abc"),
        email: String::from("definitely_not_email"),
        role: "consumer".to_string(),
    };
    let response = test_user.post_signup(&app.address).await.unwrap();
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn signup_with_correct_data_sends_confirmation_email_with_link_smtpbz() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let _confirmation_link = app
        .reg_user_get_confirmation_link(TestUser::generate())
        .await;
}

#[tokio::test]
async fn going_by_confirmation_link_confirmes_candidate_account() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;
    let confirmation_link = app
        .reg_user_get_confirmation_link(TestUser::generate())
        .await;
    let response = reqwest::get(confirmation_link.0).await.unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

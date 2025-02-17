//! tests/api/signup.rs

use std::collections::HashMap;

use crate::helpers::{TestApp, TestUser};
use fred::interfaces::HashesInterface;
use mustore::config::Settings;
use mustore::domain::user_candidate::UserCandidate;
use reqwest::redirect::Policy;

#[tokio::test]
async fn signup_with_correct_data_creates_a_new_candidate() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("consumer"), 0);

    let response = test_user.post_signup(&app.address).await.unwrap();
    assert!(response.status().is_success());

    let key = UserCandidate::key_from_email(&test_user.email);
    let candidate: HashMap<String, String> =
        app.redis_client.hgetall(&key).await.unwrap();

    let candidate = UserCandidate::try_from(candidate).unwrap();

    assert_eq!(&candidate.email, &test_user.email);
    assert_eq!(&candidate.username, &test_user.username);
}

#[tokio::test]
async fn signup_with_uncorrect_data_rejected() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 0).await;
    let test_user = TestUser {
        username: String::from("a"),
        password: String::from("abc"),
        email: String::from("definitely_not_email"),
        role: Some("consumer".to_string()),
        admin_token: None,
        idx: 0,
    };
    let response = test_user.post_signup(&app.address).await.unwrap();
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn signup_with_correct_data_sends_confirmation_email_with_link_smtpbz() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;
    let test_user = TestUser::generate_user(String::from("creator"), 0);
    let _confirmation_link =
        app.signup_user_get_confirmation_link(&test_user).await;
}

#[tokio::test]
async fn going_by_confirmation_link_confirmes_candidate_account() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;
    let test_user = TestUser::generate_user(String::from("consumer"), 0);
    let confirmation_link =
        app.signup_user_get_confirmation_link(&test_user).await;
    let response = reqwest::get(confirmation_link.0).await.unwrap();
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn wrong_confirmation_link_should_redirect_to_specific_route() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 0).await;
    let confirmation_link = format!(
        "{}/api/confirm_user_account?email=unexistent@shouldfail.net&token=26mxZMNiMnErAUd2hONzgymaw",
        app.address);

    let response = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()
        .unwrap()
        .get(confirmation_link)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(
        response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap(),
        "react-router/accountconfirmationfailed"
    );
}

//! tests/api/upload_song.rs

use crate::helpers::{TestApp, TestUser};
use mustore::config::Settings;

#[tokio::test]
async fn register_card_token_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    // Register users
    let creator = TestUser::generate_user(String::from("creator"), 0);
    assert_eq!(app.register_user(&creator).await.as_u16(), 200);

    // Login http clients on server
    let creator_client = reqwest::Client::builder()
        .cookie_store(true)
        // Don't follow redirects
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    assert_eq!(
        app.login_user(&creator, &creator_client).await.as_u16(),
        200
    );

    let bank_acc = app.banksim.create_account("abc").await.unwrap();

    // Start card registration process
    let response = creator_client
        .post(format!(
            "{}/api/protected/creator/connect_card",
            app.address
        ))
        .send()
        .await
        .unwrap();
    let url = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap();

    // Connect to webdriver
    let webdriver = fantoccini::ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("failed to connect to WebDriver");

    // Open url in web interface
    webdriver.goto(url).await.unwrap();

    // Input card number into form
    let input = webdriver
        .wait()
        .for_element(fantoccini::Locator::Id("card_number"))
        .await
        .unwrap();
    input.send_keys(&bank_acc.card_number).await.unwrap();

    // Press `submit` button
    webdriver
        .find(fantoccini::Locator::Id("submit_button"))
        .await
        .unwrap()
        .click()
        .await
        .unwrap();

    // Wait until success redirect
    webdriver
        .wait()
        .for_url("https://www.google.com".parse().unwrap())
        .await
        .unwrap();

    // Close webdriver session
    webdriver.close().await.unwrap();

    let tokens = app
        .banksim
        .tokens_for_card_number(bank_acc.card_number)
        .await
        .unwrap();

    assert_eq!(tokens.len(), 1);
}

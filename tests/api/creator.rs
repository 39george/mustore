//! tests/api/upload_song.rs

use crate::helpers::{
    creator::new_mixing_service_for_creator, TestApp, TestUser, WEBDRIVER_LOCK,
};
use mustore::{
    config::Settings,
    domain::{
        requests::creator_access::CreateOfferRequest,
        responses::user_access::DialogId,
    },
};

#[tokio::test]
async fn register_card_token_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    // Register users
    let creator = TestUser::generate_user(String::from("creator"), 0);
    assert_eq!(app.register_user(&creator).await.as_u16(), 200);

    // TODO: implement logic to use self-signed certificate only in development
    // also in startup
    let cert = include_bytes!("/home/ghashy/.local/share/mkcert/rootCA.pem");
    let creator_client = reqwest::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(cert).unwrap())
        .cookie_store(true)
        // Don't follow redirects
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    // Login http client on server
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

    let _lock = WEBDRIVER_LOCK.lock();

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

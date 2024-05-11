//! tests/api/upload_song.rs

use mustore::config::Settings;
use mustore::domain::requests::consumer_access::AcceptOffer;
use mustore::domain::{
    requests::creator_access::CreateOfferRequest,
    responses::user_access::DialogId,
};
use mustore::payments::kopeck::Kopeck;
use reqwest::redirect::Policy;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use time::OffsetDateTime;

use crate::helpers::creator::new_mixing_service_for_creator;
use crate::helpers::{extract_days_count, TestApp, TestUser, WEBDRIVER_LOCK};

#[tokio::test]
async fn accept_offer_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 2).await;
    // Register users
    let creator = TestUser::generate_user(String::from("creator"), 0);
    let consumer = TestUser::generate_user(String::from("consumer"), 1);
    assert_eq!(app.register_user(&creator).await.as_u16(), 200);
    assert_eq!(app.register_user(&consumer).await.as_u16(), 200);

    let creator_client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let consumer_client = reqwest::Client::builder()
        .cookie_store(true)
        .redirect(Policy::none())
        .build()
        .unwrap();

    // Login http clients on server
    assert_eq!(
        app.login_user(&creator, &creator_client).await.as_u16(),
        200
    );
    assert_eq!(
        app.login_user(&consumer, &consumer_client).await.as_u16(),
        200
    );

    // Initiate dialog
    let response = consumer_client
        .post(format!(
            "{}/api/protected/user/new_conversation",
            app.address
        ))
        .query(&[("with_username", &creator.username)])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);

    let dialog: DialogId = response.json().await.unwrap();

    // Create mixing service for creator
    new_mixing_service_for_creator(&app, &creator_client, 100).await;

    let service_id: i32 = app
        .pg_client
        .query(
            "SELECT id
             FROM services
             LIMIT 1",
            &[],
        )
        .await
        .unwrap()[0]
        .get("id");

    // Create offer
    let response = creator_client
        .post(format!(
            "{}/api/protected/creator/create_offer",
            app.address
        ))
        .json(&CreateOfferRequest {
            conversation_id: dialog.id,
            consumer_id: consumer.idx as i32 + 1,
            service_id,
            text: "New offer".to_string(),
            price: Decimal::from_i32(100).unwrap(),
            delivery_days: 3,
            free_revisions: 3,
            revision_price: Decimal::from_i32(10).unwrap(),
        })
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);

    let offer_id: i32 = app
        .pg_client
        .query(
            "SELECT id
             FROM offers
             LIMIT 1",
            &[],
        )
        .await
        .unwrap()[0]
        .get("id");

    // Start accepting offer session
    let response = consumer_client
        .post(format!(
            "{}/api/protected/consumer/accept_offer",
            app.address
        ))
        .form(&AcceptOffer { offer_id })
        .send()
        .await
        .unwrap();

    // Get redirection url
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

    // Create bank account
    let bank_acc = app.banksim.create_account("password").await.unwrap();
    app.banksim
        .open_credit(
            &bank_acc,
            Kopeck::from_rub_str("1000.00").unwrap().raw() as i64,
        )
        .await
        .unwrap();

    // Fill form
    let input = webdriver
        .wait()
        .for_element(fantoccini::Locator::Id("card_number"))
        .await
        .unwrap();
    input.send_keys(&bank_acc.card_number).await.unwrap();
    let input = webdriver
        .wait()
        .for_element(fantoccini::Locator::Id("password"))
        .await
        .unwrap();
    input.send_keys(&bank_acc.password).await.unwrap();

    // Submit
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

    // Check that offer is active and there is service order created
    let offer = &app
        .pg_client
        .query(
            "SELECT
                id,
                conversations_id,
                creator_id,
                consumer_id,
                services_id,
                text,
                price,
                delivery_interval::TEXT,
                free_revisions,
                revision_price,
                status::TEXT
             FROM offers
             LIMIT 1",
            &[],
        )
        .await
        .unwrap()[0];
    assert_eq!(offer.get::<&str, i32>("conversations_id"), 1);
    assert_eq!(offer.get::<&str, i32>("creator_id"), creator.idx as i32 + 1);
    assert_eq!(
        offer.get::<&str, i32>("consumer_id"),
        consumer.idx as i32 + 1
    );
    assert_eq!(offer.get::<&str, i32>("services_id"), service_id);
    assert_eq!(offer.get::<&str, &str>("text"), "New offer");
    assert_eq!(
        offer.get::<&str, Decimal>("price"),
        Decimal::from_i32(100).unwrap()
    );
    assert_eq!(
        extract_days_count(offer.get::<&str, &str>("delivery_interval")),
        Some(3)
    );
    assert_eq!(offer.get::<&str, i32>("free_revisions"), 3);
    assert_eq!(
        offer.get::<&str, Decimal>("revision_price"),
        Decimal::from_i32(10).unwrap()
    );
    assert_eq!(offer.get::<&str, &str>("status"), "accepted");

    let service_order = &app
        .pg_client
        .query(
            "SELECT
                id,
                created_at,
                offers_id,
                delivery_date,
                free_revisions_left,
                paid_revisions_made,
                status::TEXT
             FROM service_orders
             LIMIT 1",
            &[],
        )
        .await
        .unwrap()[0];

    assert_eq!(
        service_order.get::<&str, i32>("offers_id"),
        offer.get::<&str, i32>("id")
    );
    assert_eq!(
        service_order.get::<&str, OffsetDateTime>("delivery_date"),
        service_order.get::<&str, OffsetDateTime>("created_at")
            + time::Duration::days(3)
    );
    assert_eq!(
        service_order.get::<&str, i32>("free_revisions_left"),
        offer.get::<&str, i32>("free_revisions")
    );
    assert_eq!(service_order.get::<&str, i32>("paid_revisions_made"), 0);
    assert_eq!(service_order.get::<&str, &str>("status"), "paid");
}

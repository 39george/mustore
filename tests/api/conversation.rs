//! tests/api/upload_song.rs

use crate::helpers::{TestApp, TestUser};
use mustore::{
    config::Settings,
    domain::{
        requests::user_access::SendMessageRequest,
        responses::user_access::ConversationDataResponse,
    },
};

#[tokio::test]
async fn send_messages_in_two_directions_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 2).await;

    // Register users
    let user1 = TestUser::generate_user(String::from("creator"), 0);
    let user2 = TestUser::generate_user(String::from("consumer"), 1);
    assert_eq!(app.register_user(&user1).await.as_u16(), 200);
    assert_eq!(app.register_user(&user2).await.as_u16(), 200);

    // Get users id
    let user1_id: i32 = app
        .pg_client
        .query_one(
            "SELECT id FROM users WHERE username = $1",
            &[&user1.username],
        )
        .await
        .unwrap()
        .get(0);
    let user2_id: i32 = app
        .pg_client
        .query_one(
            "SELECT id FROM users WHERE username = $1",
            &[&user2.username],
        )
        .await
        .unwrap()
        .get(0);

    // Login http clients on server
    let client1 = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    let client2 = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&user1, &client1).await.as_u16(), 200);
    assert_eq!(app.login_user(&user2, &client2).await.as_u16(), 200);

    // Initiate dialog
    let response = client2
        .post(format!(
            "{}/api/protected/user/new_conversation",
            app.address
        ))
        .query(&[("with_user_id", user1_id)])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);
    let conversation_id: i32 = response.json().await.unwrap();

    // Send first message
    let response = client2
        .post(format!("{}/api/protected/user/send_message", app.address))
        .json(&SendMessageRequest {
            conversation_id,
            text: "Hello! How are you!".to_string(),
            service_id: None,
            attachments: vec![],
            reply_message_id: None,
        })
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);

    // Send second message
    let response = client1
        .post(format!("{}/api/protected/user/send_message", app.address))
        .json(&SendMessageRequest {
            conversation_id,
            text: "Thanks, I'm fine!".to_string(),
            service_id: None,
            attachments: vec![],
            reply_message_id: None,
        })
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);

    // Retrieve conversation
    let response = client1
        .get(format!(
            "{}/api/protected/user/list_conversation",
            app.address
        ))
        .query(&[("conversation_id", conversation_id), ("offset", 0)])
        .send()
        .await
        .unwrap();

    // Validate response
    let mut response: ConversationDataResponse =
        serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(
        response.interlocutors.get(&user1_id).unwrap().username,
        user1.username
    );
    assert_eq!(
        response.interlocutors.get(&user2_id).unwrap().username,
        user2.username
    );
    assert_eq!(response.entries.len(), 2);
    // Start from end
    let message2 = response.entries.pop().unwrap().message();
    let message1 = response.entries.pop().unwrap().message();
    assert_eq!(message2.text, "Thanks, I'm fine!");
    assert_eq!(message1.text, "Hello! How are you!");
    assert_eq!(message1.interlocutor_id, user2_id);
    assert_eq!(message2.interlocutor_id, user1_id);
    assert_eq!(message1.service, None);
    assert_eq!(message1.reply_message_id, None);
    assert_eq!(message1.attachments, None);
    assert_eq!(message2.service, None);
    assert_eq!(message2.reply_message_id, None);
    assert_eq!(message2.attachments, None);
}

// #[tokio::test]
// async fn send_offer_check() {
//     let app =
//         TestApp::spawn_app(Settings::load_configuration().unwrap(), 2).await;

//     // Register users
//     let creator = TestUser::generate_user(String::from("creator"), 0);
//     let consumer = TestUser::generate_user(String::from("consumer"), 1);
//     assert_eq!(app.register_user(&creator).await.as_u16(), 200);
//     assert_eq!(app.register_user(&consumer).await.as_u16(), 200);

//     // Get users id
//     let creator_id: i32 = app
//         .pg_client
//         .query_one(
//             "SELECT id FROM users WHERE username = $1",
//             &[&creator.username],
//         )
//         .await
//         .unwrap()
//         .get(0);
//     let consumer_id: i32 = app
//         .pg_client
//         .query_one(
//             "SELECT id FROM users WHERE username = $1",
//             &[&consumer.username],
//         )
//         .await
//         .unwrap()
//         .get(0);

//     // Login http clients on server
//     let creator_client = reqwest::Client::builder()
//         .cookie_store(true)
//         .build()
//         .unwrap();
//     let consumer_client = reqwest::Client::builder()
//         .cookie_store(true)
//         .build()
//         .unwrap();
//     assert_eq!(
//         app.login_user(&creator, &creator_client).await.as_u16(),
//         200
//     );
//     assert_eq!(
//         app.login_user(&consumer, &consumer_client).await.as_u16(),
//         200
//     );

//     // Initiate dialog
//     let create_offer_response = creator_client
//         .post(format!(
//             "{}/api/protected/creator/create_offer",
//             app.address
//         ))
//         .json(&CreateOfferRequest {
//             conversation_id: todo!(),
//             service_id: todo!(),
//             text: todo!(),
//             price: todo!(),
//             delivery_date: todo!(),
//             free_revisions: todo!(),
//             revision_price: todo!(),
//         })
//         .send()
//         .await
//         .unwrap();
// assert_eq!(response.status().as_u16(), 201);
// let conversation_id: i32 = response.json().await.unwrap();

// // Send first message
// let response = consumer_client
//     .post(format!("{}/api/protected/user/send_message", app.address))
//     .json(&SendMessageRequest {
//         conversation_id,
//         text: "Hello! How are you!".to_string(),
//         service_id: None,
//         attachments: vec![],
//         reply_message_id: None,
//     })
//     .send()
//     .await
//     .unwrap();
// assert_eq!(response.status().as_u16(), 201);

// // Send second message
// let response = creator_client
//     .post(format!("{}/api/protected/user/send_message", app.address))
//     .json(&SendMessageRequest {
//         conversation_id,
//         text: "Thanks, I'm fine!".to_string(),
//         service_id: None,
//         attachments: vec![],
//         reply_message_id: None,
//     })
//     .send()
//     .await
//     .unwrap();
// assert_eq!(response.status().as_u16(), 201);

// // Retrieve conversation
// let response = creator_client
//     .get(format!(
//         "{}/api/protected/user/list_conversation",
//         app.address
//     ))
//     .query(&[("conversation_id", conversation_id), ("offset", 0)])
//     .send()
//     .await
//     .unwrap();

// // Validate response
// let mut response: ConversationDataResponse =
//     serde_json::from_str(&response.text().await.unwrap()).unwrap();
// assert_eq!(
//     response.interlocutors.get(&creator_id).unwrap().username,
//     creator.username
// );
// assert_eq!(
//     response.interlocutors.get(&consumer_id).unwrap().username,
//     consumer.username
// );
// assert_eq!(response.entries.len(), 2);
// // Start from end
// let message2 = response.entries.pop().unwrap().message();
// let message1 = response.entries.pop().unwrap().message();
// assert_eq!(message2.text, "Thanks, I'm fine!");
// assert_eq!(message1.text, "Hello! How are you!");
// assert_eq!(message1.interlocutor_id, consumer_id);
// assert_eq!(message2.interlocutor_id, creator_id);
// assert_eq!(message1.service, None);
// assert_eq!(message1.reply_message_id, None);
// assert_eq!(message1.attachments, None);
// assert_eq!(message2.service, None);
// assert_eq!(message2.reply_message_id, None);
// assert_eq!(message2.attachments, None);
// }

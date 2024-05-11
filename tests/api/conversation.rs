//! tests/api/upload_song.rs

use crate::helpers::{TestApp, TestUser};
use mustore::{
    config::Settings,
    domain::{
        requests::user_access::SendMessageRequest,
        responses::user_access::{ConversationDataResponse, DialogId},
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
        .query(&[("with_username", &user1.username)])
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);
    let dialog: DialogId = response.json().await.unwrap();

    // Send first message
    let response = client2
        .post(format!("{}/api/protected/user/send_message", app.address))
        .json(&SendMessageRequest {
            conversation_id: dialog.id,
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
            conversation_id: dialog.id,
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
        .query(&[("conversation_id", dialog.id), ("offset", 0)])
        .send()
        .await
        .unwrap();

    // Validate response
    let mut response: ConversationDataResponse =
        serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(
        response
            .interlocutors
            .iter()
            .find(|&i| i.username.eq(&user1.username))
            .map(|i| &i.username),
        Some(&user1.username)
    );
    assert_eq!(
        response
            .interlocutors
            .iter()
            .find(|&i| i.username.eq(&user2.username))
            .map(|i| &i.username),
        Some(&user2.username)
    );
    assert_eq!(response.entries.len(), 2);

    // Start from end
    let message2 = response.entries.pop().unwrap().message();
    let message1 = response.entries.pop().unwrap().message();
    assert_eq!(message2.text, "Thanks, I'm fine!");
    assert_eq!(message1.text, "Hello! How are you!");
    assert_eq!(message1.service, None);
    assert_eq!(message1.reply_message_id, None);
    assert_eq!(message1.attachments, None);
    assert_eq!(message2.service, None);
    assert_eq!(message2.reply_message_id, None);
    assert_eq!(message2.attachments, None);
}

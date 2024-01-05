//! Health integration test.
//!
//! IMPORTANT: Runnig PostgreSQL server is required for testing!

use mustore::config::Settings;

use crate::helpers::TestApp;

#[tokio::test]
async fn health_check_test() {
    let config = Settings::load_configuration().unwrap();

    let app = TestApp::spawn_app(config, 0).await;

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/api/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

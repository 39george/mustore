//! tests/api/upload_song.rs

use crate::helpers::{TestApp, TestUser};
use mustore::{
    config::Settings, domain::requests::creator_access::SubmitSongRequest,
};

#[tokio::test]
async fn song_uploading_success() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate_user(String::from("creator"));
    app.register_user(&test_user, 1).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let song_file = std::fs::read("assets/song.mp3").unwrap();
    let image_file = std::fs::read("assets/image.png").unwrap();
    let arch_file = std::fs::read("assets/arch.zip").unwrap();

    let (response, song_key) = app
        .upload_file(&client, "audio/mpeg", "song.mp3", song_file)
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file)
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let (response, arch_key) = app
        .upload_file(&client, "application/zip", "arch.zip", arch_file)
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let body = SubmitSongRequest {
        song_master_object_key: song_key,
        song_master_tagged_object_key: None,
        song_multitrack_object_key: arch_key,
        song_cover_object_key: image_key,
        name: "some_song".to_string(),
        description: None,
        tags: vec!["calm".to_string()],
        primary_genre: "pop".to_string(),
        secondary_genre: None,
        tempo: 100,
        duration: 30,
        lyric: "this is song's lyric. Is it long enough or not?".to_string(),
        price: 100.into(),
        sex: mustore::domain::music_parameters::Sex::Female,
        key: mustore::domain::music_parameters::MusicKey::a_major,
    };

    let response = client
        .post(format!("{}/api/protected/creator/submit_song", app.address))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn song_uploading_without_files_fails() {
    let app = TestApp::spawn_app(Settings::load_configuration().unwrap()).await;

    let test_user = TestUser::generate_user(String::from("creator"));
    app.register_user(&test_user, 1).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let body = SubmitSongRequest {
        song_master_object_key: "some_keyyyyyyyyyyyy".to_string(),
        song_master_tagged_object_key: None,
        song_multitrack_object_key: "some_keyyyyyyyyyyyy".to_string(),
        song_cover_object_key: "some_keyyyyyyyyyyyy".to_string(),
        name: "some_song".to_string(),
        description: None,
        tags: vec!["calm".to_string()],
        primary_genre: "pop".to_string(),
        secondary_genre: None,
        tempo: 100,
        duration: 30,
        lyric: "this is song's lyric. Is it long enough or not?".to_string(),
        price: 100.into(),
        sex: mustore::domain::music_parameters::Sex::Female,
        key: mustore::domain::music_parameters::MusicKey::a_major,
    };

    let response = client
        .post(format!("{}/api/protected/creator/submit_song", app.address))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 417);
}

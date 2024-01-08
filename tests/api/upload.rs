//! tests/api/upload_song.rs

use crate::helpers::{TestApp, TestUser};
use mustore::{
    config::Settings,
    domain::requests::creator_access::{
        MusicProduct, Product, SubmitProductRequest,
    },
};

#[tokio::test]
async fn song_uploading_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
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

    let body = SubmitProductRequest::Song {
        product: Product {
            name: "some_song".to_string(),
            description: None,
            moods: vec!["веселый".to_string()],
            cover_object_key: image_key.into(),
            price: 100.into(),
        },
        music_product: MusicProduct {
            master_object_key: song_key.into(),
            master_tagged_object_key: None,
            multitrack_object_key: arch_key.into(),
            primary_genre: "Хор".to_string(),
            secondary_genre: None,
            tempo: 100,
            duration: 30,
            key: mustore::domain::music_parameters::MusicKey::a_major,
        },
        lyric: "this is song's lyric. Is it long enough or not?".into(),
        sex: mustore::domain::music_parameters::Sex::Female,
    };

    let response = client
        .post(format!(
            "{}/api/protected/creator/submit_product",
            app.address
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn song_uploading_without_files_fails() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let body = SubmitProductRequest::Song {
        product: Product {
            name: "some_song".to_string(),
            description: None,
            moods: vec!["веселый".to_string()],
            cover_object_key: "some_keyyyyyyyyyyyy".into(),
            price: 100.into(),
        },
        music_product: MusicProduct {
            master_object_key: "some_keyyyyyyyyyyyy".into(),
            master_tagged_object_key: None,
            multitrack_object_key: "some_keyyyyyyyyyyyy".into(),
            primary_genre: "Хор".to_string(),
            secondary_genre: None,
            tempo: 100,
            duration: 30,
            key: mustore::domain::music_parameters::MusicKey::a_major,
        },
        lyric: "this is song's lyric. Is it long enough or not?".into(),
        sex: mustore::domain::music_parameters::Sex::Female,
    };

    let response = client
        .post(format!(
            "{}/api/protected/creator/submit_product",
            app.address
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 417);
}

#[tokio::test]
async fn beat_uploading_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let beat_file = std::fs::read("assets/song.mp3").unwrap();
    let image_file = std::fs::read("assets/image.png").unwrap();
    let arch_file = std::fs::read("assets/arch.zip").unwrap();

    let (response, beat_key) = app
        .upload_file(&client, "audio/mpeg", "song.mp3", beat_file)
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

    let body = SubmitProductRequest::Song {
        product: Product {
            name: "some_song".to_string(),
            description: None,
            moods: vec!["веселый".to_string()],
            cover_object_key: image_key.into(),
            price: 100.into(),
        },
        music_product: MusicProduct {
            master_object_key: beat_key.into(),
            master_tagged_object_key: None,
            multitrack_object_key: arch_key.into(),
            primary_genre: "Хор".to_string(),
            secondary_genre: None,
            tempo: 100,
            duration: 30,
            key: mustore::domain::music_parameters::MusicKey::a_major,
        },
        lyric: "this is song's lyric. Is it long enough or not?".into(),
        sex: mustore::domain::music_parameters::Sex::Female,
    };

    let js = serde_json::to_string_pretty(&body).unwrap();
    println!("{js}");

    let response = client
        .post(format!(
            "{}/api/protected/creator/submit_product",
            app.address
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);
}

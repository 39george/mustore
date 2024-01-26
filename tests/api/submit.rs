//! tests/api/upload_song.rs

use std::ops::RangeBounds;

use crate::helpers::{TestApp, TestUser};
use mustore::{
    config::Settings,
    cornucopia::queries::open_access,
    domain::requests::creator_access::{
        MusicProduct, MusicService, Product, SubmitProductRequest,
        SubmitServiceRequest,
    },
};
use rand::Rng;

#[tokio::test]
async fn song_submit_success() {
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
            cover_object_key: image_key.parse().unwrap(),
            price: 100.into(),
        },
        music_product: MusicProduct {
            master_object_key: song_key.parse().unwrap(),
            master_tagged_object_key: None,
            multitrack_object_key: arch_key.parse().unwrap(),
            primary_genre: "хор".to_string(),
            secondary_genre: None,
            tempo: 100,
            duration: 30,
            music_key: mustore::domain::music_parameters::MusicKey::a_major,
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
async fn song_submit_without_files_fails() {
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
            cover_object_key:
                "abc/owner:0678418B-D415-4BC0-ADC2-B3E2686DB2F1:file"
                    .parse()
                    .unwrap(),
            price: 100.into(),
        },
        music_product: MusicProduct {
            master_object_key:
                "abc/owner:0678418B-D415-4BC0-ADC2-B3E2686DB2F1:file"
                    .parse()
                    .unwrap(),
            master_tagged_object_key: None,
            multitrack_object_key:
                "abc/owner:0678418B-D415-4BC0-ADC2-B3E2686DB2F1:file"
                    .parse()
                    .unwrap(),
            primary_genre: "хор".to_string(),
            secondary_genre: None,
            tempo: 100,
            duration: 30,
            music_key: mustore::domain::music_parameters::MusicKey::a_major,
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
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn beat_submit_success() {
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

    let body = SubmitProductRequest::Beat {
        product: Product {
            name: "some_song".to_string(),
            description: None,
            moods: vec!["веселый".to_string()],
            cover_object_key: image_key.parse().unwrap(),
            price: 100.into(),
        },
        music_product: MusicProduct {
            master_object_key: beat_key.parse().unwrap(),
            master_tagged_object_key: None,
            multitrack_object_key: arch_key.parse().unwrap(),
            primary_genre: "хор".to_string(),
            secondary_genre: None,
            tempo: 100,
            duration: 30,
            music_key: mustore::domain::music_parameters::MusicKey::a_major,
        },
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
async fn cover_submit_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let image_file = std::fs::read("assets/image.png").unwrap();
    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file)
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let body = SubmitProductRequest::Cover {
        product: Product {
            name: "some_song".to_string(),
            description: None,
            moods: vec!["веселый".to_string()],
            cover_object_key: image_key.parse().unwrap(),
            price: 100.into(),
        },
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
async fn lyric_submit_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let image_file = std::fs::read("assets/image.png").unwrap();
    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file)
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let body = SubmitProductRequest::Lyric {
        product: Product {
            name: "some_song".to_string(),
            description: None,
            moods: vec!["веселый".to_string()],
            cover_object_key: image_key.parse().unwrap(),
            price: 100.into(),
        },
        lyric: "this is just lyric. Is it long enough or not?".into(),
        sex: None,
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
async fn mixing_service_submit_with_credits_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let image_file = std::fs::read("assets/image.png").unwrap();
    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file)
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let mut credits = Vec::new();
    for i in 0..3 {
        let credit_file = std::fs::read("assets/song.mp3").unwrap();
        let (response, credit_key) = app
            .upload_file(
                &client,
                "audio/mpeg",
                &format!("song-{i}.mp3"),
                credit_file,
            )
            .await;
        assert_eq!(response.status().as_u16(), 200);
        credits.push(credit_key.parse().unwrap());
    }

    let genres_list = open_access::get_genres_list()
        .bind(&app.pg_client)
        .all()
        .await
        .unwrap();

    let genres: Vec<_> =
        get_rand_subiter(&genres_list, 0..10).cloned().collect();

    let body = SubmitServiceRequest::Mixing(MusicService {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: Some(credits),
        },
        genres: Some(genres.clone()),
    });

    let response = client
        .post(format!(
            "{}/api/protected/creator/submit_service",
            app.address
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 201);

    let row = app
        .pg_client
        .query_one("SELECT name, description, display_price::INTEGER, status::TEXT FROM services WHERE id = 1", &[])
        .await
        .unwrap();

    assert_eq!(row.get::<&str, &str>("name"), "Some service");
    assert_eq!(row.get::<&str, Option<&str>>("description"), None);
    assert_eq!(row.get::<&str, i32>("display_price"), 500);
    assert_eq!(row.get::<&str, &str>("status"), "moderation");

    let row = app
        .pg_client
        .query_one("SELECT services_id FROM mixing WHERE id = 1", &[])
        .await
        .unwrap();

    assert_eq!(row.get::<&str, i32>("services_id"), 1);

    let service_genres = app
        .pg_client
        .query(
            "
            SELECT genres.name AS genre
            FROM music_services_genres
            JOIN genres ON genres.id = music_services_genres.genres_id
            WHERE mixing_id = 1",
            &[],
        )
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.get::<&str, String>("genre"));

    for genre in service_genres {
        assert!(genres.contains(&genre));
    }
}

#[tokio::test]
async fn submit_all_kinds_of_services_without_credits_success() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let image_file = std::fs::read("assets/image.png").unwrap();

    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file.clone())
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let genres_list = open_access::get_genres_list()
        .bind(&app.pg_client)
        .all()
        .await
        .unwrap();

    let genres: Vec<_> =
        get_rand_subiter(&genres_list, 0..10).cloned().collect();

    let mixing = SubmitServiceRequest::Mixing(MusicService {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: None,
        },
        genres: Some(genres.clone()),
    });

    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file.clone())
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let song_writing = SubmitServiceRequest::SongWriting(MusicService {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: None,
        },
        genres: Some(genres.clone()),
    });

    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file.clone())
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let beat_writing = SubmitServiceRequest::BeatWriting(MusicService {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: None,
        },
        genres: Some(genres.clone()),
    });

    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file.clone())
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let ghost_writing = SubmitServiceRequest::GhostWriting {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: None,
        },
        credits: None,
    };

    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file.clone())
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let cover_design = SubmitServiceRequest::CoverDesign(
        mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: None,
        },
    );

    for body in vec![
        mixing,
        song_writing,
        beat_writing,
        ghost_writing,
        cover_design,
    ]
    .into_iter()
    {
        let response = client
            .post(format!(
                "{}/api/protected/creator/submit_service",
                app.address
            ))
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), 201);
    }
}

#[tokio::test]
async fn submitting_service_too_much_credits_fails() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    let image_file = std::fs::read("assets/image.png").unwrap();

    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file)
        .await;
    assert_eq!(response.status().as_u16(), 200);

    let mut credits = Vec::new();
    for i in 0..4 {
        let credit_file = std::fs::read("assets/song.mp3").unwrap();
        let (response, credit_key) = app
            .upload_file(
                &client,
                "audio/mpeg",
                &format!("song-{i}.mp3"),
                credit_file,
            )
            .await;
        assert_eq!(response.status().as_u16(), 200);
        credits.push(credit_key.parse().unwrap());
    }

    let genres_list = open_access::get_genres_list()
        .bind(&app.pg_client)
        .all()
        .await
        .unwrap();

    let genres: Vec<_> =
        get_rand_subiter(&genres_list, 0..10).cloned().collect();

    let body = SubmitServiceRequest::Mixing(MusicService {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: Some(credits),
        },
        genres: Some(genres.clone()),
    });

    let response = client
        .post(format!(
            "{}/api/protected/creator/submit_service",
            app.address
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn submitting_ghost_writing_service_bad_formed_credits_fails() {
    let app =
        TestApp::spawn_app(Settings::load_configuration().unwrap(), 1).await;

    let test_user = TestUser::generate_user(String::from("creator"), 0);
    app.register_user(&test_user).await;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
    assert_eq!(app.login_user(&test_user, &client).await.as_u16(), 200);

    // Should be rejected because lyric is too long
    let image_file = std::fs::read("assets/image.png").unwrap();
    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file.clone())
        .await;
    assert_eq!(response.status().as_u16(), 200);
    let credit_string = std::iter::repeat('a').take(5001).collect::<String>();
    let body = SubmitServiceRequest::GhostWriting {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: None,
        },
        credits: Some(vec![credit_string.into()]),
    };
    let response = client
        .post(format!(
            "{}/api/protected/creator/submit_service",
            app.address
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 400);

    // Should be rejected because too many credits
    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file.clone())
        .await;
    assert_eq!(response.status().as_u16(), 200);
    let credits = std::iter::repeat(String::from("abc"))
        .take(10)
        .collect::<Vec<_>>();
    let body = SubmitServiceRequest::GhostWriting {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: None,
        },
        credits: Some(credits),
    };
    let response = client
        .post(format!(
            "{}/api/protected/creator/submit_service",
            app.address
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 400);

    // Should be rejected because lyric contains control characters
    let (response, image_key) = app
        .upload_file(&client, "image/png", "image.png", image_file.clone())
        .await;
    assert_eq!(response.status().as_u16(), 200);
    let credits_string = String::from("hello\r\r");
    let body = SubmitServiceRequest::GhostWriting {
        service: mustore::domain::requests::creator_access::Service {
            name: "Some service".to_string(),
            description: None,
            cover_object_key: image_key.parse().unwrap(),
            display_price: 500.into(),
            credits_object_keys: None,
        },
        credits: Some(vec![credits_string]),
    };
    let response = client
        .post(format!(
            "{}/api/protected/creator/submit_service",
            app.address
        ))
        .json(&body)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 400);
}

// ───── Functions ────────────────────────────────────────────────────────── //

fn get_rand_subiter<'a, T, R>(
    input: &'a [T],
    bounds: R,
) -> impl Iterator<Item = &'a T> + 'a + std::fmt::Debug
where
    T: std::fmt::Debug,
    R: RangeBounds<usize>,
{
    let mut rng = rand::thread_rng();

    let start = match bounds.start_bound() {
        std::ops::Bound::Included(&start) => start,
        std::ops::Bound::Excluded(&start) => start + 1,
        std::ops::Bound::Unbounded => 0,
    };

    let end = match bounds.end_bound() {
        std::ops::Bound::Included(&end) => end + 1,
        std::ops::Bound::Excluded(&end) => end,
        std::ops::Bound::Unbounded => input.len(),
    };

    let count = rng.gen_range(start..end);

    let indices: Vec<usize> =
        rand::seq::index::sample(&mut rng, input.len(), count)
            .into_iter()
            .collect();

    indices.into_iter().map(move |i| &input[i])
}

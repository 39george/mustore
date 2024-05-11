use mustore::{
    cornucopia::queries::open_access,
    domain::requests::creator_access::{
        MusicProduct, MusicService, Product, SubmitProductRequest,
        SubmitServiceRequest,
    },
};

use crate::helpers::get_rand_subiter;

use super::{TestApp, TestUser};

pub async fn new_song_for_creator(
    app: &TestApp,
    client: &reqwest::Client,
    price: usize,
) {
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
            price: price.into(),
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

pub async fn new_mixing_service_for_creator(
    app: &TestApp,
    client: &reqwest::Client,
    price: usize,
) {
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
            display_price: price.into(),
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

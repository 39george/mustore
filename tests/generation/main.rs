use deadpool_postgres::{Client, Manager, ManagerConfig, Pool};
use fake::Fake;
use mustore::{
    config::{DatabaseSettings, Settings},
    cornucopia::queries::user_auth_queries::*,
};
use postgres_types::{FromSql, ToSql};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use rust_decimal::Decimal;
use secrecy::ExposeSecret;
use tokio_postgres::NoTls;

#[tokio::test]
#[ignore]
async fn fill_with_test_data() {
    let mut rng = rand::thread_rng();
    let mut songs = Vec::new();
    let mut users = Vec::new();
    let client =
        get_postgres_client(&Settings::load_configuration().unwrap().database)
            .await;

    client
        .simple_query(
            "
            INSERT INTO genres (name)
            VALUES  ('rock'),
                    ('pop'), ('punk'),
                    ('rap'), ('hip_hop'),
                    ('classical');
        ",
        )
        .await
        .unwrap();

    client
        .simple_query(
            "
            INSERT INTO tags (name)
            VALUES  ('kind'),
                    ('good'), ('dark'),
                    ('morning'), ('light'),
                    ('easy');
        ",
        )
        .await
        .unwrap();

    for _ in 0..200 {
        let test_user = TestUser::generate_user(String::from("creator"));
        let user_settings_id = insert_new_user_settings()
            .bind(&client)
            .one()
            .await
            .unwrap();
        let user_id = insert_new_user()
            .bind(
                &client,
                &user_settings_id,
                &test_user.username,
                &test_user.email,
                &test_user.password,
            )
            .one()
            .await
            .unwrap();
        users.push(user_id);

        for _ in 1..rng.gen_range(3..30) {
            let song = TestSong::generate_song();
            let product_id: i32 = client
                .query_one(
                    "
                INSERT INTO products (owner_id, name, price, status)
                VALUES ($1, $2, $3, $4) returning id",
                    &[&user_id, &song.name, &song.price, &song.status],
                )
                .await
                .unwrap()
                .get(0);
            let song_id: i32 = client.query_one("
                INSERT INTO songs (products_id, primary_genre, sex, tempo, key, duration, lyric)
                VALUES (
                    $1,
                    (SELECT id FROM genres WHERE name = $2),
                $3, $4, $5, $6, $7) returning id
                ", &[&product_id, &rng.gen::<Genre>().to_string(), &song.sex, &song.tempo, &song.key, &song.duration, &song.lyric]).await.unwrap().get(0);
            songs.push(song_id);
            client
                .query(
                    "
                INSERT INTO products_tags (products_id, tags_id)
                VALUES (
                    $1,
                    (SELECT id FROM tags WHERE name = $2)
                )
                ",
                    &[&product_id, &rng.gen::<Tag>().to_string()],
                )
                .await
                .unwrap();
        }
    }

    for user_id in users.iter() {
        for _ in 1..rng.gen_range(50..120) {
            let song_id: i32 = rng.gen_range(0..songs.len() as i32);
            let _ = client
                .query(
                    "
                INSERT INTO likes (users_id, songs_id)
                VALUES (
                    $1,
                    $2
                )
                ",
                    &[&user_id, &song_id],
                )
                .await;
        }
    }

    for user_id in users.iter() {
        for _ in 1..rng.gen_range(50..120) {
            let song_id: i32 = rng.gen_range(0..songs.len() as i32);
            let _ = client
                .query(
                    "
                INSERT INTO listenings (users_id, songs_id)
                VALUES (
                    $1,
                    $2
                )
                ",
                    &[&user_id, &song_id],
                )
                .await;
        }
    }
}

pub async fn get_postgres_client(configuration: &DatabaseSettings) -> Client {
    let pg_config = get_pg_conf(configuration);
    let connector = NoTls;
    let manager_config = ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(pg_config, connector, manager_config);
    let pool = Pool::builder(manager).max_size(16).build().unwrap();
    pool.get().await.unwrap()
}

fn get_pg_conf(configuration: &DatabaseSettings) -> tokio_postgres::Config {
    let mut config = tokio_postgres::Config::new();
    config.user(&configuration.username);
    config.dbname(&configuration.database_name);
    config.host(&configuration.host);
    config.password(&configuration.password.expose_secret());
    config
}

#[allow(non_camel_case_types)]
#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "productstatus")]
pub enum ProductStatus {
    moderation,
    denied,
    active,
    hidden,
    sold,
}

#[allow(non_camel_case_types)]
#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "musickey")]
pub enum MusicKey {
    a_minor,
    a_major,
    b_flat_minor,
    b_flat_major,
    b_minor,
    b_major,
    c_minor,
    c_major,
    c_sharp_minor,
    c_sharp_major,
    d_minor,
    d_major,
    e_flat_minor,
    e_flat_major,
    e_minor,
    e_major,
    f_minor,
    f_major,
    f_sharp_minor,
    f_sharp_major,
    g_minor,
    g_major,
    a_flat_minor,
    a_flat_major,
}

impl std::fmt::Display for MusicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MusicKey::a_minor => f.write_str("a_minor "),
            MusicKey::a_major => f.write_str("a_major "),
            MusicKey::b_flat_minor => f.write_str("b_flat_minor "),
            MusicKey::b_flat_major => f.write_str("b_flat_major "),
            MusicKey::b_minor => f.write_str("b_minor "),
            MusicKey::b_major => f.write_str("b_major "),
            MusicKey::c_minor => f.write_str("c_minor "),
            MusicKey::c_major => f.write_str("c_major "),
            MusicKey::c_sharp_minor => f.write_str("c_sharp_minor "),
            MusicKey::c_sharp_major => f.write_str("c_sharp_major "),
            MusicKey::d_minor => f.write_str("d_minor "),
            MusicKey::d_major => f.write_str("d_major "),
            MusicKey::e_flat_minor => f.write_str("e_flat_minor "),
            MusicKey::e_flat_major => f.write_str("e_flat_major "),
            MusicKey::e_minor => f.write_str("e_minor "),
            MusicKey::e_major => f.write_str("e_major "),
            MusicKey::f_minor => f.write_str("f_minor "),
            MusicKey::f_major => f.write_str("f_major "),
            MusicKey::f_sharp_minor => f.write_str("f_sharp_minor "),
            MusicKey::f_sharp_major => f.write_str("f_sharp_major "),
            MusicKey::g_minor => f.write_str("g_minor "),
            MusicKey::g_major => f.write_str("g_major "),
            MusicKey::a_flat_minor => f.write_str("a_flat_minor "),
            MusicKey::a_flat_major => f.write_str("a_flat_major "),
        }
    }
}

impl Distribution<MusicKey> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MusicKey {
        let key = rng.gen_range(0..24);
        match key {
            0 => MusicKey::a_minor,
            1 => MusicKey::a_major,
            2 => MusicKey::b_flat_minor,
            3 => MusicKey::b_flat_major,
            4 => MusicKey::b_minor,
            5 => MusicKey::b_major,
            6 => MusicKey::c_minor,
            7 => MusicKey::c_major,
            8 => MusicKey::c_sharp_minor,
            9 => MusicKey::c_sharp_major,
            10 => MusicKey::d_minor,
            11 => MusicKey::d_major,
            12 => MusicKey::e_flat_minor,
            13 => MusicKey::e_flat_major,
            14 => MusicKey::e_minor,
            15 => MusicKey::e_major,
            16 => MusicKey::f_minor,
            17 => MusicKey::f_major,
            18 => MusicKey::f_sharp_minor,
            19 => MusicKey::f_sharp_major,
            20 => MusicKey::g_minor,
            21 => MusicKey::g_major,
            22 => MusicKey::a_flat_minor,
            23 => MusicKey::a_flat_major,
            _ => unreachable!(),
        }
    }
}

#[allow(dead_code)]
struct TestUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: Option<String>,
    pub admin_token: Option<uuid::Uuid>,
}

#[allow(dead_code)]
impl TestUser {
    pub fn generate_user(role: String) -> Self {
        Self {
            username: fake::faker::name::en::Name().fake(),
            password: String::from("A23c(fds)Helloworld232r"),
            email: fake::faker::internet::en::FreeEmail().fake(),
            role: Some(role),
            admin_token: None,
        }
    }

    pub fn generate_admin(admin_token: uuid::Uuid) -> Self {
        Self {
            username: fake::faker::name::en::Name().fake(),
            password: String::from("A23c(fds)Helloworld232r"),
            email: fake::faker::internet::en::SafeEmail().fake(),
            role: None,
            admin_token: Some(admin_token),
        }
    }
}

struct TestSong {
    name: String,
    price: Decimal,
    status: ProductStatus,
    sex: String,
    tempo: i16,
    key: MusicKey,
    duration: f32,
    lyric: String,
}

impl TestSong {
    fn generate_song() -> Self {
        let mut rng = rand::thread_rng();
        TestSong {
            name: fake::faker::name::en::Name().fake(),
            price: rng.gen_range(1000..100000).into(),
            status: ProductStatus::active,
            sex: vec!["male", "female"][rng.gen_range(0..2)].to_string(),
            tempo: rng.gen_range(50..210),
            key: rng.gen(),
            duration: rng.gen_range(1.0..5.0),
            lyric: fake::faker::lorem::en::Paragraph(1..5).fake(),
        }
    }
}

#[allow(non_camel_case_types)]
enum Genre {
    rock,
    pop,
    punk,
    rap,
    hip_hop,
    classical,
}

impl Distribution<Genre> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Genre {
        let genre = rng.gen_range(0..5);
        match genre {
            0 => Genre::rock,
            1 => Genre::pop,
            2 => Genre::punk,
            3 => Genre::rap,
            4 => Genre::hip_hop,
            5 => Genre::classical,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Genre {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Genre::rock => f.write_str("rock"),
            Genre::pop => f.write_str("pop"),
            Genre::punk => f.write_str("punk"),
            Genre::rap => f.write_str("rap"),
            Genre::hip_hop => f.write_str("hip_hop"),
            Genre::classical => f.write_str("classical"),
        }
    }
}

#[allow(non_camel_case_types)]
enum Tag {
    kind,
    good,
    dark,
    morning,
    light,
    easy,
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tag::kind => f.write_str("kind"),
            Tag::good => f.write_str("good"),
            Tag::dark => f.write_str("dark"),
            Tag::morning => f.write_str("morning"),
            Tag::light => f.write_str("light"),
            Tag::easy => f.write_str("easy"),
        }
    }
}

impl Distribution<Tag> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tag {
        let genre = rng.gen_range(0..5);
        match genre {
            0 => Tag::kind,
            1 => Tag::good,
            2 => Tag::dark,
            3 => Tag::morning,
            4 => Tag::light,
            5 => Tag::easy,
            _ => unreachable!(),
        }
    }
}

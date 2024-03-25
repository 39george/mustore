//! This is a module with common initialization functions.

use std::collections::HashMap;
use std::path::Path;

use deadpool_postgres::Client;
use fake::Fake;
use fred::clients::RedisClient;
use mustore::service_providers::object_storage::presigned_post_form::PresignedPostData;
use mustore::service_providers::object_storage::ObjectStorage;
use mustore::startup::get_redis_connection_pool;
use reqwest::multipart::Form;
use reqwest::multipart::Part;
use secrecy::{ExposeSecret, Secret};
use tokio_postgres::NoTls;
use tracing::Level;
use wiremock::Mock;
use wiremock::MockServer;

use mustore::config::DatabaseSettings;
use mustore::config::Settings;
use mustore::startup::get_postgres_connection_pool;
use mustore::startup::Application;
use wiremock::matchers;
use wiremock::ResponseTemplate;

#[derive(Debug)]
pub struct TestUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: Option<String>,
    pub admin_token: Option<uuid::Uuid>,
    pub idx: usize,
}

impl TestUser {
    /// `Idx` - is index which response from email mock we will retrieve.
    pub fn generate_user(role: String, idx: usize) -> Self {
        Self {
            username: fake::faker::name::en::Name().fake(),
            password: String::from("A23c(fds)Helloworld232r"),
            email: fake::faker::internet::en::FreeEmail().fake(),
            role: Some(role),
            admin_token: None,
            idx,
        }
    }

    pub fn generate_admin(admin_token: uuid::Uuid, idx: usize) -> Self {
        Self {
            username: fake::faker::name::en::Name().fake(),
            password: String::from("A23c(fds)Helloworld232r"),
            email: fake::faker::internet::en::FreeEmail().fake(),
            role: None,
            admin_token: Some(admin_token),
            idx,
        }
    }

    pub async fn post_signup(
        &self,
        host: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut form = HashMap::new();
        form.insert("username", self.username.clone());
        form.insert("password", self.password.clone());
        form.insert("email", self.email.clone());
        form.insert("recaptcha_token", String::from("token"));
        match (&self.role, self.admin_token) {
            (None, Some(admin_token)) => {
                form.insert(
                    "admin_token",
                    admin_token.hyphenated().to_string(),
                );
            }
            (Some(role), None) => {
                form.insert("user_role", role.clone());
            }
            _ => unreachable!(),
        }
        client
            .post(format!("{}/api/signup", host))
            .form(&form)
            .send()
            .await
    }
}

/// This type contains MockServer, and it's address.
/// MockServer represents a email delivery service,
/// such as Postmark or SMTP.bz
pub struct TestApp {
    pg_username: String,
    pg_config_with_root_cred: DatabaseSettings,
    pub address: String,
    pub pg_client: Client,
    pub redis_client: RedisClient,
    pub email_server: MockServer,
    pub port: u16,
}

/// Confirmation links embedded in the request to the email API.
#[derive(Debug)]
pub struct ConfirmationLink(pub reqwest::Url);

impl TestApp {
    pub async fn spawn_app(
        mut config: Settings,
        email_mock_expect_times: u64,
    ) -> TestApp {
        init_tracing();

        // Run tests on 1st redis database
        config.redis.db_number = 1;
        config.object_storage.bucket_name = String::from("mustore-test-data");
        let redis_client = get_redis_connection_pool(&config.redis)
            .await
            .unwrap()
            .next()
            .clone_new();

        fred::interfaces::ClientLike::connect(&redis_client);
        fred::interfaces::ClientLike::wait_for_connect(&redis_client)
            .await
            .unwrap();

        let email_server = MockServer::start().await;
        config.email_client.base_url = email_server.uri();

        config.app_port = 0;

        let pg_config_with_root_cred = config.database.clone();
        let (pg_config, pg_pool, pg_username) =
            prepare_postgres_with_rand_user(config.database.clone()).await;
        config.database = pg_config;

        let application = Application::build(config)
            .await
            .expect("Failed to build application");

        let port = application.port();

        let address = format!("http://127.0.0.1:{}", port);

        // Very important step
        let _ = tokio::spawn(application.run_until_stopped());

        Mock::given(matchers::path("/v1/smtp/send"))
            .and(matchers::method("POST"))
            .and(matchers::header_exists("Authorization"))
            .respond_with(ResponseTemplate::new(200))
            .expect(email_mock_expect_times)
            .mount(&email_server)
            .await;

        TestApp {
            pg_username,
            pg_config_with_root_cred,
            address,
            pg_client: pg_pool,
            email_server,
            port,
            redis_client,
        }
    }

    /// `WARNING`: This function will create a new email delivery mock server.
    pub async fn signup_user_get_confirmation_link(
        &self,
        user: &TestUser,
    ) -> ConfirmationLink {
        let response = user.post_signup(&self.address).await.unwrap();
        assert!(response.status().is_success());

        let request =
            &self.email_server.received_requests().await.unwrap()[user.idx];

        self.get_confirmation_link_urlencoded(request)
    }

    /// `WARNING`: This function will create a new email delivery mock server.
    pub async fn register_user(
        &self,
        test_user: &TestUser,
    ) -> reqwest::StatusCode {
        let confirmation_link =
            self.signup_user_get_confirmation_link(&test_user).await;
        let response = reqwest::get(confirmation_link.0).await.unwrap();
        response.status()
    }

    pub async fn login_user(
        &self,
        test_user: &TestUser,
        client: &reqwest::Client,
    ) -> reqwest::StatusCode {
        let response = client
            .post(format!("{}/api/login", &self.address))
            .json(&serde_json::json!({
                "email": test_user.email,
                "password": test_user.password
            }))
            .send()
            .await
            .unwrap();
        response.status()
    }

    /// Extract the confirmation links embedded in the email API in urlencoded.
    pub fn get_confirmation_link_urlencoded(
        &self,
        email_request: &wiremock::Request,
    ) -> ConfirmationLink {
        let form_data: HashMap<String, String> =
            serde_urlencoded::from_bytes(&email_request.body).unwrap();
        let html_content = form_data.get("html").unwrap();

        // Extract the link from one of the request fields.
        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            let raw_link = links[1].as_str().to_string();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
            // Let's make sure we don't call random APIs on the web
            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let link = get_link(&html_content);

        ConfirmationLink(link)
    }

    pub async fn upload_file(
        &self,
        client_with_cookies: &reqwest::Client,
        media_type: &str,
        name: &str,
        file: Vec<u8>,
    ) -> (reqwest::Response, String) {
        let response = client_with_cookies
            .get(format!(
                "{}/api/protected/user/upload_form?media_type={}&file_name={}",
                self.address, media_type, name
            ))
            .send()
            .await
            .unwrap();
        let post_form: PresignedPostData = response.json().await.unwrap();
        let object_key = post_form.fields.get("key").unwrap().clone();
        let url = post_form.url;
        let mut multipart = Form::new();
        for (key, value) in post_form.fields.into_iter() {
            multipart = multipart.text(key, value);
        }
        multipart = multipart.part("file", Part::bytes(file));
        (
            client_with_cookies
                .post(url)
                .multipart(multipart)
                .send()
                .await
                .unwrap(),
            object_key,
        )
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        opentelemetry::global::shutdown_tracer_provider();
        // Clean pg
        let db_config = self.pg_config_with_root_cred.clone();
        let db_username = self.pg_username.clone();
        // NOTE: Spawn a new thread, because internally sync postgres client uses
        // tokio runtime, but we are already in tokio runtime here. To
        // spawn a new tokio runtime, we should do it inside new thread.
        let _ = std::thread::spawn(move || {
            // Create the runtime
            let rt = tokio::runtime::Runtime::new().unwrap();
            // Execute the future, blocking the current thread until completion
            rt.block_on(async {
                let pg_pool = get_postgres_connection_pool(&db_config);
                let client = pg_pool.get().await.unwrap();
                let create_role =
                    format!("DROP SCHEMA {0} CASCADE;", db_username);
                let create_schema = format!("DROP ROLE {0};", db_username);
                println!("Executing: {create_role}");
                client.simple_query(&create_role).await.unwrap();
                println!("Executing: {create_schema}");
                client.simple_query(&create_schema).await.unwrap();
            });
        })
        .join();
    }
}

fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::Layer;
    if let Ok(_) = std::env::var("TEST_JAEGER") {
        // Opentelemetry
        let tracer = opentelemetry_jaeger::new_agent_pipeline()
            .with_service_name("mustore_test")
            .install_simple()
            .unwrap();
        let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        // Console
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_filter(filter);

        tracing_subscriber::registry()
            .with(opentelemetry)
            .with(layer)
            .try_init()
            .unwrap();
    } else if let Ok(_) = std::env::var("TEST_TRACING") {
        let subscriber = tracing_subscriber::fmt()
            .with_timer(tracing_subscriber::fmt::time::ChronoLocal::default())
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(Level::INFO.into())
                    .add_directive("tower_sessions_core=warn".parse().unwrap())
                    .add_directive("axum::rejection=trace".parse().unwrap())
                    .add_directive("aws_config=warn".parse().unwrap()),
            )
            .compact()
            .with_level(true)
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);
    }
}

async fn prepare_postgres_with_rand_user(
    mut pg_config: DatabaseSettings,
) -> (DatabaseSettings, Client, String) {
    let pool = get_postgres_connection_pool(&pg_config);
    let pg_username = generate_username();
    let create_role =
        format!("CREATE ROLE {0} WITH LOGIN PASSWORD '{0}';", &pg_username);
    let create_schema =
        format!("CREATE SCHEMA {0} AUTHORIZATION {0};", &pg_username);
    let client = pool.get().await.unwrap();
    client.simple_query(&create_role).await.unwrap();
    client.simple_query(&create_schema).await.unwrap();
    drop(pool);
    pg_config.username = pg_username.clone();
    pg_config.password = Secret::new(pg_username.clone());
    let pg_pool = get_postgres_connection_pool(&pg_config);
    let client = pg_pool.get().await.unwrap();
    (pg_config, client, pg_username)
}

// ───── Helpers ──────────────────────────────────────────────────────────── //

pub fn generate_username() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "test_{}",
        std::iter::repeat_with(|| {
            rand::Rng::sample(&mut rng, rand::distributions::Alphanumeric)
        })
        .map(|b| char::from(b).to_lowercase().next().unwrap())
        .take(5)
        .collect::<String>()
    )
}

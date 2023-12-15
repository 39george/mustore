//! This is a module with common initialization functions.

use std::collections::HashMap;

use deadpool_postgres::Pool;
use fake::Fake;
use secrecy::{ExposeSecret, Secret};
use tokio_postgres::NoTls;
use wiremock::MockServer;

use mustore::config::DatabaseSettings;
use mustore::config::Settings;
use mustore::startup::get_postgres_connection_pool;
use mustore::startup::Application;

pub struct TestUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            username: fake::faker::name::en::Name().fake(),
            password: String::from("A23c(fds)Helloworld232r"),
            email: fake::faker::internet::en::SafeEmail().fake(),
        }
    }

    pub async fn post_signup(
        &self,
        host: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let client = reqwest::Client::new();
        let mut form = HashMap::new();
        form.insert("username", &self.username);
        form.insert("password", &self.password);
        form.insert("email", &self.email);
        client
            .post(format!("{}/signup", host))
            .form(&form)
            .send()
            .await
    }
}

/// This type contains MockServer, and it's address.
/// MockServer represents a email delivery service,
/// such as Postmark.
pub struct TestApp {
    db_username: String,
    db_config_with_root_cred: DatabaseSettings,
    pub address: String,
    pub pool: Pool,
    pub email_server: MockServer,
    pub port: u16,
}

/// Confirmation links embedded in the request to the email API.
#[derive(Debug)]
pub struct ConfirmationLink(pub reqwest::Url);

impl TestApp {
    pub async fn spawn_app(mut config: Settings) -> TestApp {
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
            let opentelemetry =
                tracing_opentelemetry::layer().with_tracer(tracer);
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
                .with_timer(
                    tracing_subscriber::fmt::time::ChronoLocal::default(),
                )
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
                .with_max_level(tracing::Level::INFO)
                .compact()
                .with_level(true)
                .finish();

            let _ = tracing::subscriber::set_global_default(subscriber);
        }

        // We should randomize app port
        let mut db_config = config.database.clone();

        // Connect as an admin user
        let pool = get_postgres_connection_pool(&db_config);
        let db_username = generate_username();

        // Create new random user account in pg
        let create_role =
            format!("CREATE ROLE {0} WITH LOGIN PASSWORD '{0}';", &db_username);
        let create_schema =
            format!("CREATE SCHEMA {0} AUTHORIZATION {0};", &db_username);
        let client = &pool.get().await.unwrap();
        client.simple_query(&create_role).await.unwrap();
        client.simple_query(&create_schema).await.unwrap();

        drop(pool);
        db_config.username = db_username.clone();
        db_config.password = Secret::new(db_username.clone());

        // Connect as a new user
        let pool = get_postgres_connection_pool(&db_config);

        let email_server = MockServer::start().await;

        // Set base_url to our MockServer instead of real email delivery service.
        config.email_client.base_url = email_server.uri();
        config.app_port = 0;
        // For Drop
        let db_config_with_root_cred = config.database.clone();

        // Store db_config with test user in config destined for Application::build
        config.database = db_config;

        let application = Application::build(config)
            .await
            .expect("Failed to build application");

        let port = application.port();

        let address = format!("http://127.0.0.1:{}", port);

        // Very important step
        let _ = tokio::spawn(application.run_until_stopped());

        TestApp {
            db_username,
            db_config_with_root_cred,
            address,
            pool,
            email_server,
            port,
        }
    }

    /// This function sends Post request to our TestApp,
    /// to /subscriptions path. If successful, it will create
    /// a line in postgres db.
    pub async fn post_subscriptions(
        &self,
        body: &'static str,
    ) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/subscriptions", &self.address))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    /// Extract the confirmation links embedded in the email API in json.
    pub fn _get_confirmation_link_json(
        &self,
        email_request: &wiremock::Request,
    ) -> ConfirmationLink {
        let body: serde_json::Value =
            serde_json::from_slice(&email_request.body).unwrap();

        // Extract the link from one of the request fields.
        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            let raw_link = links[0].as_str().to_string();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
            // Let's make sure we don't call random APIs on the web
            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let link = get_link(&body["text_body"].as_str().unwrap());

        ConfirmationLink(link)
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
            let raw_link = links[6].as_str().to_string();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
            // Let's make sure we don't call random APIs on the web
            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let link = get_link(&html_content);

        ConfirmationLink(link)
    }

    // pub async fn post_newsletters(
    //     &self,
    //     body: serde_json::Value,
    // ) -> reqwest::Response {
    //     reqwest::Client::new()
    //         .post(&format!("{}/newsletters", self.address))
    //         .json(&body)
    //         .basic_auth(
    //             &self.test_user.username,
    //             Some(&self.test_user.password),
    //         )
    //         .send()
    //         .await
    //         .expect("Failed to execute request.")
    // }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        opentelemetry::global::shutdown_tracer_provider();
        // Clean pg
        let db_config = self.db_config_with_root_cred.clone();
        let db_username = self.db_username.clone();
        // Spawn a new thread, because internally sync postgres client uses
        // tokio runtime, but we are already in tokio runtime here. To
        // spawn a new tokio runtime, we should do it inside new thread.
        let _ = std::thread::spawn(move || {
            let mut client = get_sync_postgres_client(&db_config);
            let create_role = format!("DROP SCHEMA {0} CASCADE;", db_username);
            let create_schema = format!("DROP ROLE {0};", db_username);
            client.simple_query(&create_role).unwrap();
            client.simple_query(&create_schema).unwrap();
        })
        .join();
    }
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

pub fn get_sync_postgres_client(
    configuration: &DatabaseSettings,
) -> postgres::Client {
    postgres::Client::connect(
        configuration.connection_string().expose_secret(),
        NoTls,
    )
    .unwrap()
}

use axum::error_handling::HandleErrorLayer;
use axum::routing;
use axum::serve::Serve;
use axum::BoxError;
use axum::Router;
use axum_login::AuthManagerLayerBuilder;
use deadpool_postgres::Manager;
use deadpool_postgres::ManagerConfig;

use deadpool_postgres::Pool;
use fred::clients::RedisClient;
use fred::clients::RedisPool;
use fred::types::RedisConfig;
use http::StatusCode;
use secrecy::ExposeSecret;

use time::Duration;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth;
use crate::config::DatabaseSettings;
use crate::config::EmailClientSettings;
use crate::config::RedisSettings;
use crate::config::Settings;
use crate::email_client::EmailClient;
use crate::email_client::EmailDeliveryService;
use crate::routes::health_check::health_check;
use crate::routes::open::open_router;
use crate::routes::protected::protected_router;
use crate::service_providers::object_storage::YandexObjectStorage;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod db_migration;

// ───── Body ─────────────────────────────────────────────────────────────── //

/// This is a central type of our codebase. `Application` type builds server
/// for both production and testing purposes.
pub struct Application {
    port: u16,
    serve: Serve<Router, Router>,
}

/// Shareable type, we insert it to the main `Router` as state,
/// at the launch stage.
#[derive(Clone, Debug)]
pub struct AppState {
    pub base_url: String,
    pub pg_pool: Pool,
    pub redis_pool: RedisPool,
    pub object_storage: YandexObjectStorage,
    pub email_client: EmailClient,
    pub argon2_obj: argon2::Argon2<'static>,
}

impl Application {
    /// Build a new server.
    ///
    /// This functions builds a new `Application` with given configuration.
    /// It also configures a pool of connections to the PostgreSQL database.
    pub async fn build(
        configuration: Settings,
    ) -> Result<Application, anyhow::Error> {
        let pg_pool = get_postgres_connection_pool(&configuration.database);
        let redis_pool =
            get_redis_connection_pool(&configuration.redis).await?;

        let redis_client = redis_pool.next().clone_new();
        fred::interfaces::ClientLike::connect(&redis_client);
        fred::interfaces::ClientLike::wait_for_connect(&redis_client).await?;

        let object_storage =
            YandexObjectStorage::new(configuration.object_storage).await;
        let email_client = get_email_client(
            &configuration.email_client,
            configuration.email_delivery_service,
        )?;

        db_migration::run_migration(&pg_pool).await;

        let address =
            format!("{}:{}", configuration.app_addr, configuration.app_port);
        tracing::info!("running on {} address", address);
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr()?.port();

        let serve = Self::build_server(
            &configuration.app_base_url,
            listener,
            pg_pool,
            redis_pool,
            redis_client,
            object_storage,
            email_client,
        );

        Ok(Self { serve, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    /// This function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.serve.await?;
        Ok(())
    }

    /// Configure `Server`.
    fn build_server(
        base_url: &str,
        listener: TcpListener,
        pg_pool: Pool,
        redis_pool: RedisPool,
        redis_client: RedisClient,
        object_storage: YandexObjectStorage,
        email_client: EmailClient,
    ) -> Serve<Router, Router> {
        let argon2_obj = argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            // Params are good
            argon2::Params::new(15000, 2, 1, None).unwrap(),
        );

        // We do not wrap pool into arc because internally it alreaday has an
        // `Arc`, and copying is cheap.
        let app_state = AppState {
            pg_pool: pg_pool.clone(),
            redis_pool,
            object_storage,
            email_client,
            base_url: base_url.to_string(),
            argon2_obj,
        };

        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store =
            axum_login::tower_sessions::RedisStore::new(redis_client);
        let session_layer =
            axum_login::tower_sessions::SessionManagerLayer::new(session_store)
                .with_secure(true)
                .with_expiry(axum_login::tower_sessions::Expiry::OnInactivity(
                    Duration::days(1),
                ));

        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = crate::auth::users::Backend::new(app_state.clone());
        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|e: BoxError| async move {
                tracing::error!("GOT HANDLE ERROR: {}", e);
                StatusCode::BAD_REQUEST
            }))
            .layer(
                AuthManagerLayerBuilder::new(backend, session_layer).build(),
            );

        let mut app = Router::new()
            .nest("/api/protected", protected_router())
            .nest("/api/open", open_router())
            .route("/api/health_check", routing::get(health_check))
            .route("/api/signup", routing::post(auth::user_signup::signup))
            .route(
                "/api/confirm_user_account",
                routing::get(auth::user_confirm_account::confirm),
            )
            .with_state(app_state)
            .merge(auth::user_login::login_router())
            .layer(auth_service);

        if let Ok(_) = std::env::var("TEST_TRACING") {
            app = app.layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        tower_http::trace::DefaultMakeSpan::new()
                            .level(tracing::Level::INFO),
                    )
                    .on_response(
                        tower_http::trace::DefaultOnResponse::new()
                            .level(tracing::Level::INFO),
                    )
                    .on_failure(
                        tower_http::trace::DefaultOnFailure::new()
                            .level(tracing::Level::ERROR),
                    ),
            );
        }

        axum::serve(listener, app)
    }
}

fn get_email_client(
    configuration: &EmailClientSettings,
    email_delivery_service: EmailDeliveryService,
) -> Result<EmailClient, anyhow::Error> {
    let timeout = configuration.timeout_millis();
    let sender_email = configuration.sender()?;
    let email_client = EmailClient::new(
        configuration.base_url.clone(),
        sender_email,
        configuration.authorization_token.clone(),
        timeout,
        email_delivery_service,
    )?;
    Ok(email_client)
}

pub async fn get_redis_connection_pool(
    configuration: &RedisSettings,
) -> Result<RedisPool, anyhow::Error> {
    let redis_config = RedisConfig::from_url_centralized(
        configuration.connection_string().expose_secret(),
    )
    .unwrap();
    let redis_pool = fred::types::Builder::default_centralized()
        .set_config(redis_config)
        .build_pool(5)
        .expect("Failed to build redis connections pool");
    fred::interfaces::ClientLike::connect(&redis_pool);
    fred::interfaces::ClientLike::wait_for_connect(&redis_pool).await?;
    Ok(redis_pool)
}

pub fn get_postgres_connection_pool(configuration: &DatabaseSettings) -> Pool {
    let pg_config = get_pg_conf(configuration);
    let connector = NoTls;
    let manager_config = ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(pg_config, connector, manager_config);
    let pool = Pool::builder(manager).max_size(16).build().unwrap();
    pool
}

fn get_pg_conf(configuration: &DatabaseSettings) -> tokio_postgres::Config {
    let mut config = tokio_postgres::Config::new();
    config.user(&configuration.username);
    config.dbname(&configuration.database_name);
    config.host(&configuration.host);
    config.password(&configuration.password.expose_secret());
    config
}

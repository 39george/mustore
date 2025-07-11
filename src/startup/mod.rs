use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::extract::ConnectInfo;
use axum::middleware::AddExtension;
use axum::routing;
use axum::serve::Serve;
use axum::Router;
use axum_login::AuthManagerLayerBuilder;
use deadpool_postgres::Client;
use deadpool_postgres::Manager;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::Pool;
use fred::clients::RedisClient;
use fred::clients::RedisPool;
use fred::types::ReconnectPolicy;
use fred::types::RedisConfig;
use secrecy::ExposeSecret;
use time::Duration;
use time::UtcOffset;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use crate::auth;
use crate::config::DatabaseSettings;
use crate::config::EmailClientSettings;
use crate::config::RedisSettings;
use crate::config::Settings;
use crate::email_client::EmailClient;
use crate::email_client::EmailDeliveryService;
use crate::routes::development;
use crate::routes::health_check::health_check;
use crate::routes::notifications::notification_center_router;
use crate::routes::open::open_router;
use crate::routes::protected::protected_router;
use crate::service_providers::captcha_verifier::CaptchaVerifier;
use crate::service_providers::object_storage::ObjectStorage;

use self::api_doc::ApiDoc;

// ───── Submodules ───────────────────────────────────────────────────────── //

pub mod api_doc;
pub mod db_migration;
mod tasks;

// ───── Body ─────────────────────────────────────────────────────────────── //

lazy_static::lazy_static! {
    static ref MOSKOW_TIME_OFFSET: UtcOffset = UtcOffset::from_hms(3, 0, 0).unwrap();
}

type Server = Serve<
    IntoMakeServiceWithConnectInfo<Router, SocketAddr>,
    AddExtension<Router, ConnectInfo<SocketAddr>>,
>;

/// This is a central type of our codebase. `Application` type builds server
/// for both production and testing purposes.
pub struct Application {
    port: u16,
    server: Server,
}

/// Shareable type, we insert it to the main `Router` as state,
/// at the launch stage.
#[derive(Clone, Debug)]
pub struct AppState {
    pub pg_pool: Pool,
    pub redis_pool: RedisPool,
    pub object_storage: ObjectStorage,
    pub email_client: EmailClient,
    pub argon2_obj: argon2::Argon2<'static>,
    pub captcha_verifier: CaptchaVerifier,
    pub airactions_c: airactions::Client,
    pub settings: Arc<Settings>,
}

impl Application {
    /// Build a new server.
    ///
    /// This functions builds a new `Application` with given configuration.
    /// It also configures a pool of connections to the PostgreSQL database.
    pub async fn build(
        mut configuration: Settings,
        is_test: bool,
    ) -> Result<Application, anyhow::Error> {
        let pg_pool = get_postgres_connection_pool(&configuration.database);
        let pg_client = pg_pool
            .get()
            .await
            .expect("Failed to get pg client for scheduler");
        let redis_pool =
            get_redis_connection_pool(&configuration.redis).await?;
        let redis_pool_tower_sessions =
            get_redis_connection_pool(&configuration.redis).await?;

        let redis_client = redis_pool.next().clone_new();
        fred::interfaces::ClientLike::connect(&redis_client);
        fred::interfaces::ClientLike::wait_for_connect(&redis_client).await?;

        let object_storage =
            ObjectStorage::new(configuration.object_storage.clone()).await;
        let object_storage2 = object_storage.clone();
        let email_client = get_email_client(
            &configuration.email_client,
            configuration.email_delivery_service.clone(),
        )?;
        let captcha_verifier = CaptchaVerifier::new(
            configuration.recaptcha.endpoint_url.parse().unwrap(),
            configuration.recaptcha.secret.clone(),
        );
        // TODO: implement logic to use self-signed certificate only in development
        // also in tests
        let cert =
            include_bytes!("/home/ghashy/.local/share/mkcert/rootCA.pem");
        let payments_client = airactions::Client::from_client_and_url(
            reqwest::Client::builder()
                .add_root_certificate(
                    reqwest::Certificate::from_pem(cert).unwrap(),
                )
                .build()
                .unwrap(),
            configuration.payments.merchant_api_endpoint.clone(),
        )
        .unwrap();
        db_migration::run_migration(&pg_pool).await;

        let address =
            format!("{}:{}", configuration.app_addr, configuration.app_port);
        tracing::info!("running on {} address", address);
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr()?.port();

        // Override base url in tests (which use random ports)
        if is_test {
            configuration.app_base_url =
                format!("http://{}:{}", configuration.app_addr, port);
        }

        let server = Self::build_server(
            listener,
            pg_pool,
            redis_pool,
            redis_pool_tower_sessions,
            object_storage,
            email_client,
            captcha_verifier,
            payments_client,
            Arc::new(configuration),
        );

        tokio::spawn(async {
            if let Err(e) =
                run_scheduler(pg_client, redis_client, object_storage2).await
            {
                tracing::error!("Scheduler failed with: {e}");
            }
        });

        Ok(Self { server, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    /// This function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server
            .with_graceful_shutdown(shutdown_signal())
            .await?;
        Ok(())
    }

    /// Configure `Server`.
    fn build_server(
        listener: TcpListener,
        pg_pool: Pool,
        redis_pool: RedisPool,
        redis_pool_tower_sessions: RedisPool,
        object_storage: ObjectStorage,
        email_client: EmailClient,
        captcha_verifier: CaptchaVerifier,
        payments_client: airactions::Client,
        settings: Arc<Settings>,
    ) -> Server {
        let argon2_obj = argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            // Params are good
            argon2::Params::new(15000, 2, 1, None).unwrap(),
        );

        // We do not wrap pool into arc because internally it already has an
        // `Arc`, and copying is cheap.
        let app_state = AppState {
            pg_pool: pg_pool.clone(),
            redis_pool,
            object_storage,
            email_client,
            argon2_obj,
            captcha_verifier,
            airactions_c: payments_client,
            settings,
        };

        // Set 'secure' attribute for cookies
        let with_secure = if let Ok(e) = std::env::var("ENVIRONMENT") {
            if e.eq("development") {
                false
            } else {
                true
            }
        } else {
            true
        };

        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = tower_sessions_redis_store::RedisStore::new(
            redis_pool_tower_sessions,
        );
        let session_layer =
            axum_login::tower_sessions::SessionManagerLayer::new(session_store)
                .with_secure(with_secure)
                .with_expiry(axum_login::tower_sessions::Expiry::OnInactivity(
                    Duration::days(1),
                ));

        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = crate::auth::users::Backend::new(app_state.clone());
        let auth_service =
            AuthManagerLayerBuilder::new(backend, session_layer).build();

        #[rustfmt::skip]
        let mut app = Router::new()
            .nest("/api/protected", protected_router())
            .nest("/api/open", open_router())
            .route("/api/signup", routing::post(auth::signup::signup))
            .route(
                "/api/confirm_user_account",
                routing::get(auth::confirm_account::confirm),
            )
            .nest("/notification_center", notification_center_router())
            .with_state(app_state.clone())
            .merge(auth::login::login_router(app_state.clone()))
            .layer(crate::middleware::map_response::BadRequestIntoJsonLayer) // 3
            .layer(auth_service)                                             // 2
            .layer(crate::middleware::ban_by_ip::BanLayer {                  // 1
                state: app_state.clone(),
            })
            .route("/api/health_check", routing::get(health_check));

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

        if let Ok(e) = std::env::var("ENVIRONMENT") {
            if e.eq("development") {
                app = app.merge(
                    SwaggerUi::new("/swagger-ui")
                        .url("/api-docs/openapi.json", ApiDoc::openapi()),
                );
                app = app.nest("/api", development::dev_router(app_state));
            }
        }

        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    let terminate = async {
        tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate(),
        )
        .expect("failed to install signal handler")
        .recv()
        .await;
    };
    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    tracing::info!("Terminate signal received");
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
        .set_policy(ReconnectPolicy::default())
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

async fn run_scheduler(
    pg_client: Client,
    redis_client: RedisClient,
    object_storage: ObjectStorage,
) -> Result<(), tokio_cron_scheduler::JobSchedulerError> {
    let sched = tokio_cron_scheduler::JobScheduler::new().await?;

    let pg_client = std::sync::Arc::new(pg_client);
    let redis_client = std::sync::Arc::new(redis_client);
    let object_storage = std::sync::Arc::new(object_storage);
    sched
        .add(tasks::update_available_songs_materialized_view_task(
            pg_client,
        )?)
        .await?;
    sched
        .add(tasks::check_current_user_uploads(
            object_storage,
            redis_client,
        )?)
        .await?;

    if let Err(e) = sched.start().await {
        tracing::error!("Scheduler failed with {e}");
    }

    Ok(())
}

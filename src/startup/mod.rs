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
use fred::types::RedisConfig;
use secrecy::ExposeSecret;
use time::Duration;
use time::UtcOffset;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use tower_http::cors::CorsLayer;
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
use crate::routes::open::open_router;
use crate::routes::protected::protected_router;
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
    pub object_storage: ObjectStorage,
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
        let pg_client = pg_pool
            .get()
            .await
            .expect("Failed to get pg client for scheduler");
        let redis_pool =
            get_redis_connection_pool(&configuration.redis).await?;

        let redis_client = redis_pool.next().clone_new();
        let redis_client2 = redis_pool.next().clone_new();
        fred::interfaces::ClientLike::connect(&redis_client);
        fred::interfaces::ClientLike::connect(&redis_client2);
        fred::interfaces::ClientLike::wait_for_connect(&redis_client).await?;
        fred::interfaces::ClientLike::wait_for_connect(&redis_client2).await?;

        let object_storage =
            ObjectStorage::new(configuration.object_storage).await;
        let object_storage2 = object_storage.clone();
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

        tokio::spawn(async {
            if let Err(e) =
                run_scheduler(pg_client, redis_client2, object_storage2).await
            {
                tracing::error!("Scheduler failed with: {e}");
            }
        });

        Ok(Self { serve, port })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    /// This function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.serve.with_graceful_shutdown(shutdown_signal()).await?;
        Ok(())
    }

    /// Configure `Server`.
    fn build_server(
        base_url: &str,
        listener: TcpListener,
        pg_pool: Pool,
        redis_pool: RedisPool,
        redis_client: RedisClient,
        object_storage: ObjectStorage,
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
        let session_store =
            axum_login::tower_sessions::RedisStore::new(redis_client);
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

        let mut app = Router::new()
            .nest("/api/protected", protected_router())
            .nest("/api/open", open_router())
            .route("/api/health_check", routing::get(health_check))
            .route("/api/signup", routing::post(auth::signup::signup))
            .route(
                "/api/confirm_user_account",
                routing::get(auth::confirm_account::confirm),
            )
            .with_state(app_state.clone())
            .merge(auth::login::login_router(app_state.clone()))
            .layer(crate::middleware::map_response::BadRequestIntoJsonLayer)
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

        if let Ok(e) = std::env::var("ENVIRONMENT") {
            if e.eq("development") {
                let origins = [
                    "http://127.0.0.1:5173".parse().unwrap(),
                    "http://localhost:5173".parse().unwrap(),
                ];
                let cors = CorsLayer::new().allow_origin(origins);
                app = app.layer(cors);
                app = app.merge(
                    SwaggerUi::new("/swagger-ui")
                        .url("/api-docs/openapi.json", ApiDoc::openapi()),
                );
                app = app.nest("/api", development::user_router());
            }
        }

        axum::serve(listener, app)
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(
            tokio::signal::unix::SignalKind::terminate(),
        )
        .expect("failed to install signal handler")
        .recv()
        .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
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

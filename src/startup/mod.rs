use axum::error_handling::HandleErrorLayer;
use axum::routing;
use axum::serve::Serve;
use axum::BoxError;
use axum::Router;
use axum_login::AuthManagerLayerBuilder;
use deadpool_postgres::Manager;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::Pool;

use http::StatusCode;
use secrecy::ExposeSecret;

use time::Duration;
use tokio::net::TcpListener;
use tokio_postgres::NoTls;
use tower::ServiceBuilder;

use crate::config::DatabaseSettings;
use crate::config::Settings;
use crate::email_client::EmailClient;
use crate::routes::health_check::health_check;
use crate::routes::open::open_router;
use crate::routes::private::private_router;
pub mod db_migration;

/// This is a central type of our codebase. `Application` type builds server
/// for both production and testing purposes.
pub struct Application {
    port: u16,
    serve: Serve<Router, Router>,
}

/// Shareable type, we insert it to the main `Router` as state,
/// at the launch stage.
#[derive(Clone)]
pub struct AppState {
    pub base_url: String,
    pub pool: Pool,
    pub email_client: EmailClient,
}

impl Application {
    /// Build a new server.
    ///
    /// This functions builds a new `Application` with given configuration.
    /// It also configures a pool of connections to the PostgreSQL database.
    pub async fn build(
        configuration: Settings,
    ) -> Result<Application, anyhow::Error> {
        let postgres_connection =
            get_postgres_connection_pool(&configuration.database);

        // db_migration::run_migration(&postgres_connection).await;

        let timeout = configuration.email_client.timeout_millis();

        let sender_email = configuration.email_client.sender()?;

        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
            configuration.email_delivery_service,
        )?;

        let address =
            format!("{}:{}", configuration.app_addr, configuration.app_port);
        tracing::info!("running on {} address", address);
        let listener = TcpListener::bind(address).await?;
        let port = listener.local_addr()?.port();

        let serve = Self::build_server(
            &configuration.app_base_url,
            listener,
            postgres_connection,
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
        pool: Pool,
        email_client: EmailClient,
    ) -> Serve<Router, Router> {
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let session_store = axum_login::tower_sessions::MemoryStore::default();
        let session_layer =
            axum_login::tower_sessions::SessionManagerLayer::new(session_store)
                .with_secure(true)
                .with_expiry(axum_login::tower_sessions::Expiry::OnInactivity(
                    Duration::days(1),
                ));

        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = crate::auth::users::UserBackend::new(pool.clone());
        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|e: BoxError| async move {
                tracing::error!("GOT HANDLE ERROR: {}", e);
                StatusCode::BAD_REQUEST
            }))
            .layer(
                AuthManagerLayerBuilder::new(backend, session_layer).build(),
            );

        // We do not wrap pool into arc because internally it alreaday has an
        // `Arc`, and copying is cheap.
        let app_state = AppState {
            pool: pool.clone(),
            email_client,
            base_url: base_url.to_string(),
        };

        let app = Router::new()
            .nest("/api/private", private_router())
            .nest("/api/open", open_router())
            .route("/health_check", routing::get(health_check))
            .with_state(app_state)
            .merge(crate::auth::login_router())
            .layer(auth_service);

        axum::serve(listener, app)
    }
}

/// Returns a connection pool to the PostgreSQL database.
pub fn get_postgres_connection_pool(configuration: &DatabaseSettings) -> Pool {
    let pg_config = get_pg_conf(configuration);
    // let connector = get_ssl_connector();
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

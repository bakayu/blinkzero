use crate::configuration::{DatabaseSettings, Settings};
use crate::handlers::{
    create_blink, get_action_json, get_action_metadata, health, post_action_transaction,
};
use axum::{
    Router,
    http::{Method, header},
    routing::{get, post},
};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use std::sync::Arc;
use tokio::task::JoinHandle;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::PeerIpKeyExtractor,
};
use tower_http::cors::{Any, CorsLayer};

pub struct Application {
    server_task: JoinHandle<Result<(), std::io::Error>>,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;

        let server_task = run(listener, connection_pool, false).await?;

        tracing::info!("Server running at : {}", address);

        Ok(Self { server_task })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server_task.await?
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    enable_rate_limiting: bool,
) -> Result<JoinHandle<Result<(), std::io::Error>>, std::io::Error> {
    listener.set_nonblocking(true)?;
    let tokio_listener = tokio::net::TcpListener::from_std(listener)?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any)
        .expose_headers([
            header::HeaderName::from_static("x-action-version"),
            header::HeaderName::from_static("x-blockchain-ids"),
        ]);

    let app = if enable_rate_limiting {
        let governor_conf = Arc::new(
            GovernorConfigBuilder::default()
                .per_second(60)
                .burst_size(5)
                .key_extractor(PeerIpKeyExtractor)
                .finish()
                .unwrap(),
        );

        Router::new()
            .route("/health", get(health))
            .route("/.well-known/actions.json", get(get_action_json))
            .route("/actions.json", get(get_action_json))
            .route(
                "/api/blinks",
                post(create_blink).layer(GovernorLayer::new(governor_conf)),
            )
            .route(
                "/api/actions/{id}",
                get(get_action_metadata).post(post_action_transaction),
            )
            .layer(cors)
            .with_state(db_pool)
    } else {
        Router::new()
            .route("/health", get(health))
            .route("/.well-known/actions.json", get(get_action_json))
            .route("/actions.json", get(get_action_json))
            .route("/api/blinks", post(create_blink))
            .route(
                "/api/actions/{id}",
                get(get_action_metadata).post(post_action_transaction),
            )
            .layer(cors)
            .with_state(db_pool)
    };

    let handle = tokio::spawn(async move { axum::serve(tokio_listener, app).await });

    Ok(handle)
}

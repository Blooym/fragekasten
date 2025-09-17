mod database;
mod discord;
mod routes;

use crate::database::Database;
use anyhow::Result;
use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, header},
    middleware::{self as axum_middleware, Next},
    routing::{get, post},
};
use axum_client_ip::ClientIpSource;
use clap::Parser;
use dotenvy::dotenv;
use reqwest::{Client, Url};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, signal};
use tower_http::{
    catch_panic::CatchPanicLayer,
    normalize_path::NormalizePathLayer,
    trace::{self, TraceLayer},
};
use tracing::{Level, info};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct StartupConfig {
    /// Internet socket address that the server should be ran on.
    #[arg(
        long = "address",
        env = "FRAGEKASTEN_ADDRESS",
        default_value = "127.0.0.1:6251"
    )]
    address: SocketAddr,

    /// SQLite database connection string to use for temporarily storing questions.
    #[clap(long = "database-url", env = "DATABASE_URL")]
    database_url: String,

    /// ClientIpSource to use when obtaining the users's IP address. Defaults to "ConnectInfo", although this will not work when behind a reverse proxy and should be changed accordingly.
    ///
    /// See https://github.com/imbolc/axum-client-ip/blob/6d970edce4f7f0d1782e328fe688e021c42f1f3e/README.md#configurable-vs-specific-extractors for details on accepted values.
    #[clap(
        long = "ip-source",
        env = "FRAGEKASTEN_IP_SOURCE",
        default_value = "ConnectInfo"
    )]
    ip_source: ClientIpSource,

    /// Discord Webhook URL to send asked questions to.
    #[clap(long = "discord-webhook-url", env = "FRAGEKASTEN_DISCORD_WEBHOOK_URL")]
    discord_webhook_url: Url,

    /// Discord User ID (not name) to mention when sending asked questions.
    #[clap(long = "discord-user-id", env = "FRAGEKASTEN_DISCORD_USER_ID")]
    discord_user_id: usize,

    /// The name of the owner of the page - you probably want to use your online username.
    #[clap(long = "page-owner-name", env = "FRAGEKASTEN_PAGE_OWNER_NAME")]
    page_owner_name: String,

    /// The title to use for the questions page.
    #[clap(long = "page-title", env = "FRAGEKASTEN_PAGE_TITLE")]
    page_title: String,

    /// The description to use for the questions page. Supports inline HTML tags.
    #[clap(long = "page-description", env = "FRAGEKASTEN_PAGE_DESCRIPTION")]
    page_description: String,

    /// The minimum length a question is allowed to be.
    #[clap(
        long = "page-question-min-length",
        env = "FRAGEKASTEN_PAGE_QUESTION_MIN_LENGTH",
        default_value_t = 15
    )]
    page_question_min_length: usize,

    /// The maximum length a question is allowed to be.
    #[clap(
        long = "page-question-max-length",
        env = "FRAGEKASTEN_PAGE_QUESTION_MAX_LENGTH",
        default_value_t = 300
    )]
    page_question_max_length: usize,

    /// The placeholder text to use in the question ask box, this can be anything you want.
    #[clap(
        long = "page-question-placeholder",
        env = "FRAGEKASTEN_PAGE_QUESTION_PLACEHOLDER",
        default_value = "Would you like to hold hands in the rain together?"
    )]
    page_question_placeholder: String,
}

#[derive(Clone)]
struct AppState {
    database: Arc<Database>,
    reqwest_client: Arc<Client>,
    discord_webhook_url: Url,
    discord_user_id: usize,
    page_owner_name: String,
    page_title: String,
    page_description: String,
    page_question_min_length: usize,
    page_question_max_length: usize,
    page_question_placeholder: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load startup configuration.
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")))
        .init();
    let args = StartupConfig::parse();

    // Prepare server.
    let state = AppState {
        database: Arc::new(Database::new(&args.database_url).await?),
        reqwest_client: Arc::new(Client::new()),
        discord_webhook_url: args.discord_webhook_url,
        discord_user_id: args.discord_user_id,
        page_owner_name: args.page_owner_name,
        page_title: args.page_title,
        page_description: args.page_description,
        page_question_min_length: args.page_question_min_length,
        page_question_max_length: args.page_question_max_length,
        page_question_placeholder: args.page_question_placeholder,
    };
    Database::spawn_cleanup_task(state.database.clone());
    let router = Router::new()
        .route("/", get(routes::serve_index))
        .route("/index.html", get(routes::serve_index))
        .route("/api/asks", post(routes::api::asks::add_ask))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CatchPanicLayer::new())
        .layer(args.ip_source.clone().into_extension())
        .layer(axum_middleware::from_fn(
            async |req: Request, next: Next| {
                let mut res = next.run(req).await;
                let res_headers = res.headers_mut();
                res_headers.insert(
                    header::SERVER,
                    HeaderValue::from_static(env!("CARGO_PKG_NAME")),
                );
                res_headers.insert("X-Robots-Tag", HeaderValue::from_static("none"));
                res
            },
        ))
        .with_state(state);

    // Start server.
    let tcp_listener = TcpListener::bind(&args.address).await?;
    info!(
        "Internal server started - listening on: http://{}",
        args.address,
    );
    axum::serve(
        tcp_listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

// https://github.com/tokio-rs/axum/blob/15917c6dbcb4a48707a20e9cfd021992a279a662/examples/graceful-shutdown/src/main.rs#L55
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

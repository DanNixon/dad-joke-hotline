mod handlers;
mod jambonz;
mod voice;

use axum::{routing::post, Router};
use clap::Parser;
use metrics::describe_counter;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(long, env, default_value = "127.0.0.1:8000")]
    webhook_address: SocketAddr,

    #[arg(long, env, default_value = "127.0.0.1:9090")]
    observability_address: SocketAddr,
}

#[derive(Clone)]
struct AppState {
    http_client: reqwest::Client,
}

const METRIC_CALLS_NAME: &str = "dadjokehotline_calls";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt::init();

    // Set up metrics server
    let builder = PrometheusBuilder::new();
    builder
        .with_http_listener(cli.observability_address)
        .install()?;

    describe_counter!(METRIC_CALLS_NAME, "Total number of calls received");

    // Shared application state
    let state = AppState {
        http_client: reqwest::Client::new(),
    };

    let app = Router::new()
        .route("/call_status", post(handlers::call_status))
        .route("/call", post(handlers::call_incoming))
        .with_state(state);

    info!("Let comedy commence!");

    let listener = TcpListener::bind(&cli.webhook_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

use crate::modules::db::connection::get_connection;
use crate::modules::env::env::EnvConfig;
use crate::modules::router::router::router;
use crate::modules::tracer::init::init_tracing;
use envconfig::Envconfig;
use opentelemetry::global;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, warn};

mod domain;
mod modules;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EnvConfig::init_from_env()?;

    // Initialize tracing and OpenTelemetry
    let metrics = init_tracing(config.clone()).await?;
    let db_pool = get_connection(config.clone()).await?;

    // Get address to listen on
    let addr = format!("{}:{:?}", config.http_host, config.http_port).parse::<SocketAddr>()?;
    let listener = TcpListener::bind(addr).await?;
    debug!(config.http_port, config.http_host, "Will start");

    // Start the server
    warn!("Server running on http://{:?}", listener.local_addr()?);
    let app = router(metrics, db_pool).await?;
    axum::serve(listener, app).await?;

    // Shutdown the tracer provider
    global::shutdown_tracer_provider();
    Ok(())
}
use crate::modules::env::env::EnvConfig;
use crate::modules::router::router::router;
use crate::modules::tracer::init::init_tracing;
use envconfig::Envconfig;
use opentelemetry::global;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{debug, info};

mod modules;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EnvConfig::init_from_env()?;

    // Initialize tracing and OpenTelemetry
    let metrics = init_tracing(config.clone())?;

    // Run the server
    let addr = format!("{}:{:?}", config.http_host, config.http_port).parse::<SocketAddr>()?;
    debug!(
        config.http_port,
        config.http_host, "Will listen on {:?}", addr
    );

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind(addr).await?;
    info!("Server running on http://{:?}", listener.local_addr()?);
    axum::serve(listener, router(metrics).await?).await?;

    global::shutdown_tracer_provider();
    Ok(())
}
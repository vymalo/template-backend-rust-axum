use crate::modules::env::env::EnvConfig;
use crate::modules::router::router::router;
use crate::modules::tracer::init::init_tracing;
use axum::http::Method;
use envconfig::Envconfig;
use opentelemetry::global;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::compression::predicate::{NotForContentType, SizeAbove};
use tower_http::compression::{CompressionLayer, DefaultPredicate, Predicate};
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, info};

mod modules;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = EnvConfig::init_from_env()?;

    // Initialize tracing and OpenTelemetry
    let metrics = init_tracing(config.clone())?;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        // allow requests from any origin
        .allow_origin(Any);
    let compression_predicate = DefaultPredicate::new()
        .and(NotForContentType::new("application/cbor"))
        .and(SizeAbove::new(0));
    let compression_layer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
        .compress_when(compression_predicate);

    // Run the server
    let addr = format!("{}:{:?}", config.http_host, config.http_port).parse::<SocketAddr>()?;
    debug!(
        config.http_port,
        config.http_host, "Will listen on {:?}", addr
    );

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind(addr).await?;
    info!("Server running on http://{:?}", listener.local_addr()?);
    axum::serve(
        listener,
        router(metrics).await?.layer(compression_layer).layer(cors),
    )
    .await?;

    global::shutdown_tracer_provider();
    Ok(())
}
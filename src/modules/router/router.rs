use crate::services::health::health::health;
use crate::services::metrics::handler::metrics_handler;
use crate::services::welcome_home::handler::{welcome_home_handler, welcome_post_handler};
use axum::routing::{get, post};
use axum::Router;
use axum_otel_metrics::HttpMetricsLayer;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

pub async fn router(metrics: HttpMetricsLayer) -> Result<Router, Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(welcome_home_handler))
        .route("/", post(welcome_post_handler))
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(metrics);

    // Add health and metrics endpoints
    let app = app
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler));

    Ok(app)
}
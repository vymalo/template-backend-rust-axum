use crate::modules::rpc::handler::rpc_handler;
use crate::modules::rpc::rpc_options::rpc_options;
use crate::services::health::health::health;
use crate::services::metrics::handler::metrics_handler;
use crate::services::welcome_home::handler::{welcome_home_handler, welcome_post_handler};
use axum::routing::{get, options, post};
use axum::Router;
use axum_otel_metrics::HttpMetricsLayer;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

pub async fn router(metrics: HttpMetricsLayer) -> Result<Router, Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(welcome_home_handler))
        .route("/", post(welcome_post_handler))
        .route(
            "/rpc.:method_name",
            options(rpc_options)
                .get(rpc_handler)
                .post(rpc_handler)
                .patch(rpc_handler)
                .put(rpc_handler),
        )
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(metrics);

    // Add health and metrics endpoints
    let app = app
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler));

    Ok(app)
}
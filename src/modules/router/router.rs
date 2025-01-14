use crate::modules::router::middlewares::apply_common_middlewares;
use crate::services::health::health::health;
use crate::services::metrics::handler::metrics_handler;
use crate::services::todos::handler::TodoServiceBuilder;
use anyhow::Result;
use axum::routing::get;
use axum::Router;
use axum_otel_metrics::HttpMetricsLayer;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;
use gen_server::server::new;

pub async fn router(metrics: HttpMetricsLayer, pool: Pool<AsyncPgConnection>) -> Result<Router> {
    let todo_service = TodoServiceBuilder::default().pool(pool).build()?;

    // Create the main router
    let app = new(todo_service)
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(metrics);

    // Add health and metrics endpoints
    let app = app
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler));

    let router = apply_common_middlewares(app);
    Ok(router)
}
use crate::modules::db::state::DbStateBuilder;
use crate::modules::router::middlewares::apply_common_middlewares;
use crate::modules::rpc::handler::rpc_handler;
use crate::modules::rpc::rpc_options::rpc_options;
use crate::services::health::health::health;
use crate::services::metrics::handler::metrics_handler;
use crate::services::welcome_home::handler::{welcome_home_handler, welcome_post_handler};
use app_models::Result;
use axum::routing::{get, options, post};
use axum::Router;
use axum_otel_metrics::HttpMetricsLayer;
use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;

pub async fn router(metrics: HttpMetricsLayer, pool: Pool<AsyncPgConnection>) -> Result<Router> {
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

    let db_state = DbStateBuilder::default().pool(pool).build()?;

    // Add health and metrics endpoints
    let app = app
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler))
        .with_state(db_state);

    let router = apply_common_middlewares(app);
    Ok(router)
}
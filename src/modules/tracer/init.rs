use crate::modules::env::env::EnvConfig;
use axum_otel_metrics::{HttpMetricsLayer, HttpMetricsLayerBuilder};
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::metrics::SdkMeterProvider;
use opentelemetry_sdk::runtime::Tokio;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, TracerProvider};
use opentelemetry_sdk::Resource;
use std::time::Duration;
use opentelemetry::trace::TracerProvider as _;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn init_tracing(
    EnvConfig {
        otlp_metric_endpoint,
        otlp_span_endpoint,
        otlp_service_name,
        otlp_version,
        log_level,
        ..
    }: EnvConfig,
) -> Result<HttpMetricsLayer, Box<dyn std::error::Error>> {
    // Set up OTLP exporter
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_span_endpoint) // Tempo or OTLP endpoint
        .with_timeout(Duration::from_secs(3))
        .build()?;

    let tracer_provider = TracerProvider::builder()
        .with_batch_exporter(exporter, Tokio)
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_max_events_per_span(16)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            otlp_service_name.clone(),
        )]))
        .build();

    global::set_tracer_provider(tracer_provider.clone());

    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(otlp_metric_endpoint)
        .with_protocol(Protocol::Grpc)
        .with_timeout(Duration::from_secs(3))
        .build()?;

    let reader = opentelemetry_sdk::metrics::PeriodicReader::builder(exporter, Tokio)
        .with_interval(Duration::from_secs(3))
        .with_timeout(Duration::from_secs(10))
        .build();

    let provider = SdkMeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            otlp_service_name.clone(),
        )]))
        .build();

    let prometheus_exporter = opentelemetry_prometheus::exporter()
        .with_registry(prometheus::default_registry().clone())
        .build()?;
    let metrics = HttpMetricsLayerBuilder::new()
        .with_service_name(otlp_service_name)
        .with_service_version(otlp_version)
        .with_metric_reader(prometheus_exporter)
        .build();

    // Set up Tracing with OpenTelemetry
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_level)) // Adjust log level
        .with(tracing_subscriber::fmt::layer()) // Log to console
        .with(
            tracing_opentelemetry::layer()
                .with_error_records_to_exceptions(true)
                .with_tracer(tracer_provider.tracer("tracer-name")),
        )
        .with(tracing_opentelemetry::MetricsLayer::new(provider))
        .init();

    Ok(metrics)
}
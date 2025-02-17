use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct EnvConfig {
    #[envconfig(
        from = "DB_URL"
    )]
    pub db_url: String,

    #[envconfig(from = "DB_MAX_THREAD_POOL", default = "10")]
    pub db_max_thread_pool: usize,

    #[envconfig(from = "HTTP_HOST", default = "0.0.0.0")]
    pub http_host: String,

    #[envconfig(from = "HTTP_PORT", default = "3000")]
    pub http_port: u16,

    #[envconfig(from = "LOG_LEVEL", default = "debug")]
    pub log_level: String,

    #[envconfig(from = "OTLP_SPAN_ENDPOINT", default = "http://localhost:4317")]
    pub otlp_span_endpoint: String,

    #[envconfig(
        from = "OTLP_METRIC_ENDPOINT",
        default = "http://localhost:4318/v1/metrics"
    )]
    pub otlp_metric_endpoint: String,

    #[envconfig(from = "OTLP_SERVICE_NAME", default = "rust-app-example")]
    pub otlp_service_name: String,

    #[envconfig(from = "OTLP_VERSION", default = "0.1.0")]
    pub otlp_version: String,
}
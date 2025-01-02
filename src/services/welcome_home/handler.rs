use tracing::{info, instrument};

#[instrument]
pub async fn welcome_home_handler() -> &'static str {
    info!(name: "welcome", "Handling GET request");
    "Welome Home"
}

#[instrument]
pub async fn welcome_post_handler(payload: String) -> String {
    info!(%payload, "Handling GET request");
    format!("Received: {}", payload)
}
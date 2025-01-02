use prometheus::{Encoder, TextEncoder};

pub async fn metrics_handler() -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
    // return metrics
    String::from_utf8(buffer).unwrap()
}
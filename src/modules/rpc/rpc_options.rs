use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;

pub async fn rpc_options() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert(
        "Allow",
        "GET,POST,PUT,PATCH,DELETE,HEAD,OPTIONS".parse().unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        "GET,POST,PUT,PATCH,DELETE,HEAD,OPTIONS".parse().unwrap(),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        "Content-Type".parse().unwrap(),
    );
    headers.insert("Access-Control-Max-Age", "86400".parse().unwrap());
    headers.insert(
        "Cache-Control",
        "no-store, no-cache, must-revalidate, max-age=0"
            .parse()
            .unwrap(),
    );
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Expires", "0".parse().unwrap());
    headers.insert("Vary", "Origin".parse().unwrap());
    headers.insert("Vary", "Access-Control-Request-Method".parse().unwrap());
    headers.insert("Vary", "Access-Control-Request-Headers".parse().unwrap());

    (StatusCode::OK, headers)
}
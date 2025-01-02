use axum::http::header::GetAll;
use axum::http::HeaderValue;

pub fn is_accept_json(all: GetAll<HeaderValue>) -> bool {
    for x in all.iter() {
        if x == "application/cbor" {
            return false;
        }

        if x == "application/json" {
            return true;
        }
    }

    false
}
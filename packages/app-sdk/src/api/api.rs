use app_models::api::method_name::MethodName;
use app_models::utils::converter::{deserialize_payload, serialize_payload};
use app_models::Result;
use bytes::Bytes;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

pub async fn make_api_call<P: Serialize, R: for<'a> Deserialize<'a>>(
    url: String,
    method_name: MethodName,
    payload: P,
) -> Result<R> {
    let bytes = serialize_payload(false, &payload).map(|b| JsValue::from(b.to_vec()))?;

    let resp = Request::post(&format!("{}/rpc.{:?}", url, method_name))
        .header("Content-Type", "application/cbor")
        .header("Accept", "application/cbor")
        .body(bytes)?;

    let vec = resp.binary().await?;

    deserialize_payload(false, Bytes::from(vec))
}
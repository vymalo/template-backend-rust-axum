use crate::modules::rpc::is_accept_json::is_accept_json;
use crate::services::schools::handler::{get_schools, get_single_school};
use anyhow::Result;
use app_models::api::api::ApiResponse;
use app_models::api::method_name::MethodName;
use app_models::errors::codes::ErrorCode;
use app_models::errors::model::ApiErrorBuilder;
use app_models::utils::converter::{deserialize_payload, serialize_payload};
use axum::body::Bytes;
use axum::extract::Path;
use axum::http::header::HeaderMap;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::str::FromStr;
use tracing::instrument;
use tracing::log::error;

#[instrument]
pub async fn rpc_handler(
    Path(method_name): Path<String>,
    headers: HeaderMap,
    body: Option<Bytes>,
) -> impl IntoResponse {
    let json = is_accept_json(headers.get_all("accept"));


    let response = rpc_split(json, method_name, body).await;
    let response = response.unwrap_or_else(|err| {
        error!("Unhandled error {:?}", err);
        let error = ApiErrorBuilder::default()
            .code(ErrorCode::Internal)
            .message(format!("Unhandled error {:?}", err))
            .status(500)
            .build()
            .unwrap();
        let response = ApiResponse::Error(error);
        serialize_payload(json, &response).unwrap()
    });

    let mut headers = HeaderMap::new();
    if json {
        headers.insert("content-type", "application/json".parse().unwrap());
    } else {
        headers.insert("content-type", "application/cbor".parse().unwrap());
    }
    headers.insert("content-length", response.len().to_string().parse().unwrap());
    (StatusCode::OK, headers, response)
}

pub async fn rpc_split(json: bool, method_name: String, body: Option<Bytes>) -> Result<Bytes> {
    match MethodName::from_str(&method_name) {
        Ok(MethodName::GetSchool) => {
            let body = body.unwrap();
            let body = deserialize_payload(json, body)?;
            let result = get_single_school(body).await;
            let response = ApiResponse::GetSchool(result);
            Ok(serialize_payload(json, &response)?)
        }
        Ok(MethodName::GetAllSchool) => {
            let body = body.unwrap();
            let body = deserialize_payload(json, body)?;
            let result = get_schools(body).await;
            let response = ApiResponse::GetAllSchool(result);
            Ok(serialize_payload(json, &response)?)
        }
        Err(err) => {
            error!("Method {:?} not found", err);
            let error = ApiErrorBuilder::default()
                .code(ErrorCode::WrongMethod(method_name))
                .message(format!("Method {:?} not found", err))
                .status(400)
                .build()?;
            let response = ApiResponse::Error(error);
            Ok(serialize_payload(json, &response)?)
        }
    }
}
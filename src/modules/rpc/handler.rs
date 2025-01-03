use crate::modules::db::state::DbState;
use crate::modules::rpc::is_accept_json::is_accept_json;
use crate::services::schools::handler::{get_schools, get_single_school};
use app_models::api::api::ApiResponse;
use app_models::api::method_name::MethodName;
use app_models::errors::codes::ErrorCode;
use app_models::errors::model::ApiErrorBuilder;
use app_models::utils::converter::{deserialize_payload, serialize_payload};
use app_models::Result;
use axum::body::Bytes;
use axum::extract::{Path, Query, RawQuery, State};
use axum::http::header::HeaderMap;
use axum::http::{Method, StatusCode};
use axum::response::IntoResponse;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::AsyncPgConnection;
use std::str::FromStr;
use tracing::instrument;
use tracing::log::error;

#[instrument]
pub async fn rpc_handler(
    method: Method,
    Path(method_name): Path<String>,
    headers: HeaderMap,
    RawQuery(query): RawQuery,
    State(db_state): State<DbState>,
    body: Option<Bytes>,
) -> impl IntoResponse {
    let json = is_accept_json(headers.get_all("accept"));

    let response = rpc_split(json, method, db_state.pool, method_name, query, body).await;
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
    headers.insert(
        "content-length",
        response.len().to_string().parse().unwrap(),
    );
    (StatusCode::OK, headers, response)
}

pub async fn rpc_split(
    json: bool,
    _method: Method,
    pool: Pool<AsyncPgConnection>,
    method_name: String,
    _query: Option<String>,
    body: Option<Bytes>,
) -> Result<Bytes> {
    match MethodName::from_str(&method_name) {
        Ok(MethodName::GetSchool) => {
            let body = body.unwrap();
            let body = deserialize_payload(json, body)?;
            let result = get_single_school(body, pool).await?;
            let response = ApiResponse::GetSchool(result);
            Ok(serialize_payload(json, &response)?)
        }
        Ok(MethodName::GetAllSchool) => {
            let body = body.unwrap();
            let body = deserialize_payload(json, body)?;
            let result = get_schools(body, pool).await?;
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
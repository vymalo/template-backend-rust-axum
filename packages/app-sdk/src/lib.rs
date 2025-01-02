mod api;
mod utils;

use crate::api::api::make_api_call;
use app_models::api::method_name::MethodName;
use app_models::models::school::School;
use serde_wasm_bindgen::{from_value, to_value};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use app_models::api::api::{Page, Pagination};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub async fn make_rpc_request(
    url: JsValue,
    method_name: JsValue,
    request_data: JsValue,
) -> Result<JsValue, JsValue> {
    let method_name: MethodName = from_value(method_name)?;
    let url: String = from_value(url)?;

    log(&format!("Should send: {:?}", method_name));

    match method_name {
        MethodName::GetSchool => {
            let id: String = from_value(request_data)?;
            let response: School = make_api_call(url, method_name, id).await.unwrap();
            let result = to_value(&response)?;
            Ok(result)
        }
        MethodName::GetAllSchool => {
            let page: Pagination = from_value(request_data)?;
            let response: Page<School> = make_api_call(url, method_name, page).await.unwrap();
            let result = to_value(&response)?;
            Ok(result)
        }
    }
}
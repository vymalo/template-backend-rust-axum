use derive_builder::Builder;
use crate::errors::model::ApiError;
use crate::models::school::School;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum ApiResponse {
    GetSchool(School),
    GetAllSchool(Page<School>),
    Error(ApiError),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Builder)]
pub struct Page<T> {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub items: Vec<T>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Builder)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}
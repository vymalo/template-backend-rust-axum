use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use crate::errors::codes::ErrorCode;

#[derive(Debug, Clone, Serialize, Deserialize, Builder, PartialEq)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    pub status: u16,
}
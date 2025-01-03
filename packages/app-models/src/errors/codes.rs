use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ErrorCode {
    WrongMethod(String),
    Internal,
}
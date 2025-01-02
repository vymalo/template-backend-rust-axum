use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("Found no username in {0}")]
    EmptyUsername(String),

    #[error("Encoding error with JSON: {0}")]
    SerdeJsonError(serde_json::Error),

    #[error("Encoding error with CBOR: {0}")]
    SerdeCborError(serde_cbor::Error),
}
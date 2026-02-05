use thiserror::Error;

#[derive(Debug, Error)]
pub enum FoxError {
    #[error("FoxCloud: {0}")]
    FoxCloud(String),
    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("JsonParseError: {0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("ChronoParseError: {0}")]
    ChronoParseError(#[from] chrono::format::ParseError),
}
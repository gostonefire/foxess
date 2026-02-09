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
    #[error("SettingParseError: setting={setting}, value={value}, error={error}")]
    SettingParseError {
        setting: &'static str,
        value: String,
        error: String,
    },
    #[error("VariableConversionError: variable={variable}, value={value}, error={error}")]
    VariableConversionError {
        variable: &'static str,
        value: String,
        error: String,
    },
    #[error("UnallowedSetSetting")]
    UnallowedSetSetting,
    #[error("VariableNotFoundError: variable={variable}")]
    VariableNotFoundError { variable: &'static str },
    #[error("ScheduleBuildError: {0}")]
    ScheduleBuildError(String),
}
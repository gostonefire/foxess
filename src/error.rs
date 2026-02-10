//! Error types and result handling for the FoxESS client.
//!
//! This module provides the [`FoxError`] enum, which encapsulates all potential
//! errors that can occur when interacting with the FoxCloud API or processing its data.

use thiserror::Error;

/// Custom error types for the FoxESS client.
#[derive(Debug, Error)]
pub enum FoxError {
    /// An error returned by the FoxCloud API.
    #[error("FoxCloud: {0}")]
    FoxCloud(String),
    /// An error occurred during an HTTP request.
    #[error("ReqwestError: {0}")]
    ReqwestError(#[from] reqwest::Error),
    /// An error occurred while parsing JSON.
    #[error("JsonParseError: {0}")]
    JsonParseError(#[from] serde_json::Error),
    /// An error occurred while parsing a date or time.
    #[error("ChronoParseError: {0}")]
    ChronoParseError(#[from] chrono::format::ParseError),
    /// An error occurred while parsing a setting value.
    #[error("SettingParseError: setting={setting}, value={value}, error={error}")]
    SettingParseError {
        /// The name of the setting.
        setting: &'static str,
        /// The raw value that failed to parse.
        value: String,
        /// The error message.
        error: String,
    },
    /// An error occurred while converting a variable value.
    #[error("VariableConversionError: variable={variable}, value={value}, error={error}")]
    VariableConversionError {
        /// The name of the variable.
        variable: &'static str,
        /// The raw value that failed to convert.
        value: String,
        /// The error message.
        error: String,
    },
    /// The requested variable was not found in the API response.
    #[error("VariableNotFoundError: variable={variable}")]
    VariableNotFoundError {
        /// The name of the variable.
        variable: &'static str,
    },
    /// An error occurred while building a schedule.
    #[error("ScheduleBuildError: {0}")]
    ScheduleBuildError(String),
}
//! Error types and result handling for the FoxESS client.
//!
//! This module provides the [`FoxError`] enum, which encapsulates all potential
//! errors that can occur when interacting with the FoxCloud API or processing its data.

use thiserror::Error;

/// Custom error types for the FoxESS client.
///
/// This enum is `#[non_exhaustive]`: new variants may be added in future releases,
/// so `match` expressions over it must include a wildcard arm.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FoxError {
    /// An application-level error reported by the FoxCloud API.
    ///
    /// The request reached FoxCloud and it answered, but rejected it with a
    /// non-zero `errno`. Note that these codes are distinct from the inverter
    /// fault codes returned by
    /// [`get_error_code_information`](crate::Fox::get_error_code_information).
    #[error("FoxCloud: errno={errno}, msg={msg}")]
    FoxCloud {
        /// The `errno` field from the FoxCloud response.
        errno: u32,
        /// The `msg` field from the FoxCloud response.
        msg: String,
    },
    /// The FoxCloud API replied with a non-success HTTP status.
    ///
    /// This is a transport-level failure: the response either carried no
    /// FoxCloud error payload at all, or one that could not be parsed.
    #[error("HttpStatus: status={status}, body={body}")]
    HttpStatus {
        /// The HTTP status code of the response.
        status: u16,
        /// The response body, truncated if long.
        body: String,
    },
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

impl FoxError {
    /// Returns the FoxCloud `errno`, if this was an application-level API error.
    ///
    /// Use this to react to specific FoxCloud error codes without matching on
    /// the variant or inspecting the formatted message.
    ///
    /// # Returns
    /// * `Option<u32>` - The `errno`, or `None` for any other error kind.
    pub fn errno(&self) -> Option<u32> {
        match self {
            FoxError::FoxCloud { errno, .. } => Some(*errno),
            _ => None,
        }
    }

    /// Returns the HTTP status code, if the request failed at the transport level.
    ///
    /// # Returns
    /// * `Option<u16>` - The status code, or `None` for any other error kind.
    pub fn http_status(&self) -> Option<u16> {
        match self {
            FoxError::HttpStatus { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// Returns whether retrying the same request might plausibly succeed.
    ///
    /// Covers server-side failures (HTTP 5xx), rate limiting (HTTP 429), and
    /// connection or timeout failures. Application-level FoxCloud errors and
    /// local parsing errors are never treated as transient, since retrying an
    /// identical request cannot change their outcome.
    ///
    /// # Returns
    /// * `bool` - `true` if a retry is worth attempting.
    pub fn is_transient(&self) -> bool {
        match self {
            FoxError::HttpStatus { status, .. } => *status == 429 || *status >= 500,
            FoxError::ReqwestError(e) => e.is_timeout() || e.is_connect(),
            _ => false,
        }
    }
}
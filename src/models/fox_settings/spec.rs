//! This module hosts the [SettingSpec] trait that can be implemented for marker structs to be used together with typed get functions.
//!

use crate::{FoxError, FoxSettings};

/// A trait defining the specification for a FoxESS setting.
///
/// Types implementing this trait define how to parse a raw string value from the API
/// into a specific Rust type.
pub trait SettingSpec {
    /// The Rust type that the setting value will be parsed into.
    type Value;
    /// The `FoxSettings` variant associated with this specification.
    const SETTING: FoxSettings;
    /// Parses the raw string value into the associated `Value` type.
    ///
    /// # Arguments
    /// * `raw` - The raw string value received from the FoxESS API.
    ///
    /// # Returns
    /// * `Result<Self::Value, FoxError>` - The parsed value or an error if parsing fails.
    fn parse(raw: String) -> Result<Self::Value, FoxError>;
}

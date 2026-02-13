//! This module hosts the [VariableSpec] trait that can be implemented for marker structs to be used together with typed get functions.
//!

use crate::{FoxError, FoxVariables};

/// A trait defining the specification for a FoxESS variable.
///
/// Types implementing this trait define how to convert a raw floating-point value
/// from the API into a specific Rust type.
pub trait VariableSpec {
    /// The Rust type that the variable value will be converted into.
    type Value;
    /// The `FoxVariables` variant associated with this specification.
    const VARIABLE: FoxVariables;
    /// Converts the raw `f64` value into the associated `Value` type.
    ///
    /// # Arguments
    /// * `raw` - The raw `f64` value received from the FoxESS API.
    ///
    /// # Returns
    /// * `Result<Self::Value, FoxError>` - The converted value or an error if conversion fails.
    fn into(raw: f64) -> Result<Self::Value, FoxError>;
}

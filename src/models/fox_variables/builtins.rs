//! This module hosts builtin marker structs to be used together with typed get functions.
//! These marker structs are used to implement the [VariableSpec] trait for the builtin variables.
//!

use crate::{FoxError, FoxVariables};
use crate::fox_variables::VariableSpec;

/// Specification for the `PvPower` variable.
pub struct PvPower;
impl VariableSpec for PvPower {
    type Value = f64;
    const VARIABLE: FoxVariables = FoxVariables::PvPower;

    fn into(raw: f64) -> Result<Self::Value, FoxError> {
        Ok(raw)
    }
}

/// Specification for the `LoadsPower` variable.
pub struct LoadsPower;
impl VariableSpec for LoadsPower {
    type Value = f64;
    const VARIABLE: FoxVariables = FoxVariables::LoadsPower;
    fn into(raw: f64) -> Result<Self::Value, FoxError> {
        Ok(raw)
    }
}

/// Specification for the `SoC` variable.
pub struct SoC;
impl VariableSpec for SoC {
    type Value = u8;
    const VARIABLE: FoxVariables = FoxVariables::SoC;

    fn into(raw: f64) -> Result<Self::Value, FoxError> {
        try_f64_to_u8_percentage(raw).map_err(|e| FoxError::VariableConversionError {
            variable: Self::VARIABLE.as_str(),
            value: raw.to_string(),
            error: e.to_string(),
        })
    }
}

/// Specification for the `SoH` variable.
pub struct SoH;
impl VariableSpec for SoH {
    type Value = u8;
    const VARIABLE: FoxVariables = FoxVariables::SOH;

    fn into(raw: f64) -> Result<Self::Value, FoxError> {
        try_f64_to_u8_percentage(raw).map_err(|e| FoxError::VariableConversionError {
            variable: Self::VARIABLE.as_str(),
            value: raw.to_string(),
            error: e.to_string(),
        })
    }
}

/// Specification for the `BatTemperature` variable.
pub struct BatTemperature;
impl VariableSpec for BatTemperature {
    type Value = f64;
    const VARIABLE: FoxVariables = FoxVariables::BatTemperature;

    fn into(raw: f64) -> Result<Self::Value, FoxError> {
        Ok(raw)
    }
}

/// Attempts to convert a floating-point value to an 8-bit unsigned integer percentage.
///
/// The value is rounded to the nearest integer and checked to be in the range [0, 100].
///
/// # Arguments
/// * `x` - The raw floating-point value.
///
/// # Returns
/// * `Result<u8, &'static str>` - The converted percentage or an error if the value is invalid.
fn try_f64_to_u8_percentage(x: f64) -> Result<u8, &'static str> {
    if !x.is_finite() {
        return Err("value is NaN or infinite");
    }

    let y = x.round(); // ties-to-even (banker's rounding)

    if !(0.0..=100.0).contains(&y) {
        return Err("value out of range for u8 percentage after rounding");
    }

    Ok(y as u8)
}
use std::str::FromStr;
use crate::FoxError;

const PV_POWER: &str = "pvPower";
const LOADS_POWER: &str = "loadsPower";
const SOC: &str = "SoC";
const SOH: &str = "SOH";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoxVariables {
    PvPower,
    LoadsPower,
    SoC,
    SoH,
}

impl FoxVariables {
    pub const fn as_str(&self) -> &'static str {
        match self {
            FoxVariables::PvPower => PV_POWER,
            FoxVariables::LoadsPower => LOADS_POWER,
            FoxVariables::SoC => SOC,
            FoxVariables::SoH => SOH,
        }
    }
}
impl FromStr for FoxVariables {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PV_POWER => Ok(FoxVariables::PvPower),
            LOADS_POWER => Ok(FoxVariables::LoadsPower),
            SOC => Ok(FoxVariables::SoC),
            SOH => Ok(FoxVariables::SoH),
            _ => Err(()),
        }
    }
}

/// A typed "spec" for a variable: it fixes the setting key and the value type.
pub trait VariableSpec {
    type Value;
    const VARIABLE: FoxVariables;
    fn into(raw: f64) -> Result<Self::Value, FoxError>;
}

pub struct PvPower;
impl VariableSpec for PvPower {
    type Value = f64;
    const VARIABLE: FoxVariables = FoxVariables::PvPower;

    fn into(raw: f64) -> Result<Self::Value, FoxError> {
        Ok(raw)
    }
}

pub struct LoadsPower;
impl VariableSpec for LoadsPower {
    type Value = f64;
    const VARIABLE: FoxVariables = FoxVariables::LoadsPower;
    fn into(raw: f64) -> Result<Self::Value, FoxError> {
        Ok(raw)
    }
}

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

pub struct SoH;
impl VariableSpec for SoH {
    type Value = u8;
    const VARIABLE: FoxVariables = FoxVariables::SoH;

    fn into(raw: f64) -> Result<Self::Value, FoxError> {
        try_f64_to_u8_percentage(raw).map_err(|e| FoxError::VariableConversionError {
            variable: Self::VARIABLE.as_str(),
            value: raw.to_string(),
            error: e.to_string(),
        })
    }
}

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
//! This module hosts builtin marker structs to be used together with typed get functions.
//! These marker structs are used to implement the [SettingSpec] trait for the builtin settings.
//!

use std::str::FromStr;
use crate::fox_settings::{SettableSettingSpec, SettingSpec};
use crate::{FoxError, FoxSettings, FoxWorkModes};

/// Specification for the `ExportLimit` setting.
pub struct ExportLimit;
impl SettingSpec for ExportLimit {
    type Value = f64;
    const SETTING: FoxSettings = FoxSettings::ExportLimit;

    fn parse(raw: String) -> Result<Self::Value, FoxError> {
        raw.parse::<f64>().map_err(|e| FoxError::SettingParseError {
            setting: Self::SETTING.as_str(),
            value: raw,
            error: e.to_string(),
        })
    }
}

/// Specification for the `MinSocOnGrid` setting.
pub struct MinSocOnGrid;
impl SettingSpec for MinSocOnGrid {
    type Value = u8;
    const SETTING: FoxSettings = FoxSettings::MinSocOnGrid;

    fn parse(raw: String) -> Result<Self::Value, FoxError> {
        raw.parse::<u8>()
            .map(|v| v.clamp(0, 100))
            .map_err(|e| FoxError::SettingParseError {
                setting: Self::SETTING.as_str(),
                value: raw,
                error: e.to_string(),
            })
    }
}

impl SettableSettingSpec for MinSocOnGrid {
    fn format(value: &Self::Value) -> String {
        value.clamp(&10, &100).to_string()
    }
}

/// Specification for the `MaxSoc` setting.
pub struct MaxSoc;
impl SettingSpec for MaxSoc {
    type Value = u8;
    const SETTING: FoxSettings = FoxSettings::MaxSoc;

    fn parse(raw: String) -> Result<Self::Value, FoxError> {
        raw.parse::<u8>()
            .map(|v| v.clamp(0, 100))
            .map_err(|e| FoxError::SettingParseError {
                setting: Self::SETTING.as_str(),
                value: raw,
                error: e.to_string(),
            })
    }
}

impl SettableSettingSpec for MaxSoc {
    fn format(value: &Self::Value) -> String {
        value.clamp(&10, &100).to_string()
    }
}

/// Specification for the `WorkMode` setting.
pub struct WorkMode;
impl SettingSpec for WorkMode {
    type Value = FoxWorkModes;
    const SETTING: FoxSettings = FoxSettings::WorkMode;

    fn parse(raw: String) -> Result<Self::Value, FoxError> {
        FoxWorkModes::from_str(&raw).map_err(|_| FoxError::SettingParseError {
            setting: Self::SETTING.as_str(),
            value: raw,
            error: String::new(),
        })
    }
}

impl SettableSettingSpec for WorkMode {
    fn format(value: &Self::Value) -> String {
        value.as_str().to_string()
    }
}

/// Specification for the `MaxSetChargeCurrent` setting.
pub struct MaxSetChargeCurrent;
impl SettingSpec for MaxSetChargeCurrent {
    type Value = f64;
    const SETTING: FoxSettings = FoxSettings::MaxSetChargeCurrent;

    fn parse(raw: String) -> Result<Self::Value, FoxError> {
        raw.parse::<f64>().map_err(|e| FoxError::SettingParseError {
            setting: Self::SETTING.as_str(),
            value: raw,
            error: e.to_string(),
        })
    }
}
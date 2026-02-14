use crate::fox_settings::{SettableSettingSpec, SettingSpec};
use crate::{FoxError, FoxSettings};

/// Specification for the `ExportLimit` setting.
pub struct ExportLimit;
impl SettingSpec for crate::fox_settings::ExportLimit {
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
impl SettingSpec for crate::fox_settings::MinSocOnGrid {
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

impl SettableSettingSpec for crate::fox_settings::MinSocOnGrid {
    fn format(value: &Self::Value) -> String {
        value.clamp(&10, &100).to_string()
    }
}

/// Specification for the `MaxSoc` setting.
pub struct MaxSoc;
impl SettingSpec for crate::fox_settings::MaxSoc {
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

impl SettableSettingSpec for crate::fox_settings::MaxSoc {
    fn format(value: &Self::Value) -> String {
        value.clamp(&10, &100).to_string()
    }
}

/// Specification for the `WorkMode` setting.
pub struct WorkMode;
impl SettingSpec for crate::fox_settings::WorkMode {
    type Value = String;
    const SETTING: FoxSettings = FoxSettings::WorkMode;

    fn parse(raw: String) -> Result<Self::Value, FoxError> {
        Ok(raw)
    }
}

/// Specification for the `MaxSetChargeCurrent` setting.
pub struct MaxSetChargeCurrent;
impl SettingSpec for crate::fox_settings::MaxSetChargeCurrent {
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
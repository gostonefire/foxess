use std::str::FromStr;
use crate::FoxError;
// Available settings from FoxESS cloud:
// * ExportLimit
// * MinSoc
// * MinSocOnGrid
// * MaxSoc
// * GridCode
// * WorkMode
// * ActivePowerLimit
// * ExportLimitPower
// * EpsOutPut
// * MaxSetChargeCurrent
// * MaxSetDischargeCurrent
// * ECOMode
// * Meter1Enable
// * Meter2Enable
// * SysSwitch
// * GroundProtection

const EXPORT_LIMIT: &str = "ExportLimit";
const MIN_SOC_ON_GRID: &str = "MinSocOnGrid";
const MAX_SOC: &str = "MaxSoc";
const WORK_MODE: &str = "WorkMode";
const MAX_SET_CHARGE_CURRENT: &str = "MaxSetChargeCurrent";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoxSettings {
    ExportLimit,
    MinSocOnGrid,
    MaxSoc,
    WorkMode,
    MaxSetChargeCurrent,
}

impl FoxSettings {
    /// Returns the string representation of the FoxSettings enum variant
    /// 
    pub const fn as_str(&self) -> &'static str {
        match self {
            FoxSettings::ExportLimit => EXPORT_LIMIT,
            FoxSettings::MinSocOnGrid => MIN_SOC_ON_GRID,
            FoxSettings::MaxSoc => MAX_SOC,
            FoxSettings::WorkMode => WORK_MODE,
            FoxSettings::MaxSetChargeCurrent => MAX_SET_CHARGE_CURRENT,
        }
    }
    
    /// Returns true if the setting is allowed to be set
    /// 
    pub fn set_allowed(&self) -> bool {
        matches!(self, FoxSettings::MinSocOnGrid | FoxSettings::MaxSoc)
    }
}

impl FromStr for FoxSettings {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            EXPORT_LIMIT => Ok(FoxSettings::ExportLimit),
            MIN_SOC_ON_GRID => Ok(FoxSettings::MinSocOnGrid),
            MAX_SOC => Ok(FoxSettings::MaxSoc),
            WORK_MODE => Ok(FoxSettings::WorkMode),
            MAX_SET_CHARGE_CURRENT => Ok(FoxSettings::MaxSetChargeCurrent),
            _ => Err(()),
        }
    }
}

/// A typed "spec" for a setting: it fixes the setting key and the value type.
pub trait SettingSpec {
    type Value;
    const SETTING: FoxSettings;
    fn parse(raw: String) -> Result<Self::Value, FoxError>;
}

/// A typed "spec" for settings that are allowed to be set.
/// Implement this trait only for settable settings.
pub trait SettableSettingSpec: SettingSpec {
    fn format(value: &Self::Value) -> String;
}

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

pub struct WorkMode;
impl SettingSpec for WorkMode {
    type Value = String;
    const SETTING: FoxSettings = FoxSettings::WorkMode;

    fn parse(raw: String) -> Result<Self::Value, FoxError> {
        Ok(raw)
    }
}

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
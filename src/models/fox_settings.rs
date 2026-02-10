//! This module hosts setting enumerations, specifications, and associated traits.
//! The [FoxSettings] enumeration is mainly used when requesting several settings from FoxESS cloud at once.
//! 
//! The [SettingSpec] is implemented for marker structs to be used together with typed get functions.
//! 
//! The [SettableSettingSpec] is implemented for marker structs to be used together with typed set functions.
//! It can be dangerous to alter some settings in the inverter without first consulting with the installer first,
//! hence the trait is currently only implemented for a few settings.
//! 
//! FoxESS Cloud currently supports the following settings, end the foxess crate implements only a subset from them:
//! * ExportLimit
//! * MinSoc
//! * MinSocOnGrid
//! * MaxSoc
//! * GridCode
//! * WorkMode
//! * ActivePowerLimit
//! * ExportLimitPower
//! * EpsOutPut
//! * MaxSetChargeCurrent
//! * MaxSetDischargeCurrent
//! * ECOMode
//! * Meter1Enable
//! * Meter2Enable
//! * SysSwitch
//! * GroundProtection

use std::str::FromStr;
use crate::FoxError;

const EXPORT_LIMIT: &str = "ExportLimit";
const MIN_SOC_ON_GRID: &str = "MinSocOnGrid";
const MAX_SOC: &str = "MaxSoc";
const WORK_MODE: &str = "WorkMode";
const MAX_SET_CHARGE_CURRENT: &str = "MaxSetChargeCurrent";

/// An enumeration representing implemented settings from FoxESS cloud, i.e., a subset from available settings.
///
/// These settings can be retrieved or set on FoxESS devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoxSettings {
    /// The maximum power that can be exported to the grid.
    ExportLimit,
    /// The minimum State of Charge (SoC) while the device is on-grid.
    MinSocOnGrid,
    /// The maximum State of Charge (SoC) allowed.
    MaxSoc,
    /// The operational mode of the device.
    WorkMode,
    /// The maximum allowed charge current.
    MaxSetChargeCurrent,
}

impl FoxSettings {
    /// Returns the string representation of the `FoxSettings` enum variant.
    /// 
    /// This string matches the key used by the FoxESS cloud API.
    ///
    /// # Returns
    /// * `&'static str` - The string representation of the setting.
    pub const fn as_str(&self) -> &'static str {
        match self {
            FoxSettings::ExportLimit => EXPORT_LIMIT,
            FoxSettings::MinSocOnGrid => MIN_SOC_ON_GRID,
            FoxSettings::MaxSoc => MAX_SOC,
            FoxSettings::WorkMode => WORK_MODE,
            FoxSettings::MaxSetChargeCurrent => MAX_SET_CHARGE_CURRENT,
        }
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

/// A trait for settings that can be updated on the FoxESS cloud.
///
/// Types implementing this trait provide a way to format a value back into a string
/// for API requests.
pub trait SettableSettingSpec: SettingSpec {
    /// Formats the value into a string suitable for an API request.
    ///
    /// # Arguments
    /// * `value` - A reference to the value to be formatted.
    ///
    /// # Returns
    /// * `String` - The formatted string.
    fn format(value: &Self::Value) -> String;
}

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
    type Value = String;
    const SETTING: FoxSettings = FoxSettings::WorkMode;

    fn parse(raw: String) -> Result<Self::Value, FoxError> {
        Ok(raw)
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
//! Request DTOs for FoxESS API operations.
//!

use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use serde_json::Value;
use std::collections::HashMap;


/// Request for device history data.
#[derive(Serialize)]
pub struct RequestDeviceHistoryData<'a> {
    /// Device serial number.
    pub sn: &'a str,
    /// List of variable names to retrieve.
    pub variables: Vec<&'a str>,
    /// Start timestamp (Unix milliseconds).
    pub begin: i64,
    /// End timestamp (Unix milliseconds).
    pub end: i64,
}

/// Request for device real-time data.
#[derive(Serialize)]
pub struct RequestDeviceRealTimeData<'a> {
    /// List of variable names to retrieve.
    pub variables: Vec<&'a str>,
    /// List of device serial numbers.
    pub sns: Vec<&'a str>,
}

/// Request for device settings data.
#[derive(Serialize)]
pub struct RequestSettingsData<'a> {
    /// Device serial number.
    pub sn: &'a str,
    /// Setting key.
    pub key: &'a str,
}

/// Request to update a device setting.
#[derive(Serialize)]
pub struct SetSetting<'a> { 
    /// Device serial number.
    pub sn: &'a str, 
    /// Setting key.
    pub key: &'a str, 
    /// New value for the setting.
    pub value: &'a str, 
}

/// Charging time schedule configuration.
#[derive(Serialize)]
pub struct ChargingTimeSchedule {
    /// Device serial number.
    #[serde(skip_deserializing)]
    pub sn: String,
    /// Whether the first time period is enabled.
    #[serde(rename = "enable1")]
    pub enable_1: bool,
    /// Start time for the first period.
    #[serde(rename = "startTime1")]
    pub start_time_1: ChargingTime,
    /// End time for the first period.
    #[serde(rename = "endTime1")]
    pub end_time_1: ChargingTime,
    /// Whether the second time period is enabled.
    #[serde(rename = "enable2")]
    pub enable_2: bool,
    /// Start time for the second period.
    #[serde(rename = "startTime2")]
    pub start_time_2: ChargingTime,
    /// End time for the second period.
    #[serde(rename = "endTime2")]
    pub end_time_2: ChargingTime,
}

/// Representation of a specific time (hour and minute).
#[derive(Serialize)]
pub struct ChargingTime {
    /// Hour (0-23).
    pub hour: u8,
    /// Minute (0-59).
    pub minute: u8,
}

/// A single data point containing a timestamp and a value.
#[derive(Serialize, Deserialize)]
pub struct Data {
    /// Timestamp or time string.
    pub time: String,
    /// The recorded value, potentially in scientific notation.
    #[serde(deserialize_with = "deserialize_scientific_notation")]
    pub value: f64,
}

/// A set of data points for a specific variable.
#[derive(Deserialize)]
pub struct DataSet {
    /// List of data points.
    pub data: Vec<Data>,
    /// The variable name.
    pub variable: String,
}

/// Historical data for a device.
#[derive(Deserialize)]
pub struct DeviceHistoryData {
    /// List of data sets.
    #[serde(rename = "datas")]
    pub data_set: Vec<DataSet>,
}

/// Result of a device history query.
#[derive(Deserialize)]
pub struct DeviceHistoryResult {
    /// List of history data.
    pub result: Vec<DeviceHistoryData>,
}

/// Real-time data for a single variable.
#[derive(Deserialize)]
pub struct RealTimeData {
    /// The variable name.
    pub variable: String,
    /// The current value, potentially in scientific notation.
    #[serde(deserialize_with = "deserialize_scientific_notation")]
    pub value: f64,
}

/// Wrapper for real-time variables data.
#[derive(Deserialize)]
pub struct RealTimeVariables {
    /// List of real-time data points.
    pub datas: Vec<RealTimeData>,
}

/// Result of a real-time data query.
#[derive(Deserialize)]
pub struct DeviceRealTimeResult {
    /// List of real-time variables.
    pub result: Vec<RealTimeVariables>,
}

/// Settings data containing a single value.
#[derive(Deserialize)]
pub struct SettingsData {
    /// The setting value as a string.
    pub value: String,
}

/// Result of a device settings query.
#[derive(Deserialize)]
pub struct DeviceSettingsResult {
    /// The settings data.
    pub result: SettingsData,
}

/// Variable information containing unit and localized names.
#[derive(Deserialize)]
pub struct DeviceVariableInfo {
    /// The unit of the variable (e.g., "kW", "V").
    pub unit: Option<String>,
    /// Localized names for the variable.
    pub name: HashMap<String, String>,
    /// Enumeration values for the variable, if any.
    #[serde(rename = "enum")]
    pub enumeration: Option<HashMap<String, String>>,
}

/// Result of a device variables query.
#[derive(Deserialize)]
pub struct DeviceVariablesResult {
    /// List of variables, where each entry is a map from variable name to its info.
    pub result: Vec<HashMap<String, DeviceVariableInfo>>,
}


/// Deserializes an `f64` value that may be represented as a number or a string in scientific notation.
fn deserialize_scientific_notation<'de, D>(deserializer: D) -> Result<f64, D::Error>
where D: Deserializer<'de> {

    let v = Value::deserialize(deserializer)?;
    let x = v.as_f64()
        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        .ok_or_else(|| Error::custom("non-f64"))?
        .try_into()
        .map_err(|_| Error::custom("overflow"))?;

    Ok(x)
}



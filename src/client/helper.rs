//! Internal helper utilities for the FoxESS API client.
//!
//! This module provides the [`FoxHelper`] struct and associated functions that handle the
//! heavy lifting for the [`Fox`](crate::client::Fox) struct. It encapsulates the logic for:
//!
//! - Preparing request payloads and paths for the FoxESS Open API.
//! - Parsing and transforming raw API responses into domain models.
//! - Managing authentication headers and request signing.
//! - Coordinating time conversions and data formatting.
//!
//! The relationship between `Fox` and `FoxHelper` follows a separation of concerns where
//! `Fox` manages the high-level API (async/blocking clients, network transport) while
//! `FoxHelper` contains the pure, testable logic for interacting with the FoxESS API protocol.

use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;
use chrono::{DateTime, Local, NaiveTime, TimeDelta, Timelike, Utc};
use md5::{Digest, Md5};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use crate::{FoxError, FoxSettings, FoxVariables};
use crate::fox_settings::SettableSettingSpec;
use crate::models::{VariablesDataHistory, VariablesData, VariableDataSet, VariableDataPoint, VariableInfo, AvailableVariables};
use crate::models::dto::{ChargingTime, ChargingTimeSchedule, DeviceHistoryData, DeviceHistoryResult, DeviceRealTimeResult, DeviceSettingsResult, DeviceVariablesResult, RequestDeviceHistoryData, RequestDeviceRealTimeData, RequestSettingsData, SetSetting};


pub(crate) struct FoxHelper {
    api_key: String,
    sn: String,
    base_url: String,
    now_millis: fn() -> i64,
}

impl FoxHelper {
    /// Returns a new instance of the [`FoxHelper`] struct.
    ///
    /// # Arguments
    /// * `api_key` - The FoxESS API Key.
    /// * `sn` - The FoxESS inverter serial number.
    /// * `base_url` - The base URL for the FoxESS API.
    /// * `now_millis` - A function that returns the current time in milliseconds.
    pub(crate) fn new(api_key: &str, sn: &str, base_url: &str, now_millis: fn() -> i64) -> Self {
        Self {
            api_key: api_key.to_string(),
            sn: sn.to_string(),
            base_url: base_url.to_string(),
            now_millis,
        }
    }
    
    /// Pre-network request: Prepare the request for historical data.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20history20data0a3ca20id3dget20device20history20data4303e203ca3e).
    ///
    /// # Arguments
    /// * `start` - The start time for the data range.
    /// * `end` - The end time for the data range.
    /// * `parameters` - A list of variables to retrieve.
    ///
    /// # Returns
    /// * `Result<(String, &'static str), FoxError>` - A tuple containing the JSON request body and the API path.
    pub(crate) fn pre_get_variables_history(&self, start: DateTime<Utc>, end: DateTime<Utc>, parameters: Vec<FoxVariables>) -> Result<(String, &'static str), FoxError> {
        let path = "/op/v0/device/history/query";

        let req = RequestDeviceHistoryData {
            sn: &self.sn,
            variables: parameters.iter().map(|p| p.as_str()).collect(),
            begin: start.timestamp_millis(),
            end: end.timestamp_millis(),
        };

        Ok((serde_json::to_string(&req)?, path))
    }

    /// Post-network request: Process the response from the historical data request.
    ///
    /// # Arguments
    /// * `json` - The JSON response string from the API.
    ///
    /// # Returns
    /// * `Result<VariablesDataHistory, FoxError>` - A structure containing the historical data points.
    pub(crate) fn post_get_variables_history(&self, json: &str) -> Result<VariablesDataHistory, FoxError> {
        let fox_data: DeviceHistoryResult = serde_json::from_str(json)?;
        let device_history = transform_history_data(fox_data.result)?;

        Ok(device_history)
    }

    /// Pre-network request: Prepare the request for real-time data.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20real-time20data0a3ca20id3dget20device20real-time20data5603e203ca3e).
    ///
    /// # Arguments
    /// * `variables` - A list of variables to retrieve.
    ///
    /// # Returns
    /// * `Result<(String, &'static str), FoxError>` - A tuple containing the JSON request body and the API path.
    pub(crate) fn pre_get_variables(&self, variables: Vec<FoxVariables>) -> Result<(String, &'static str), FoxError> {
        let path = "/op/v1/device/real/query";

        let req = RequestDeviceRealTimeData {
            variables: variables.iter().map(|p| p.as_str()).collect(),
            sns: vec![&self.sn],
        };

        Ok((serde_json::to_string(&req)?, path))
    }

    /// Post-network request: Process the response from the real-time data request.
    ///
    /// # Arguments
    /// * `json` - The JSON response string from the API.
    ///
    /// # Returns
    /// * `Result<VariablesData, FoxError>` - A structure containing the latest data points.
    pub(crate) fn post_get_variables(&self, json: &str) -> Result<VariablesData, FoxError> {
        let fox_data: DeviceRealTimeResult = serde_json::from_str(json)?;

        let mut data_points: HashMap<FoxVariables, VariableDataPoint> = HashMap::new();

        // Be defensive: Fox API returns a Vec; can't assume [0] exists.
        let Some(first) = fox_data.result.first() else {
            return Ok(VariablesData {
                data_points,
            });
        };

        for data in first.datas.iter() {
            // Only accept variables that are part of FoxParameter.
            let Ok(p) = FoxVariables::from_str(data.variable.as_str()) else {
                continue;
            };

            let value = data.value;
            data_points.insert(p, VariableDataPoint(value));
        }

        Ok(VariablesData { data_points })
    }

    /// Pre-network request: Prepare the request for a single setting.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20device20settings20item0a3ca20id3dget20the20device20settings20item4303e203ca3e).
    ///
    /// # Arguments
    /// * `setting` - The setting to retrieve.
    ///
    /// # Returns
    /// * `Result<(String, &'static str), FoxError>` - A tuple containing the JSON request body and the API path.
    pub(crate) fn pre_get_setting(&self, setting: FoxSettings) -> Result<(String, &'static str), FoxError> {
        let path = "/op/v0/device/setting/get";

        let req = RequestSettingsData { sn: &self.sn, key: setting.as_str() };

        Ok((serde_json::to_string(&req)?, path))
    }

    /// Post-network request: Process the response from the get setting request.
    ///
    /// # Arguments
    /// * `json` - The JSON response string from the API.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The raw string value of the setting.
    pub(crate) fn post_get_setting(&self, json: &str) -> Result<String, FoxError> {
        let fox_data: DeviceSettingsResult = serde_json::from_str(json)?;

        Ok(fox_data.result.value)
    }
    
    /// Pre-network request: Prepare the request to set a single inverter setting.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20device20settings20item0a3ca20id3dset20the20device20settings20item4303e203ca3e).
    ///
    /// # Type Parameters
    /// * `S` - A [`SettableSettingSpec`] describing which setting can be set and how to format it.
    ///
    /// # Arguments
    /// * `value` - The new value for `S::SETTING`.
    ///
    /// # Returns
    /// * `Result<(String, &'static str), FoxError>` - A tuple containing the JSON request body and the API path.
    pub(crate) fn pre_set_setting_typed<S: SettableSettingSpec>(&self, value: S::Value) -> Result<(String, &'static str), FoxError> {
        let path = "/op/v0/device/setting/set";
        let data = S::format(&value);

        let req = SetSetting { sn: &self.sn, key: S::SETTING.as_str(), value: &data };

        Ok((serde_json::to_string(&req)?, path))
    }

    /// Pre-network request: Prepare the request to set the battery charging time schedule.
    ///
    /// This is the standard charging scheduler setting. No time overlaps are permitted between the two schedules.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20battery20charging20time0a3ca20id3dset20the20battery20charging20time4303e203ca3e).
    ///
    /// # Arguments
    /// * `enable` - Whether schedule 1 should be enabled.
    /// * `start` - The start time of schedule 1 as a [`DateTime<Utc>`].
    /// * `end` - The end time of schedule 1 as a [`DateTime<Utc>`] (non-inclusive).
    ///
    /// # Returns
    /// * `Result<(String, &'static str), FoxError>` - A tuple containing the JSON request body and the API path.
    pub (crate) fn pre_set_battery_charging_time_schedule(&self, enable: bool, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<(String,&'static str), FoxError> {
        let path = "/op/v0/device/battery/forceChargeTime/set";

        let mut start_hour: u8 = 0;
        let mut start_minute: u8 = 0;
        let mut end_hour: u8 = 0;
        let mut end_minute: u8 = 0;

        if enable {
            let start_local = start.with_timezone(&Local);
            let end_local = end.with_timezone(&Local).add(TimeDelta::minutes(-1));

            start_hour = start_local.hour() as u8;
            start_minute = start_local.minute() as u8;
            end_hour = end_local.hour() as u8;
            end_minute = end_local.minute() as u8;
        }

        let schedule = self.build_charge_time_schedule(
            enable, start_hour, start_minute, end_hour, end_minute,
            false, 0, 0, 0, 0,
        )?;
        let req_json = serde_json::to_string(&schedule)?;

        Ok((req_json, path))
    }

    /// Pre-network request: Prepare the request for available variables.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20available20variables0a3ca20id3dget20available20variables4303e203ca3e).
    ///
    /// # Returns
    /// * `Result<&'static str, FoxError>` - A string containing the API path.
    pub(crate) fn pre_get_available_variables(&self) -> Result<&'static str, FoxError> {
        let path = "/op/v0/device/variable/get";

        Ok(path)
    }

    /// Post-network request: Process the response from the get available variables request.
    ///
    /// # Arguments
    /// * `json` - The JSON response string from the API.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - A vector with available variables.
    pub(crate) fn post_get_available_variables(&self, json: &str) -> Result<AvailableVariables, FoxError> {
        let fox_data: DeviceVariablesResult = serde_json::from_str(json)?;

        let variables: Vec<VariableInfo> = fox_data
            .result
            .into_iter()
            .filter_map(|mut v| {
                // Each entry is expected to be a one-key map; skip unexpected shapes.
                let (variable, info) = v.drain().next()?;

                let name = info
                    .name
                    .get("en")
                    .cloned()
                    .unwrap_or_else(|| variable.clone());

                Some(VariableInfo {
                    variable,
                    name,
                    unit: info.unit,
                    enumeration: info.enumeration,
                })
            })
            .collect();

        Ok(AvailableVariables { variables })
    }

    /// Pre-network request: Prepare a POST request.
    ///
    /// # Arguments
    /// * `path` - The API path, excluding the domain.
    ///
    /// # Returns
    /// * `(String, HeaderMap)` - A tuple containing the full URL and the required headers.
    pub(crate) fn pre_network_post_request(&self, path: &str) -> (String,HeaderMap) {
        (
            format!("{}{}", self.base_url, path), // Full URL
            generate_headers(&self.api_key, path, (self.now_millis)(), Some(vec![("Content-Type", "application/json")])), // Headers
        )
    }

    /// Post-network request: Validate the response from a POST request.
    ///
    /// # Arguments
    /// * `json` - The JSON response string to validate.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The original JSON string if the response is successful.
    pub(crate) fn post_network_post_request(&self, json: String) -> Result<String, FoxError> {
        let fox_res: FoxResponse = serde_json::from_str(&json)?;
        if fox_res.errno != 0 {
            return Err(FoxError::FoxCloud(format!("errno: {}, msg: {}", fox_res.errno, fox_res.msg)));
        }

        Ok(json)
    }

    /// Pre-network request: Prepare a GET request.
    ///
    /// # Arguments
    /// * `path` - The API path, excluding the domain.
    ///
    /// # Returns
    /// * `(String, HeaderMap)` - A tuple containing the full URL and the required headers.
    pub(crate) fn pre_network_get_request(&self, path: &str) -> (String,HeaderMap) {
        (
            format!("{}{}", self.base_url, path), // Full URL
            generate_headers(&self.api_key, path, (self.now_millis)(), Some(vec![("Content-Type", "application/json")])), // Headers
        )
    }

    /// Post-network request: Validate the response from a POST request.
    ///
    /// # Arguments
    /// * `json` - The JSON response string to validate.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The original JSON string if the response is successful.
    pub(crate) fn post_network_get_request(&self, json: String) -> Result<String, FoxError> {
        let fox_res: FoxResponse = serde_json::from_str(&json)?;
        if fox_res.errno != 0 {
            return Err(FoxError::FoxCloud(format!("errno: {}, msg: {}", fox_res.errno, fox_res.msg)));
        }

        Ok(json)
    }

    /// Builds a charge time schedule after first checking for inconsistencies.
    ///
    /// Inconsistencies include:
    /// * Invalid time (e.g., hour outside 0-23 or minute outside 0-59).
    /// * Start time after end time.
    /// * Overlap between schedule 1 and 2 (times are inclusive on both ends).
    ///
    /// Minor errors are corrected automatically:
    /// * A schedule that is not enabled is set to zero start and end time.
    /// * A schedule that is enabled but has the same start and end time is disabled and zeroed.
    ///
    /// # Arguments
    /// * `enable_1` - Whether schedule 1 should be enabled.
    /// * `start_hour_1` - Start hour of schedule 1.
    /// * `start_minute_1` - Start minute of schedule 1.
    /// * `end_hour_1` - End hour of schedule 1.
    /// * `end_minute_1` - End minute of schedule 1.
    /// * `enable_2` - Whether schedule 2 should be enabled.
    /// * `start_hour_2` - Start hour of schedule 2.
    /// * `start_minute_2` - Start minute of schedule 2.
    /// * `end_hour_2` - End hour of schedule 2.
    /// * `end_minute_2` - End minute of schedule 2.
    ///
    /// # Returns
    /// * `Result<ChargingTimeSchedule, FoxError>` - The validated [`ChargingTimeSchedule`].
    fn build_charge_time_schedule(
        &self,
        mut enable_1: bool, mut start_hour_1: u8, mut start_minute_1: u8, mut end_hour_1: u8, mut end_minute_1: u8,
        mut enable_2: bool, mut start_hour_2: u8, mut start_minute_2: u8, mut end_hour_2: u8, mut end_minute_2: u8,
    ) -> Result<ChargingTimeSchedule, FoxError> {

        // Check schedule 1 for inconsistencies
        let start_1 = NaiveTime::from_hms_opt(start_hour_1 as u32, start_minute_1 as u32, 0)
            .ok_or(FoxError::ScheduleBuildError("charge schedule 1 start time error".to_string()))?;
        let end_1 = NaiveTime::from_hms_opt(end_hour_1 as u32, end_minute_1 as u32, 0)
            .ok_or(FoxError::ScheduleBuildError("charge schedule 1 end time error".to_string()))?;
        let dur_1 = end_1 - start_1;

        if dur_1 < TimeDelta::new(0, 0).unwrap() {
            return Err(FoxError::ScheduleBuildError("charge schedule 1 start time is after end time".to_string()));
        }

        if !enable_1 || dur_1 == TimeDelta::new(0, 0).unwrap() {
            enable_1 = false;
            start_hour_1 = 0;
            start_minute_1 = 0;
            end_hour_1 = 0;
            end_minute_1 = 0;
        }

        // Check schedule 2 for inconsistencies
        let start_2 = NaiveTime::from_hms_opt(start_hour_2 as u32, start_minute_2 as u32, 0)
            .ok_or(FoxError::ScheduleBuildError("charge schedule 2 start time error".to_string()))?;
        let end_2 = NaiveTime::from_hms_opt(end_hour_2 as u32, end_minute_2 as u32, 0)
            .ok_or(FoxError::ScheduleBuildError("charge schedule 2 end time error".to_string()))?;
        let dur_2 = end_2 - start_2;

        if dur_2 < TimeDelta::new(0, 0).unwrap() {
            return Err(FoxError::ScheduleBuildError("charge schedule 2 start time is after end time".to_string()));
        }

        if !enable_2 || dur_2 <= TimeDelta::new(0, 0).unwrap() {
            enable_2 = false;
            start_hour_2 = 0;
            start_minute_2 = 0;
            end_hour_2 = 0;
            end_minute_2 = 0;
        }


        // Check if schedules are overlapping
        if enable_1 && enable_2 {
            if start_2 >= start_1 && start_2 <= start_1 + dur_1 {
                return Err(FoxError::ScheduleBuildError("overlapping charge schedules".to_string()));
            }
            if end_2 >= start_1 && end_2 <= start_1 + dur_1 {
                return Err(FoxError::ScheduleBuildError("overlapping charge schedules".to_string()));
            }
        }

        // All checks seem fine, return schedule struct
        Ok(ChargingTimeSchedule {
            sn: self.sn.clone(),
            enable_1,
            start_time_1: ChargingTime { hour: start_hour_1, minute: start_minute_1 },
            end_time_1: ChargingTime { hour: end_hour_1, minute: end_minute_1 },
            enable_2,
            start_time_2: ChargingTime { hour: start_hour_2, minute: start_minute_2 },
            end_time_2: ChargingTime { hour: end_hour_2, minute: end_minute_2 },
        })
    }
}

/// Generates HTTP headers required by the Fox Open API.
///
/// This includes building an MD5 hashed signature.
///
/// # Arguments
/// * `api_key` - The FoxESS API Key.
/// * `path` - The API path, excluding the domain.
/// * `timestamp_millis` - The current timestamp in milliseconds.
/// * `extra` - Any extra headers to add.
///
/// # Returns
/// * `HeaderMap` - The generated headers.
fn generate_headers(api_key: &str, path: &str, timestamp_millis: i64, extra: Option<Vec<(&str, &str)>>) -> HeaderMap {
    let mut headers = HeaderMap::new();

    let signature = format!("{}\\r\\n{}\\r\\n{}", path, api_key, timestamp_millis);

    let mut hasher = Md5::new();
    hasher.update(signature.as_bytes());
    let signature_md5 = hasher.finalize().iter().map(|x| format!("{:02x}", x)).collect::<String>();

    headers.insert("token", HeaderValue::from_str(api_key).unwrap());
    headers.insert("timestamp", HeaderValue::from_str(&timestamp_millis.to_string()).unwrap());
    headers.insert("signature", HeaderValue::from_str(&signature_md5).unwrap());
    headers.insert("lang", HeaderValue::from_str("en").unwrap());

    if let Some(h) = extra {
        h.iter().for_each(|&(k, v)| {
            headers.insert(HeaderName::from_str(k).unwrap(), HeaderValue::from_str(v).unwrap());
        });
    }

    headers
}

/// Transforms device history data to a format easier to save.
///
/// # Arguments
/// * `input` - The raw data to transform.
///
/// # Returns
/// * `Result<VariablesDataHistory, FoxError>` - The transformed historical data.
fn transform_history_data(input: Vec<DeviceHistoryData>) -> Result<VariablesDataHistory, FoxError> {
    let mut series: HashMap<FoxVariables, Vec<VariableDataSet<f64>>> = HashMap::new();

    // Be defensive: Fox API returns a Vec; can't assume [0] exists.
    let Some(first) = input.first() else {
        return Ok(VariablesDataHistory {
            series,
        });
    };

    for set in &first.data_set {
        // Only accept variables that are part of FoxParameter.
        let Ok(p) = FoxVariables::from_str(set.variable.as_str()) else {
            continue;
        };

        // History payload uses f64 values; store as-is.
        for d in set.data.iter() {
            let utc = cet_to_utc(&d.time)?;
            series
                .entry(p)
                .or_insert_with(Vec::new)
                .push(VariableDataSet {
                    date_time: utc,
                    data: d.value,
                });
        }
    }

    Ok(VariablesDataHistory {
        series,
    })
}

/// Converts a date time string in the Fox API format to UTC.
///
/// # Arguments
/// * `time` - A date time string (e.g., "2025-12-03 00:08:51 CET+0100").
///
/// # Returns
/// * `Result<DateTime<Utc>, FoxError>` - The parsed time in UTC.
fn cet_to_utc(time: &str) -> Result<DateTime<Utc>, FoxError> {
    let dt = DateTime::parse_from_str(&time.replace("+", " +"), "%Y-%m-%d %H:%M:%S %Z %z")?;
    Ok(dt.with_timezone(&Utc))
}

#[derive(Serialize, Deserialize)]
struct FoxResponse {
    errno: u32,
    msg: String,
}

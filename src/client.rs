use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use chrono::{DateTime, Utc};
use md5::{Digest, Md5};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use crate::error::FoxError;
use crate::models::{VariableDataPoint, HistoryDataSet, DeviceHistory, DeviceRealTime, FoxVariables};
use crate::models::dto::{DeviceHistoryData, DeviceHistoryResult, DeviceRealTimeResult, DeviceSettingsResult, RequestDeviceHistoryData, RequestDeviceRealTimeData, RequestSettingsData, SetSetting};
use crate::models::fox_settings::FoxSettings;
use crate::models::settings::{DeviceSettings, SettingsDataPoint};

const DEFAULT_REQUEST_DOMAIN: &str = "https://www.foxesscloud.com";

fn default_now_millis() -> i64 {
    Utc::now().timestamp() * 1000
}

#[cfg(feature = "async")]
pub struct Fox {
    api_key: String,
    sn: String,
    base_url: String,
    now_millis: fn() -> i64,
    client: reqwest::Client,
}

#[cfg(feature = "blocking")]
pub struct Fox {
    api_key: String,
    sn: String,
    base_url: String,
    now_millis: fn() -> i64,
    client: reqwest::blocking::Client,
}

#[cfg(feature = "async")]
impl Fox {
    /// Returns a new instance of the Fox struct
    ///
    /// # Arguments
    ///
    /// * 'api_key' - FoxESS API Key
    /// * 'sn' - FoxESS inverter serial number
    /// * 'request_timeout' - Request timeout in seconds
    pub fn new(api_key: &str, sn: &str, request_timeout: u64) -> Result<Self, FoxError> {
        Self::new_with_base_url(api_key, sn, request_timeout, DEFAULT_REQUEST_DOMAIN)
    }

    fn new_with_base_url(api_key: &str, sn: &str, request_timeout: u64, base_url: &str) -> Result<Self, FoxError> {
        Self::new_with_base_url_and_clock(api_key, sn, request_timeout, base_url, default_now_millis)
    }

    fn new_with_base_url_and_clock(
        api_key: &str,
        sn: &str,
        request_timeout: u64,
        base_url: &str,
        now_millis: fn() -> i64,
    ) -> Result<Self, FoxError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(request_timeout))
            .build()?;

        Ok(Self {
            api_key: api_key.to_string(),
            sn: sn.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
            now_millis,
            client,
        })
    }

    /// Collect history data from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20history20data0a3ca20id3dget20device20history20data4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'start' - the start time of the report
    /// * 'end' - the end time of the report
    /// * 'variables' - List of variables to retrieve from the inverter
    pub async fn get_device_history_data(&self, start: DateTime<Utc>, end: DateTime<Utc>, parameters: Vec<FoxVariables>) -> Result<DeviceHistory, FoxError> {
        let path = "/op/v0/device/history/query";

        let req = RequestDeviceHistoryData {
            sn: &self.sn,
            variables: parameters.iter().map(|p| p.as_str()).collect(),
            begin: start.timestamp_millis(),
            end: end.timestamp_millis(),
        };

        let req_json = serde_json::to_string(&req)?;

        let json = self.post_request(&path, req_json).await?;

        let fox_data: DeviceHistoryResult = serde_json::from_str(&json)?;
        let device_history = transform_history_data(fox_data.result)?;

        Ok(device_history)
    }

    /// Collect real-time data from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20real-time20data0a3ca20id3dget20device20real-time20data5603e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'variables' - List of variables to retrieve from the inverter
    pub async fn get_device_real_time_data(&self, variables: Vec<FoxVariables>) -> Result<DeviceRealTime, FoxError> {
        let path = "/op/v1/device/real/query";

        let req = RequestDeviceRealTimeData {
            variables: variables.iter().map(|p| p.as_str()).collect(),
            sns: vec![&self.sn],
        };

        let req_json = serde_json::to_string(&req)?;

        let json = self.post_request(&path, req_json).await?;

        let fox_data: DeviceRealTimeResult = serde_json::from_str(&json)?;

        let mut data_points: HashMap<FoxVariables, VariableDataPoint> = HashMap::new();

        // Be defensive: Fox API returns a Vec; can't assume [0] exists.
        let Some(first) = fox_data.result.first() else {
            return Ok(DeviceRealTime {
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

        Ok(DeviceRealTime { data_points })
    }

    /// Get settings from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20device20settings20item0a3ca20id3dget20the20device20settings20item4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'settings' - List of settings to retrieve from the inverter
    pub async fn get_settings(&self, settings: Vec<FoxSettings>) -> Result<DeviceSettings, FoxError> {
        let path = "/op/v0/device/setting/get";

        let mut data_points: HashMap<FoxSettings, SettingsDataPoint> = HashMap::new();

        for s in settings.iter() {
            let req = RequestSettingsData { sn: &self.sn, key: s.as_str() };

            let req_json = serde_json::to_string(&req)?;

            let json = self.post_request(&path, req_json).await?;

            let fox_data: DeviceSettingsResult = serde_json::from_str(&json)?;

            data_points.insert(*s, SettingsDataPoint(fox_data.result.value));
        }

        Ok(DeviceSettings { data_points })
    }

    /// Set setting in the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20device20settings20item0a3ca20id3dset20the20device20settings20item4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'setting' - Settings to set in the inverter
    pub async fn set_setting<T: ToString>(&self, setting: FoxSettings, value: T) -> Result<(), FoxError> {
        if !setting.set_allowed() { return Err(FoxError::UnallowedSetSetting); }
        
        let path = "/op/v0/device/setting/set";

        let data = value.to_string();

        let req = SetSetting { sn: &self.sn, key: setting.as_str(), value: &data };
        let req_json = serde_json::to_string(&req)?;

        let _ = self.post_request(&path, req_json).await?;
        
        Ok(())
    }

    /// Builds a request and sends it as a POST.
    /// The return is the json representation of the result as specified by
    /// respective FoxESS API
    ///
    /// # Arguments
    ///
    /// * path - the API path excluding the domain
    /// * body - a string containing the payload in json format
    async fn post_request(&self, path: &str, body: String) -> Result<String, FoxError> {
        let url = format!("{}{}", self.base_url, path);

        let timestamp = (self.now_millis)();
        let headers = generate_headers_at(&self.api_key, path, timestamp, Some(vec![("Content-Type", "application/json")]));

        let req = self.client.post(url)
            .headers(headers)
            .body(body)
            .send().await?;

        let status = req.status();
        if !status.is_success() {
            return Err(FoxError::FoxCloud(format!("{:?}", status)));
        }

        let json = req.text().await?;
        let fox_res: FoxResponse = serde_json::from_str(&json)?;
        if fox_res.errno != 0 {
            return Err(FoxError::FoxCloud(format!("errno: {}, msg: {}", fox_res.errno, fox_res.msg)));
        }

        Ok(json)
    }
}

#[cfg(feature = "blocking")]
impl Fox {
    /// Returns a new instance of the Fox struct
    ///
    /// # Arguments
    ///
    /// * 'api_key' - FoxESS API Key
    /// * 'sn' - FoxESS inverter serial number
    /// * 'request_timeout' - Request timeout in seconds
    pub fn new(api_key: &str, sn: &str, request_timeout: u64) -> Result<Self, FoxError> {
        Self::new_with_base_url(api_key, sn, request_timeout, DEFAULT_REQUEST_DOMAIN)
    }

    fn new_with_base_url(api_key: &str, sn: &str, request_timeout: u64, base_url: &str) -> Result<Self, FoxError> {
        Self::new_with_base_url_and_clock(api_key, sn, request_timeout, base_url, default_now_millis)
    }

    fn new_with_base_url_and_clock(
        api_key: &str,
        sn: &str,
        request_timeout: u64,
        base_url: &str,
        now_millis: fn() -> i64,
    ) -> Result<Self, FoxError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(request_timeout))
            .build()?;

        Ok(Self {
            api_key: api_key.to_string(),
            sn: sn.to_string(),
            base_url: base_url.trim_end_matches('/').to_string(),
            now_millis,
            client,
        })
    }

    /// Collect history data from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20history20data0a3ca20id3dget20device20history20data4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'start' - the start time of the report
    /// * 'end' - the end time of the report
    /// * 'variables' - List of variables to retrieve from the inverter
    pub fn get_device_history_data(&self, start: DateTime<Utc>, end: DateTime<Utc>, parameters: Vec<FoxVariables>) -> Result<DeviceHistory, FoxError> {
        let path = "/op/v0/device/history/query";

        let req = RequestDeviceHistoryData {
            sn: &self.sn,
            variables: parameters.iter().map(|p| p.as_str()).collect(),
            begin: start.timestamp_millis(),
            end: end.timestamp_millis(),
        };

        let req_json = serde_json::to_string(&req)?;

        let json = self.post_request(&path, req_json)?;

        let fox_data: DeviceHistoryResult = serde_json::from_str(&json)?;
        let device_history = transform_history_data(fox_data.result)?;

        Ok(device_history)
    }

    /// Collect real-time data from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20real-time20data0a3ca20id3dget20device20real-time20data5603e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'variables' - List of variables to retrieve from the inverter
    pub fn get_device_real_time_data(&self, variables: Vec<FoxVariables>) -> Result<DeviceRealTime, FoxError> {
        let path = "/op/v1/device/real/query";

        let req = RequestDeviceRealTimeData {
            variables: variables.iter().map(|p| p.as_str()).collect(),
            sns: vec![&self.sn],
        };

        let req_json = serde_json::to_string(&req)?;

        let json = self.post_request(&path, req_json)?;

        let fox_data: DeviceRealTimeResult = serde_json::from_str(&json)?;

        let mut data_points: HashMap<FoxVariables, VariableDataPoint> = HashMap::new();

        // Be defensive: Fox API returns a Vec; can't assume [0] exists.
        let Some(first) = fox_data.result.first() else {
            return Ok(DeviceRealTime {
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

        Ok(DeviceRealTime { data_points })
    }

    /// Get settings from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20device20settings20item0a3ca20id3dget20the20device20settings20item4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'settings' - List of settings to retrieve from the inverter
    pub fn get_settings(&self, settings: Vec<FoxSettings>) -> Result<DeviceSettings, FoxError> {
        let path = "/op/v0/device/setting/get";

        let mut data_points: HashMap<FoxSettings, SettingsDataPoint> = HashMap::new();

        for s in settings.iter() {
            let req = RequestSettingsData { sn: &self.sn, key: s.as_str() };

            let req_json = serde_json::to_string(&req)?;

            let json = self.post_request(&path, req_json)?;

            let fox_data: DeviceSettingsResult = serde_json::from_str(&json)?;

            data_points.insert(*s, SettingsDataPoint(fox_data.result.value));
        }

        Ok(DeviceSettings { data_points })
    }

    /// Set setting in the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20device20settings20item0a3ca20id3dset20the20device20settings20item4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'setting' - Settings to set in the inverter
    pub fn set_setting<T: ToString>(&self, setting: FoxSettings, value: T) -> Result<(), FoxError> {
        if !setting.set_allowed() { return Err(FoxError::UnallowedSetSetting); }

        let path = "/op/v0/device/setting/set";

        let data = value.to_string();

        let req = SetSetting { sn: &self.sn, key: setting.as_str(), value: &data };
        let req_json = serde_json::to_string(&req)?;

        let _ = self.post_request(&path, req_json)?;

        Ok(())
    }

    /// Builds a request and sends it as a POST.
    /// The return is the json representation of the result as specified by
    /// respective FoxESS API
    ///
    /// # Arguments
    ///
    /// * path - the API path excluding the domain
    /// * body - a string containing the payload in json format
    fn post_request(&self, path: &str, body: String) -> Result<String, FoxError> {
        let url = format!("{}{}", self.base_url, path);

        let timestamp = (self.now_millis)();
        let headers = generate_headers_at(&self.api_key, path, timestamp, Some(vec![("Content-Type", "application/json")]));

        let req = self.client.post(url)
            .headers(headers)
            .body(body)
            .send()?;

        let status = req.status();
        if !status.is_success() {
            return Err(FoxError::FoxCloud(format!("{:?}", status)));
        }

        let json = req.text()?;
        let fox_res: FoxResponse = serde_json::from_str(&json)?;
        if fox_res.errno != 0 {
            return Err(FoxError::FoxCloud(format!("errno: {}, msg: {}", fox_res.errno, fox_res.msg)));
        }

        Ok(json)
    }
}

/// Generates http headers required by Fox Open API, this includes also building a
/// md5 hashed signature.
///
/// # Arguments
///
/// * 'path' - the path, excluding the domain part, to the FoxESS specific API
/// * 'extra' - any extra headers to add besides FoxCloud standards
fn generate_headers_at(api_key: &str, path: &str, timestamp_millis: i64, extra: Option<Vec<(&str, &str)>>) -> HeaderMap {
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

/// Transforms device history data to a format easier to save as non-json file
///
/// # Arguments
///
/// * 'input' - the data to transform
fn transform_history_data(input: Vec<DeviceHistoryData>) -> Result<DeviceHistory, FoxError> {
    let mut series: HashMap<FoxVariables, Vec<HistoryDataSet<f64>>> = HashMap::new();

    // Be defensive: Fox API returns a Vec; can't assume [0] exists.
    let Some(first) = input.first() else {
        return Ok(DeviceHistory {
            series,
        });
    };

    for set in &first.data_set {
        let utc = cet_to_utc(&set.data[0].time)?;

        // Only accept variables that are part of FoxParameter.
        let Ok(p) = FoxVariables::from_str(set.variable.as_str()) else {
            continue;
        };

        // History payload uses f64 values; store as-is.
        set.data.iter().for_each(
            |d| series
                .entry(p)
                .or_insert_with(Vec::new)
                .push(HistoryDataSet { date_time: utc, data: d.value })
        );
    }

    Ok(DeviceHistory {
        series,
    })
}

/// Converts a date time string in special Fox format to UTC
///
/// # Arguments
///
/// * 'time' - date time string in 2025-12-03 00:08:51 CET+0100 format
fn cet_to_utc(time: &str) -> Result<DateTime<Utc>, FoxError> {
    let dt = DateTime::parse_from_str(&time.replace("+", " +"), "%Y-%m-%d %H:%M:%S %Z %z")?;
    Ok(dt.with_timezone(&Utc))
}

#[derive(Serialize, Deserialize)]
struct FoxResponse {
    errno: u32,
    msg: String,
}

#[cfg(test)]
mod tests;
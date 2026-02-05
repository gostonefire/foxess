use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use chrono::{DateTime, Utc};
use md5::{Digest, Md5};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use crate::error::FoxError;
use crate::models::{DataPoint, DataSet, DeviceHistory, DeviceRealTime, FoxParameter};
use crate::models::dto::{DeviceHistoryData, DeviceHistoryResult, DeviceRealTimeResult, RequestDeviceHistoryData, RequestDeviceRealTimeData};

const REQUEST_DOMAIN: &str = "https://www.foxesscloud.com";

pub struct Fox {
    api_key: String,
    sn: String,
    client: Client,
}

impl Fox {
    /// Returns a new instance of the Fox struct
    ///
    /// # Arguments
    ///
    /// * 'api_key' - FoxESS API Key
    /// * 'sn' - FoxESS inverter serial number
    /// * 'request_timeout' - Request timeout in seconds
    pub fn new(api_key: &str, sn: &str, request_timeout: u64) -> Result<Self, FoxError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(request_timeout))
            .build()?;

        Ok(Self { api_key: api_key.to_string(), sn: sn.to_string(), client })
    }

    /// Obtain history data from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20history20data0a3ca20id3dget20device20history20data4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'start' - the start time of the report
    /// * 'end' - the end time of the report
    pub async fn get_device_history_data(&self, start: DateTime<Utc>, end: DateTime<Utc>, parameters: Vec<FoxParameter>) -> Result<DeviceHistory, FoxError> {
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
        let device_history = transform_history_data(end, fox_data.result)?;

        Ok(device_history)
    }

    /// Obtain real time data from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20real-time20data0a3ca20id3dget20device20real-time20data5603e203ca3e
    ///
    /// # Arguments
    /// 
    /// * 'parameters' - List of parameters to retrieve from the inverter
    pub async fn get_device_real_time_data(&self, parameters: Vec<FoxParameter>) -> Result<DeviceRealTime, FoxError> {
        let path = "/op/v1/device/real/query";

        let req = RequestDeviceRealTimeData {
            variables: parameters.iter().map(|p| p.as_str()).collect(),
            sns: vec![&self.sn],
        };

        let req_json = serde_json::to_string(&req)?;

        let json = self.post_request(&path, req_json).await?;

        let fox_data: DeviceRealTimeResult = serde_json::from_str(&json)?;

        let mut data_points: HashMap<FoxParameter, DataPoint<f64>> = HashMap::new();
        
        // Be defensive: Fox API returns a Vec; can't assume [0] exists.
        let Some(first) = fox_data.result.first() else {
            return Ok(DeviceRealTime {
                data_points,
            });
        };

        for data in first.datas.iter() {
            // Only accept variables that are part of FoxParameter.
            let Ok(p) = FoxParameter::from_str(data.variable.as_str()) else {
                continue;
            };
            
            let value = data.value;
            data_points.insert(p, DataPoint(value));
        }

        Ok(DeviceRealTime { data_points })
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
        let url = format!("{}{}", REQUEST_DOMAIN, path);

        //let mut req = self.client.post(url);
        let headers = self.generate_headers(&path, Some(vec!(("Content-Type", "application/json"))));

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

    /// Generates http headers required by Fox Open API, this includes also building a
    /// md5 hashed signature.
    ///
    /// # Arguments
    ///
    /// * 'path' - the path, excluding the domain part, to the FoxESS specific API
    /// * 'extra' - any extra headers to add besides FoxCloud standards
    fn generate_headers(&self, path: &str, extra: Option<Vec<(&str, &str)>>) -> HeaderMap {
        let mut headers = HeaderMap::new();

        let timestamp = Utc::now().timestamp() * 1000;
        let signature = format!("{}\\r\\n{}\\r\\n{}", path, self.api_key, timestamp);

        let mut hasher = Md5::new();
        hasher.update(signature.as_bytes());
        let signature_md5 = hasher.finalize().iter().map(|x| format!("{:02x}", x)).collect::<String>();

        headers.insert("token", HeaderValue::from_str(&self.api_key).unwrap());
        headers.insert("timestamp", HeaderValue::from_str(&timestamp.to_string()).unwrap());
        headers.insert("signature", HeaderValue::from_str(&signature_md5).unwrap());
        headers.insert("lang", HeaderValue::from_str("en").unwrap());

        if let Some(h) = extra {
            h.iter().for_each(|&(k, v)| {
                headers.insert(HeaderName::from_str(k).unwrap(), HeaderValue::from_str(v).unwrap());
            });
        }

        headers
    }
}

/// Transforms device history data to a format easier to save as non-json file
///
/// # Arguments
///
/// * 'last_end_time' - the last given end time when requesting history data
/// * 'input' - the data to transform
fn transform_history_data(last_end_time: DateTime<Utc>, input: Vec<DeviceHistoryData>) -> Result<DeviceHistory, FoxError> {
    let mut series: HashMap<FoxParameter, Vec<DataSet<f64>>> = HashMap::new();

    // Be defensive: Fox API returns a Vec; can't assume [0] exists.
    let Some(first) = input.first() else {
        return Ok(DeviceHistory {
            request_end_time: last_end_time,
            series,
        });
    };

    for set in &first.data_set {
        let utc = cet_to_utc(&set.data[0].time)?;

        // Only accept variables that are part of FoxParameter.
        let Ok(p) = FoxParameter::from_str(set.variable.as_str()) else {
            continue;
        };

        // History payload uses f64 values; store as-is.
        let value = set.data[0].value;

        series
            .entry(p)
            .or_insert_with(Vec::new)
            .push(DataSet { date_time: utc, data: value });
    }

    Ok(DeviceHistory {
        request_end_time: last_end_time,
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

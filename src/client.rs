//! FoxESS API Client implementation.
//!
//! This module provides the core [`Fox`] client, which serves as the primary entry point for
//! interacting with the FoxESS Open API. It handles authentication, request signing,
//! and provides both asynchronous and blocking implementations.
//!
//! ## Key Capabilities
//!
//! - **Real-time Data**: Retrieve current inverter metrics using [`get_variables`](Fox::get_variables)
//!   or strongly-typed variants like [`get_variable_typed`](Fox::get_variable_typed).
//! - **Historical Data**: Access historical performance data via [`get_variables_history`](Fox::get_variables_history).
//! - **Inverter Settings**: Read and modify inverter configuration using [`get_settings`](Fox::get_settings)
//!   and [`set_setting_typed`](Fox::set_setting_typed).
//! - **Battery Management**: Specific helpers for managing battery charging schedules and work modes.
//!
//! ## Usage
//!
//! The client requires a FoxESS API key and the inverter's serial number. It can be configured
//! with a custom timeout and base URL if needed (only for unit tests due to visibility).

use std::collections::HashMap;
use std::time::Duration;
use chrono::{DateTime, Utc};
use crate::client::helper::FoxHelper;
use crate::{FoxError, TimeSegmentsDataRequest};
use crate::{VariablesDataHistory, VariablesData, FoxVariables, AvailableVariables};
use crate::fox_variables::VariableSpec;
use crate::{FoxSettings,SettingsData, SettingsDataPoint};
use crate::fox_settings::{SettableSettingSpec, SettingSpec};
use crate::models::main_switch::MainSwitchStatus;
use crate::models::scheduler::TimeSegmentsData;

const DEFAULT_REQUEST_DOMAIN: &str = "https://www.foxesscloud.com";

fn default_now_millis() -> i64 {
    Utc::now().timestamp_millis()
}

/// A client for interacting with the FoxESS Open API.
///
/// The `Fox` struct provides methods to retrieve real-time data, history data,
/// and manage settings for a FoxESS inverter. It handles authentication,
/// request signing, and data parsing.
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub struct Fox {
    client: reqwest::Client,
    fox_helper: FoxHelper,
}

/// A blocking client for interacting with the FoxESS Open API.
///
/// This version of the `Fox` struct uses a blocking HTTP client.
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub struct Fox {
    client: reqwest::blocking::Client,
    fox_helper: FoxHelper,
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl Fox {
    /// Creates a new asynchronous instance of the `Fox` client.
    ///
    /// # Arguments
    /// * `api_key` - Your FoxESS API Key.
    /// * `sn` - The serial number of your FoxESS inverter.
    /// * `request_timeout` - Request timeout in seconds.
    ///
    /// # Returns
    /// * `Result<Self, FoxError>` - A new `Fox` instance or an error if the client could not be initialized.
    pub fn new(api_key: &str, sn: &str, request_timeout: u64) -> Result<Self, FoxError> {
        Self::new_with_base_url_and_clock(api_key, sn, request_timeout, DEFAULT_REQUEST_DOMAIN, default_now_millis)
    }

    /// Creates a new asynchronous instance with a custom base URL and clock function.
    ///
    /// This is primarily used for testing or when using a proxy.
    ///
    /// # Arguments
    /// * `api_key` - Your FoxESS API Key.
    /// * `sn` - The serial number of your FoxESS inverter.
    /// * `request_timeout` - Request timeout in seconds.
    /// * `base_url` - The base URL for API requests.
    /// * `now_millis` - A function that returns the current timestamp in milliseconds.
    ///
    /// # Returns
    /// * `Result<Self, FoxError>` - A new `Fox` instance or an error if initialization fails.
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
            client,
            fox_helper: FoxHelper::new(api_key, sn, base_url.trim_end_matches('/'), now_millis),
        })
    }

    /// Collects historical data from the inverter.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20history20data0a3ca20id3dget20device20history20data4303e203ca3e).
    ///
    /// # Arguments
    /// * `start` - The start time for the data range.
    /// * `end` - The end time for the data range.
    /// * `parameters` - A list of variables to retrieve.
    ///
    /// # Returns
    /// * `Result<VariablesDataHistory, FoxError>` - A structure containing the historical data points.
    pub async fn get_variables_history(&self, start: DateTime<Utc>, end: DateTime<Utc>, parameters: Vec<FoxVariables>) -> Result<VariablesDataHistory, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_variables_history(start, end, parameters)?;

        let json = self.post_request(&path, req_json).await?;

        Ok(self.fox_helper.post_get_variables_history(&json)?)
    }

    /// Collects real-time data from the inverter.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20real-time20data0a3ca20id3dget20device20real-time20data5603e203ca3e).
    ///
    /// # Arguments
    /// * `variables` - A list of variables to retrieve.
    ///
    /// # Returns
    /// * `Result<VariablesData, FoxError>` - A structure containing the latest data points.
    pub async fn get_variables(&self, variables: Vec<FoxVariables>) -> Result<VariablesData, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_variables(variables)?;

        let json = self.post_request(&path, req_json).await?;

        Ok(self.fox_helper.post_get_variables(&json)?)
    }

    /// Retrieves a single inverter variable, parsed into a strongly-typed value.
    ///
    /// This is a typed variant of [`get_variables`](Self::get_variables). Instead of passing a variable key
    /// as an argument, you specify a variable spec type `S` that implements [`VariableSpec`].
    ///
    /// The spec determines:
    /// - The variable key being fetched (`S::VARIABLE`)
    /// - How the raw numerical data (`f64`) is converted (`S::into`)
    /// - The resulting value type (`S::Value`)
    ///
    /// # Type Parameters
    /// * `S` - A type implementing [`VariableSpec`] that describes the variable.
    ///
    /// # Returns
    /// * `Result<S::Value, FoxError>` - The parsed variable value.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_variables::{PvPower, LoadsPower, SoC};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Fetch values by specifying the spec type:
    /// let pv_power: f64 = fox.get_variable_typed::<PvPower>().await?;
    /// let loads_power: f64 = fox.get_variable_typed::<LoadsPower>().await?;
    /// let soc: u8 = fox.get_variable_typed::<SoC>().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_variable_typed<S: VariableSpec>(&self) -> Result<S::Value, FoxError> {
        let data = self.get_variables(vec![S::VARIABLE]).await?
            .get(S::VARIABLE)
            .ok_or(FoxError::VariableNotFoundError {
                variable: S::VARIABLE.as_str(),
            })?;

        S::into(data)
    }

    /// Retrieves a single setting from the inverter.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20device20settings20item0a3ca20id3dget20the20device20settings20item4303e203ca3e).
    ///
    /// # Arguments
    /// * `setting` - The setting to retrieve.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The raw string value of the setting.
    pub async fn get_setting(&self, setting: FoxSettings) -> Result<String, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_setting(setting)?;

        let json = self.post_request(&path, req_json).await?;

        Ok(self.fox_helper.post_get_setting(&json)?)
    }

    /// Retrieves a single inverter setting, parsed into a strongly-typed value.
    ///
    /// This is a typed variant of [`get_setting`](Self::get_setting). Instead of passing a setting key
    /// as an argument, you specify a setting spec type `S` that implements [`SettingSpec`].
    ///
    /// The spec determines:
    /// - The setting key being fetched (`S::SETTING`)
    /// - How the raw string value is parsed (`S::parse`)
    /// - The resulting value type (`S::Value`)
    ///
    /// # Type Parameters
    /// * `S` - A type implementing [`SettingSpec`] that describes the setting.
    ///
    /// # Returns
    /// * `Result<S::Value, FoxError>` - The parsed setting value.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_settings::{MaxSoc, MinSocOnGrid, WorkMode};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Fetch settings by specifying the spec type:
    /// let max_soc: u8 = fox.get_setting_typed::<MaxSoc>().await?;
    /// let min_soc: u8 = fox.get_setting_typed::<MinSocOnGrid>().await?;
    /// let work_mode: String = fox.get_setting_typed::<WorkMode>().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_setting_typed<S: SettingSpec>(&self) -> Result<S::Value, FoxError> {
        let raw = self.get_setting(S::SETTING).await?;
        S::parse(raw)
    }

    /// Retrieves multiple settings from the inverter.
    ///
    /// # Arguments
    /// * `settings` - A list of settings to retrieve.
    ///
    /// # Returns
    /// * `Result<SettingsData, FoxError>` - A structure containing the retrieved settings.
    pub async fn get_settings(&self, settings: Vec<FoxSettings>) -> Result<SettingsData, FoxError> {
        let mut data_points: HashMap<FoxSettings, SettingsDataPoint> = HashMap::new();

        for s in settings.iter() {
            let data = self.get_setting(*s).await?;
            data_points.insert(*s, SettingsDataPoint(data));
        }

        Ok(SettingsData { data_points })
    }

    /// Sets a single inverter setting using a strongly-typed value.
    ///
    /// This is a typed variant for writing settings. Instead of passing a setting key
    /// and a raw value, you specify a setting spec type `S` that implements [`SettableSettingSpec`].
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20device20settings20item0a3ca20id3dset20the20device20settings20item4303e203ca3e).
    ///
    /// # Type Parameters
    /// * `S` - A type implementing [`SettableSettingSpec`] that describes the setting.
    ///
    /// # Arguments
    /// * `value` - The new value for the setting (type `S::Value`).
    ///
    /// # Returns
    /// * `Result<(), FoxError>` - `Ok(())` if the setting was updated successfully.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_settings::{MaxSoc, MinSocOnGrid};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Update settings by specifying the spec type:
    /// fox.set_setting_typed::<MaxSoc>(90).await?;
    /// fox.set_setting_typed::<MinSocOnGrid>(20).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_setting_typed<S: SettableSettingSpec>(&self, value: S::Value) -> Result<(), FoxError> {
        let (req_json, path) = self.fox_helper.pre_set_setting_typed::<S>(value)?;

        let _ = self.post_request(&path, req_json).await?;

        Ok(())
    }

    /// Sets the battery charging time schedule.
    ///
    /// This is the standard charging scheduler setting. Note that no time overlaps
    /// are permitted between different schedules.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20battery20charging20time0a3ca20id3dset20the20battery20charging20time4303e203ca3e).
    ///
    /// # Arguments
    /// * `enable` - Whether to enable the schedule.
    /// * `start` - The start time of the charging window.
    /// * `end` - The end time of the charging window (non-inclusive).
    ///
    /// # Returns
    /// * `Result<(), FoxError>` - `Ok(())` if the schedule was set successfully.
    pub async fn set_battery_charging_time_schedule(&self, enable: bool, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<(), FoxError> {
        let (req_json, path) = self.fox_helper.pre_set_battery_charging_time_schedule(enable, start, end)?;

        let _ = self.post_request(&path, req_json).await?;

        Ok(())
    }

    /// Disables any active charging schedule in the inverter.
    ///
    /// # Returns
    /// * `Result<(), FoxError>` - `Ok(())` if the schedule was disabled successfully.
    pub async fn disable_charge_schedule(&self) -> Result<(), FoxError> {
        self.set_battery_charging_time_schedule(
            false, Default::default(), Default::default()
        ).await
    }

    /// Gets inverter scheduler time segments.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20time20segment20information0a3ca20id3dget20the20time20segment20information14983e203ca3e)
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - A string containing the scheduler time segments.
    pub async fn get_scheduler_time_segments(&self) -> Result<TimeSegmentsData, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_scheduler_time_segments()?;

        let json = self.post_request(path, req_json).await?;

        self.fox_helper.post_get_scheduler_time_segments(&json)
    }

    /// Sets inverter scheduler time segments.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20time20segment20information0a3ca20id3dset20the20time20segment20information14983e203ca3e)
    ///
    /// # Arguments
    /// * `time_segments` - The time segments to set.
    ///
    /// # Returns
    /// * `Result<(), FoxError>` - `Ok(())` if the set scheduler time segments was successfull.
    pub async fn set_scheduler_time_segments(&self, time_segments: &TimeSegmentsDataRequest) -> Result<(), FoxError> {
        let (req_json, path) = self.fox_helper.pre_set_scheduler_time_segments(time_segments)?;

        let _ = self.post_request(path, req_json).await?;

        Ok(())
    }

    /// Gets the main switch status
    ///
    /// The result shows whether the main switch is supported and whether it is enabled.
    /// When enabled, the mode scheduler is active and controls the mode selections through its schedule,
    /// which also means that it is not possible to set the working mode through the set settings API
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20main20switch20status0a3ca20id3dget20the20main20switch20status7193e203ca3e)
    ///
    /// # Returns
    /// * `Result<MainSwitchStatus, FoxError>` - The main switch status.
    pub async fn get_main_switch_status(&self) -> Result<MainSwitchStatus, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_main_switch_status()?;

        let json = self.post_request(path, req_json).await?;

        self.fox_helper.post_get_main_switch_status(&json)
    }

    /// Sets the main switch status
    ///
    /// If the main switch status is enabled, the mode scheduler kicks in and any mode selection is done
    /// through its schedule. To be able to control working modes, the main switch must be disabled.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20main20switch20status0a3ca20id3dset20the20main20switch20status7193e203ca3e)
    ///
    /// # Arguments
    /// * `enable`- true to enable, false to disable
    pub async fn set_main_switch_status(&self, enable: bool) -> Result<(), FoxError> {
        let (req_json, path) = self.fox_helper.pre_set_main_switch_status(enable)?;

        let _ = self.post_request(path, req_json).await?;

        Ok(())
    }

    /// Gets a list of available variables from the FoxESS Cloud.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20available20variables0a3ca20id3dget20available20variables4303e203ca3e).
    ///
    /// # Returns
    /// * `Result<AvailableVariables, FoxError>` - A vector of available variables.
    pub async fn get_available_variables(&self) -> Result<AvailableVariables, FoxError> {
        let path = self.fox_helper.pre_get_available_variables()?;

        let json = self.get_request(path, None).await?;

        let available_variables = self.fox_helper.post_get_available_variables(&json)?;

        Ok(available_variables)
    }

    /// Builds and sends a POST request to the FoxESS API.
    ///
    /// This is an internal helper method that handles request signing and network communication.
    ///
    /// # Arguments
    /// * `path` - The API endpoint path (excluding the domain).
    /// * `body` - The JSON-formatted request body.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The JSON response from the API.
    async fn post_request(&self, path: &str, body: String) -> Result<String, FoxError> {
        let (url, headers) = self.fox_helper.pre_network_post_request(path);

        let req = self.client.post(url)
            .headers(headers)
            .body(body)
            .send().await?;

        let status = req.status();
        if !status.is_success() {
            return Err(FoxError::FoxCloud(format!("{:?}", status)));
        }

        Ok(self.fox_helper.post_network_post_request(req.text().await?)?)
    }

    /// Builds and sends a GET request to the FoxESS API.
    ///
    /// This is an internal helper method that handles request signing and network communication.
    ///
    /// # Arguments
    /// * `path` - The API endpoint path (excluding the domain).
    /// * `query` - The query parameters for the request.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The JSON response from the API.
    async fn get_request(&self, path: &str, query: Option<Vec<(String,String)>>) -> Result<String, FoxError> {
        let (url, headers) = self.fox_helper.pre_network_get_request(path);

        let req = self.client.get(url)
            .headers(headers)
            .query(&query)
            .send().await?;

        let status = req.status();
        if !status.is_success() {
            return Err(FoxError::FoxCloud(format!("{:?}", status)));
        }

        Ok(self.fox_helper.post_network_get_request(req.text().await?)?)
    }
}

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
impl Fox {
    /// Creates a new blocking instance of the `Fox` client.
    ///
    /// # Arguments
    /// * `api_key` - Your FoxESS API Key.
    /// * `sn` - The serial number of your FoxESS inverter.
    /// * `request_timeout` - Request timeout in seconds.
    ///
    /// # Returns
    /// * `Result<Self, FoxError>` - A new `Fox` instance or an error if the client could not be initialized.
    pub fn new(api_key: &str, sn: &str, request_timeout: u64) -> Result<Self, FoxError> {
        Self::new_with_base_url_and_clock(api_key, sn, request_timeout, DEFAULT_REQUEST_DOMAIN, default_now_millis)
    }

    /// Creates a new blocking instance with a custom base URL and clock function.
    ///
    /// This is primarily used for testing or when using a proxy.
    ///
    /// # Arguments
    /// * `api_key` - Your FoxESS API Key.
    /// * `sn` - The serial number of your FoxESS inverter.
    /// * `request_timeout` - Request timeout in seconds.
    /// * `base_url` - The base URL for API requests.
    /// * `now_millis` - A function that returns the current timestamp in milliseconds.
    ///
    /// # Returns
    /// * `Result<Self, FoxError>` - A new `Fox` instance or an error if initialization fails.
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
            client,
            fox_helper: FoxHelper::new(api_key, sn, base_url.trim_end_matches('/'), now_millis),
        })
    }

    /// Collects historical data from the inverter.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20history20data0a3ca20id3dget20device20history20data4303e203ca3e).
    ///
    /// # Arguments
    /// * `start` - The start time for the data range.
    /// * `end` - The end time for the data range.
    /// * `parameters` - A list of variables to retrieve.
    ///
    /// # Returns
    /// * `Result<VariablesDataHistory, FoxError>` - A structure containing the historical data points.
    pub fn get_variables_history(&self, start: DateTime<Utc>, end: DateTime<Utc>, parameters: Vec<FoxVariables>) -> Result<VariablesDataHistory, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_variables_history(start, end, parameters)?;

        let json = self.post_request(&path, req_json)?;

        Ok(self.fox_helper.post_get_variables_history(&json)?)
    }

    /// Collects real-time data from the inverter.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20real-time20data0a3ca20id3dget20device20real-time20data5603e203ca3e).
    ///
    /// # Arguments
    /// * `variables` - A list of variables to retrieve.
    ///
    /// # Returns
    /// * `Result<VariablesData, FoxError>` - A structure containing the latest data points.
    pub fn get_variables(&self, variables: Vec<FoxVariables>) -> Result<VariablesData, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_variables(variables)?;

        let json = self.post_request(&path, req_json)?;

        Ok(self.fox_helper.post_get_variables(&json)?)
    }

    /// Retrieves a single inverter variable, parsed into a strongly-typed value.
    ///
    /// This is a typed variant of [`get_variables`](Self::get_variables). Instead of passing a variable key
    /// as an argument, you specify a variable spec type `S` that implements [`VariableSpec`].
    ///
    /// The spec determines:
    /// - The variable key being fetched (`S::VARIABLE`)
    /// - How the raw numerical data (`f64`) is converted (`S::into`)
    /// - The resulting value type (`S::Value`)
    ///
    /// # Type Parameters
    /// * `S` - A type implementing [`VariableSpec`] that describes the variable.
    ///
    /// # Returns
    /// * `Result<S::Value, FoxError>` - The parsed variable value.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_variables::{PvPower, LoadsPower, SoC};
    ///
    /// # fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Fetch values by specifying the spec type:
    /// let pv_power: f64 = fox.get_variable_typed::<PvPower>()?;
    /// let loads_power: f64 = fox.get_variable_typed::<LoadsPower>()?;
    /// let soc: u8 = fox.get_variable_typed::<SoC>()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_variable_typed<S: VariableSpec>(&self) -> Result<S::Value, FoxError> {
        let data = self.get_variables(vec![S::VARIABLE])?
            .get(S::VARIABLE)
            .ok_or(FoxError::VariableNotFoundError {
                variable: S::VARIABLE.as_str(),
            })?;

        S::into(data)
    }

    /// Retrieves a single setting from the inverter.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20device20settings20item0a3ca20id3dget20the20device20settings20item4303e203ca3e).
    ///
    /// # Arguments
    /// * `setting` - The setting to retrieve.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The raw string value of the setting.
    pub fn get_setting(&self, setting: FoxSettings) -> Result<String, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_setting(setting)?;

        let json = self.post_request(&path, req_json)?;

        Ok(self.fox_helper.post_get_setting(&json)?)
    }

    /// Retrieves a single inverter setting, parsed into a strongly-typed value.
    ///
    /// This is a typed variant of [`get_setting`](Self::get_setting). Instead of passing a setting key
    /// as an argument, you specify a setting spec type `S` that implements [`SettingSpec`].
    ///
    /// The spec determines:
    /// - The setting key being fetched (`S::SETTING`)
    /// - How the raw string value is parsed (`S::parse`)
    /// - The resulting value type (`S::Value`)
    ///
    /// # Type Parameters
    /// * `S` - A type implementing [`SettingSpec`] that describes the setting.
    ///
    /// # Returns
    /// * `Result<S::Value, FoxError>` - The parsed setting value.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_settings::{MaxSoc, MinSocOnGrid, WorkMode};
    ///
    /// # fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Fetch settings by specifying the spec type:
    /// let max_soc: u8 = fox.get_setting_typed::<MaxSoc>()?;
    /// let min_soc: u8 = fox.get_setting_typed::<MinSocOnGrid>()?;
    /// let work_mode: String = fox.get_setting_typed::<WorkMode>()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_setting_typed<S: SettingSpec>(&self) -> Result<S::Value, FoxError> {
        let raw = self.get_setting(S::SETTING)?;
        S::parse(raw)
    }

    /// Retrieves multiple settings from the inverter.
    ///
    /// # Arguments
    /// * `settings` - A list of settings to retrieve.
    ///
    /// # Returns
    /// * `Result<SettingsData, FoxError>` - A structure containing the retrieved settings.
    pub fn get_settings(&self, settings: Vec<FoxSettings>) -> Result<SettingsData, FoxError> {
        let mut data_points: HashMap<FoxSettings, SettingsDataPoint> = HashMap::new();

        for s in settings.iter() {
            let data = self.get_setting(*s)?;
            data_points.insert(*s, SettingsDataPoint(data));
        }

        Ok(SettingsData { data_points })
    }

    /// Sets a single inverter setting using a strongly-typed value.
    ///
    /// This is a typed variant for writing settings. Instead of passing a setting key
    /// and a raw value, you specify a setting spec type `S` that implements [`SettableSettingSpec`].
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20device20settings20item0a3ca20id3dset20the20device20settings20item4303e203ca3e).
    ///
    /// # Type Parameters
    /// * `S` - A type implementing [`SettableSettingSpec`] that describes the setting.
    ///
    /// # Arguments
    /// * `value` - The new value for the setting (type `S::Value`).
    ///
    /// # Returns
    /// * `Result<(), FoxError>` - `Ok(())` if the setting was updated successfully.
    ///
    /// # Examples
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_settings::{MaxSoc, MinSocOnGrid};
    ///
    /// # fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Update settings by specifying the spec type:
    /// fox.set_setting_typed::<MaxSoc>(90)?;
    /// fox.set_setting_typed::<MinSocOnGrid>(20)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_setting_typed<S: SettableSettingSpec>(&self, value: S::Value) -> Result<(), FoxError> {
        let (req_json, path) = self.fox_helper.pre_set_setting_typed::<S>(value)?;

        let _ = self.post_request(&path, req_json)?;

        Ok(())
    }

    /// Sets the battery charging time schedule.
    ///
    /// This is the standard charging scheduler setting. Note that no time overlaps
    /// are permitted between different schedules.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20battery20charging20time0a3ca20id3dset20the20battery20charging20time4303e203ca3e).
    ///
    /// # Arguments
    /// * `enable` - Whether to enable the schedule.
    /// * `start` - The start time of the charging window.
    /// * `end` - The end time of the charging window (non-inclusive).
    ///
    /// # Returns
    /// * `Result<(), FoxError>` - `Ok(())` if the schedule was set successfully.
    pub fn set_battery_charging_time_schedule(&self, enable: bool, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<(), FoxError> {
        let (req_json, path) = self.fox_helper.pre_set_battery_charging_time_schedule(enable, start, end)?;

        let _ = self.post_request(&path, req_json)?;

        Ok(())
    }

    /// Disables any active charging schedule in the inverter.
    ///
    /// # Returns
    /// * `Result<(), FoxError>` - `Ok(())` if the schedule was disabled successfully.
    pub fn disable_charge_schedule(&self) -> Result<(), FoxError> {
        self.set_battery_charging_time_schedule(
            false, Default::default(), Default::default()
        )
    }

    /// Gets inverter scheduler time segments.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20time20segment20information0a3ca20id3dget20the20time20segment20information14983e203ca3e)
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - A string containing the scheduler time segments.
    pub fn get_scheduler_time_segments(&self) -> Result<TimeSegmentsData, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_scheduler_time_segments()?;

        let json = self.post_request(path, req_json)?;

        self.fox_helper.post_get_scheduler_time_segments(&json)
    }

    /// Sets inverter scheduler time segments.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20time20segment20information0a3ca20id3dset20the20time20segment20information14983e203ca3e)
    /// 
    /// # Arguments
    /// * `time_segments` - The time segments to set.
    /// 
    /// # Returns
    /// * `Result<(), FoxError>` - `Ok(())` if the set scheduler time segments was successfull.
    pub fn set_scheduler_time_segments(&self, time_segments: &TimeSegmentsDataRequest) -> Result<(), FoxError> {
        let (req_json, path) = self.fox_helper.pre_set_scheduler_time_segments(time_segments)?;

        let _ = self.post_request(path, req_json)?;

        Ok(())
    }

    /// Gets the main switch status
    ///
    /// The result shows whether the main switch is supported and whether it is enabled.
    /// When enabled, the mode scheduler is active and controls the mode selections through its schedule,
    /// which also means that it is not possible to set the working mode through the set settings API
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20main20switch20status0a3ca20id3dget20the20main20switch20status7193e203ca3e)
    ///
    /// # Returns
    /// * `Result<MainSwitchStatus, FoxError>` - The main switch status.
    pub fn get_main_switch_status(&self) -> Result<MainSwitchStatus, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_main_switch_status()?;

        let json = self.post_request(path, req_json)?;

        self.fox_helper.post_get_main_switch_status(&json)
    }

    /// Sets the main switch status
    ///
    /// If the main switch status is enabled, the mode scheduler kicks in and any mode selection is done
    /// through its schedule. To be able to control working modes, the main switch must be disabled.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20main20switch20status0a3ca20id3dset20the20main20switch20status7193e203ca3e)
    ///
    /// # Arguments
    /// * `enable`- true to enable, false to disable
    pub fn set_main_switch_status(&self, enable: bool) -> Result<(), FoxError> {
        let (req_json, path) = self.fox_helper.pre_set_main_switch_status(enable)?;

        let _ = self.post_request(path, req_json)?;

        Ok(())
    }

    /// Gets a list of available variables from the FoxESS Cloud.
    ///
    /// For more information, see the [FoxESS API documentation](https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20available20variables0a3ca20id3dget20available20variables4303e203ca3e).
    ///
    /// # Returns
    /// * `Result<AvailableVariables, FoxError>` - A vector of available variables.
    pub fn get_available_variables(&self) -> Result<AvailableVariables, FoxError> {
        let path = self.fox_helper.pre_get_available_variables()?;

        let json = self.get_request(path, None)?;

        let available_variables = self.fox_helper.post_get_available_variables(&json)?;

        Ok(available_variables)
    }

    /// Builds and sends a POST request to the FoxESS API.
    ///
    /// This is an internal helper method that handles request signing and network communication.
    ///
    /// # Arguments
    /// * `path` - The API endpoint path (excluding the domain).
    /// * `body` - The JSON-formatted request body.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The JSON response from the API.
    fn post_request(&self, path: &str, body: String) -> Result<String, FoxError> {
        let (url, headers) = self.fox_helper.pre_network_post_request(path);

        let req = self.client.post(url)
            .headers(headers)
            .body(body)
            .send()?;

        let status = req.status();
        if !status.is_success() {
            return Err(FoxError::FoxCloud(format!("{:?}", status)));
        }

        Ok(self.fox_helper.post_network_post_request(req.text()?)?)
    }

    /// Builds and sends a GET request to the FoxESS API.
    ///
    /// This is an internal helper method that handles request signing and network communication.
    ///
    /// # Arguments
    /// * `path` - The API endpoint path (excluding the domain).
    /// * `query` - The query parameters for the request.
    ///
    /// # Returns
    /// * `Result<String, FoxError>` - The JSON response from the API.
    fn get_request(&self, path: &str, query: Option<Vec<(String,String)>>) -> Result<String, FoxError> {
        let (url, headers) = self.fox_helper.pre_network_get_request(path);

        let req = self.client.get(url)
            .headers(headers)
            .query(&query)
            .send()?;

        let status = req.status();
        if !status.is_success() {
            return Err(FoxError::FoxCloud(format!("{:?}", status)));
        }

        Ok(self.fox_helper.post_network_get_request(req.text()?)?)
    }
}

#[cfg(test)]
mod tests;
mod helper;
use std::collections::HashMap;
use std::time::Duration;
use chrono::{DateTime, Utc};
use crate::client::helper::FoxHelper;
use crate::error::FoxError;
use crate::models::{DeviceHistory, DeviceRealTime, FoxVariables};
use crate::models::fox_settings::{FoxSettings, SettableSettingSpec, SettingSpec};
use crate::models::fox_variables::VariableSpec;
use crate::models::settings::{DeviceSettings, SettingsDataPoint};

const DEFAULT_REQUEST_DOMAIN: &str = "https://www.foxesscloud.com";

fn default_now_millis() -> i64 {
    Utc::now().timestamp() * 1000
}

#[cfg(feature = "async")]
pub struct Fox {
    client: reqwest::Client,
    fox_helper: FoxHelper,
}

#[cfg(feature = "blocking")]
pub struct Fox {
    client: reqwest::blocking::Client,
    fox_helper: FoxHelper,
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
        Self::new_with_base_url_and_clock(api_key, sn, request_timeout, DEFAULT_REQUEST_DOMAIN, default_now_millis)
    }

    /// Returns a new instance of the Fox struct (used only by unit tests)
    ///
    /// # Arguments
    ///
    /// * 'api_key' - FoxESS API Key
    /// * 'sn' - FoxESS inverter serial number
    /// * 'request_timeout' - Request timeout in seconds
    /// * 'base_url' - Base url to use
    /// * 'now_millis' - Function to get a timestamp in milliseconds
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
        let (req_json, path) = self.fox_helper.pre_get_device_history_data(start, end, parameters)?;

        let json = self.post_request(&path, req_json).await?;

        Ok(self.fox_helper.post_get_device_history_data(&json)?)
    }

    /// Collect real-time data from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20real-time20data0a3ca20id3dget20device20real-time20data5603e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'variables' - List of variables to retrieve from the inverter
    pub async fn get_device_real_time_data(&self, variables: Vec<FoxVariables>) -> Result<DeviceRealTime, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_device_real_time_data(variables)?;

        let json = self.post_request(&path, req_json).await?;

        Ok(self.fox_helper.post_get_device_real_time_data(&json)?)
    }

    /// Get a single inverter variable, parsed into a strongly-typed value.
    ///
    /// This is the typed variant of `get_variables`: instead of passing a variable key
    /// as an argument, you choose a *variable spec type* `S` that implements
    /// [`VariableSpec`]. The spec determines:
    /// - which variable key is fetched (`S::VARIABLE`)
    /// - how the raw data (f64) is cast (`S::into`)
    /// - the return type (`S::Value`)
    ///
    /// # Type Parameters
    ///
    /// * `S` - A [`VariableSpec`] describing which variable to read and how to cast it.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_variables::{PvPower, LoadsPower, SoC};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Pick the variable by choosing the spec type:
    /// let pv_power: f64 = fox.get_variable_typed::<PvPower>().await?;
    /// let loads_power: f64 = fox.get_variable_typed::<LoadsPower>().await?;
    /// let soc: u8 = fox.get_variable_typed::<SoC>().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_variable_typed<S: VariableSpec>(&self) -> Result<S::Value, FoxError> {
        let data = self.get_device_real_time_data(vec![S::VARIABLE]).await?
            .get(S::VARIABLE)
            .ok_or(FoxError::VariableNotFoundError {
                variable: S::VARIABLE.as_str(),
            })?;

        S::into(data)
    }

    /// Get setting from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20device20settings20item0a3ca20id3dget20the20device20settings20item4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'setting' - List of settings to retrieve from the inverter
    async fn get_setting(&self, setting: FoxSettings) -> Result<String, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_setting(setting)?;

        let json = self.post_request(&path, req_json).await?;

        Ok(self.fox_helper.post_get_setting(&json)?)
    }

    /// Get a single inverter setting, parsed into a strongly-typed value.
    ///
    /// This is the typed variant of `get_setting`: instead of passing a setting key
    /// as an argument, you choose a *setting spec type* `S` that implements
    /// [`SettingSpec`]. The spec determines:
    /// - which setting key is fetched (`S::SETTING`)
    /// - how the raw string value is parsed (`S::parse`)
    /// - the return type (`S::Value`)
    ///
    /// # Type Parameters
    ///
    /// * `S` - A [`SettingSpec`] describing which setting to read and how to parse it.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_settings::{MaxSoc, MinSocOnGrid, WorkMode};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Pick the setting by choosing the spec type:
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

    /// Get settings from the inverter
    ///
    /// # Arguments
    ///
    /// * 'settings' - List of settings to retrieve from the inverter
    pub async fn get_settings(&self, settings: Vec<FoxSettings>) -> Result<DeviceSettings, FoxError> {
        let mut data_points: HashMap<FoxSettings, SettingsDataPoint> = HashMap::new();

        for s in settings.iter() {
            let data = self.get_setting(*s).await?;
            data_points.insert(*s, SettingsDataPoint(data));
        }

        Ok(DeviceSettings { data_points })
    }

    /// Set a single inverter setting using a strongly-typed value.
    ///
    /// This is the typed variant of `set_setting`: instead of passing a setting key
    /// as an argument, you choose a *setting spec type* `S` that implements
    /// [`SettableSettingSpec`]. The spec determines:
    /// - which setting key is written (`S::SETTING`)
    /// - how the typed value is formatted for the API (`S::format`)
    /// - which value type is accepted (`S::Value`)
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20device20settings20item0a3ca20id3dset20the20device20settings20item4303e203ca3e
    ///
    /// # Type Parameters
    ///
    /// * `S` - A [`SettableSettingSpec`] describing which setting can be set and how to format it.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value for `S::SETTING` (type `S::Value`).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_settings::{MaxSoc, MinSocOnGrid};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Pick the setting by choosing the spec type:
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

    /// Builds a request and sends it as a POST.
    /// The return is the json representation of the result as specified by
    /// respective FoxESS API
    ///
    /// # Arguments
    ///
    /// * path - the API path excluding the domain
    /// * body - a string containing the payload in json format
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
        Self::new_with_base_url_and_clock(api_key, sn, request_timeout, DEFAULT_REQUEST_DOMAIN, default_now_millis)
    }


    /// Returns a new instance of the Fox struct (used only by unit tests)
    ///
    /// # Arguments
    ///
    /// * 'api_key' - FoxESS API Key
    /// * 'sn' - FoxESS inverter serial number
    /// * 'request_timeout' - Request timeout in seconds
    /// * 'base_url' - Base url to use
    /// * 'now_millis' - Function to get a timestamp in milliseconds
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
        let (req_json, path) = self.fox_helper.pre_get_device_history_data(start, end, parameters)?;

        let json = self.post_request(&path, req_json)?;

        Ok(self.fox_helper.post_get_device_history_data(&json)?)
    }

    /// Collect real-time data from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20device20real-time20data0a3ca20id3dget20device20real-time20data5603e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'variables' - List of variables to retrieve from the inverter
    pub fn get_device_real_time_data(&self, variables: Vec<FoxVariables>) -> Result<DeviceRealTime, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_device_real_time_data(variables)?;

        let json = self.post_request(&path, req_json)?;

        Ok(self.fox_helper.post_get_device_real_time_data(&json)?)
    }

    /// Get a single inverter variable, parsed into a strongly-typed value.
    ///
    /// This is the typed variant of `get_variables`: instead of passing a variable key
    /// as an argument, you choose a *variable spec type* `S` that implements
    /// [`VariableSpec`]. The spec determines:
    /// - which variable key is fetched (`S::VARIABLE`)
    /// - how the raw data (f64) is cast (`S::into`)
    /// - the return type (`S::Value`)
    ///
    /// # Type Parameters
    ///
    /// * `S` - A [`VariableSpec`] describing which variable to read and how to cast it.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_variables::{PvPower, LoadsPower, SoC};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Pick the variable by choosing the spec type:
    /// let pv_power: f64 = fox.get_variable_typed::<PvPower>()?;
    /// let loads_power: f64 = fox.get_variable_typed::<LoadsPower>()?;
    /// let soc: u8 = fox.get_variable_typed::<SoC>()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_variable_typed<S: VariableSpec>(&self) -> Result<S::Value, FoxError> {
        let data = self.get_device_real_time_data(vec![S::VARIABLE])?
            .get(S::VARIABLE)
            .ok_or(FoxError::VariableNotFoundError {
                variable: S::VARIABLE.as_str(),
            })?;

        S::into(data)
    }

    /// Get setting from the inverter
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#get20the20device20settings20item0a3ca20id3dget20the20device20settings20item4303e203ca3e
    ///
    /// # Arguments
    ///
    /// * 'setting' - List of settings to retrieve from the inverter
    fn get_setting(&self, setting: FoxSettings) -> Result<String, FoxError> {
        let (req_json, path) = self.fox_helper.pre_get_setting(setting)?;

        let json = self.post_request(&path, req_json)?;

        Ok(self.fox_helper.post_get_setting(&json)?)
    }

    /// Get a single inverter setting, parsed into a strongly-typed value.
    ///
    /// This is the typed variant of `get_setting`: instead of passing a setting key
    /// as an argument, you choose a *setting spec type* `S` that implements
    /// [`SettingSpec`]. The spec determines:
    /// - which setting key is fetched (`S::SETTING`)
    /// - how the raw string value is parsed (`S::parse`)
    /// - the return type (`S::Value`)
    ///
    /// # Type Parameters
    ///
    /// * `S` - A [`SettingSpec`] describing which setting to read and how to parse it.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_settings::{MaxSoc, MinSocOnGrid, WorkMode};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Pick the setting by choosing the spec type:
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

    /// Get settings from the inverter
    ///
    /// # Arguments
    ///
    /// * 'settings' - List of settings to retrieve from the inverter
    pub fn get_settings(&self, settings: Vec<FoxSettings>) -> Result<DeviceSettings, FoxError> {
        let mut data_points: HashMap<FoxSettings, SettingsDataPoint> = HashMap::new();

        for s in settings.iter() {
            let data = self.get_setting(*s)?;
            data_points.insert(*s, SettingsDataPoint(data));
        }

        Ok(DeviceSettings { data_points })
    }

    /// Set a single inverter setting using a strongly-typed value.
    ///
    /// This is the typed variant of `set_setting`: instead of passing a setting key
    /// as an argument, you choose a *setting spec type* `S` that implements
    /// [`SettableSettingSpec`]. The spec determines:
    /// - which setting key is written (`S::SETTING`)
    /// - how the typed value is formatted for the API (`S::format`)
    /// - which value type is accepted (`S::Value`)
    ///
    /// See https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html#set20the20device20settings20item0a3ca20id3dset20the20device20settings20item4303e203ca3e
    ///
    /// # Type Parameters
    ///
    /// * `S` - A [`SettableSettingSpec`] describing which setting can be set and how to format it.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value for `S::SETTING` (type `S::Value`).
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use foxess::Fox;
    /// use foxess::fox_settings::{MaxSoc, MinSocOnGrid};
    ///
    /// # async fn demo(fox: Fox) -> Result<(), foxess::FoxError> {
    /// // Pick the setting by choosing the spec type:
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

    /// Builds a request and sends it as a POST.
    /// The return is the json representation of the result as specified by
    /// respective FoxESS API
    ///
    /// # Arguments
    ///
    /// * path - the API path excluding the domain
    /// * body - a string containing the payload in json format
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
}

#[cfg(test)]
mod tests;
mod helper;
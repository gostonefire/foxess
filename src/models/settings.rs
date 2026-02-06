use std::collections::HashMap;
use crate::models::FoxSettings;

pub struct SettingsDataPoint (pub String);


pub struct DeviceSettings {
    pub(crate) data_points: HashMap<FoxSettings, SettingsDataPoint>,
}

impl DeviceSettings {
    /// Convenience: get the raw string for a setting (if present).
    pub fn get(&self, p: FoxSettings) -> Option<String> {
        self.data_points.get(&p).map(|v| v.0.clone())
    }

    /// Convenience for percent-like parameters.
    /// You can adjust rounding/clamping rules to your needs.
    pub fn get_f64(&self, p: FoxSettings) -> Result<Option<f64>, core::num::ParseFloatError> {
        self.data_points
            .get(&p)
            .map(|v| v.0.parse::<f64>())
            .transpose()
    }

    /// Convenience for percent-like parameters.
    /// You can adjust rounding/clamping rules to your needs.
    pub fn get_u8_percent(&self, p: FoxSettings) -> Result<Option<u8>, core::num::ParseIntError> {
        self.data_points
            .get(&p)
            .map(|v| v.0.parse::<u8>().map(|v| v.clamp(0, 100)))
            .transpose()
    }
}
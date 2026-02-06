use std::collections::HashMap;
use crate::models::FoxVariables;

pub struct VariableDataPoint (pub f64);


pub struct DeviceRealTime {
    pub(crate) data_points: HashMap<FoxVariables, VariableDataPoint>,
}

impl DeviceRealTime {
    /// Convenience: get the raw f64 time series for a parameter (if present).
    pub fn get(&self, p: FoxVariables) -> Option<f64> {
        self.data_points.get(&p).map(|v| v.0)
    }

    /// Convenience for percent-like parameters.
    /// You can adjust rounding/clamping rules to your needs.
    pub fn get_u8_percent(&self, p: FoxVariables) -> Option<u8> {
        self.data_points.get(&p).map(|v| v.0.round().clamp(0.0, 100.0) as u8)
    }
}

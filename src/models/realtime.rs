use std::collections::HashMap;
use crate::FoxParameter;

pub struct DataPoint<T> (pub T);


pub struct DeviceRealTime {
    pub data_points: HashMap<FoxParameter, DataPoint<f64>>,
}

impl DeviceRealTime {
    /// Convenience: get the raw f64 time series for a parameter (if present).
    pub fn get(&self, p: FoxParameter) -> Option<f64> {
        self.data_points.get(&p).map(|v| v.0)
    }

    /// Convenience for percent-like parameters.
    /// You can adjust rounding/clamping rules to your needs.
    pub fn get_u8_percent(&self, p: FoxParameter) -> Option<u8> {
        self.data_points.get(&p).map(|v| v.0.round().clamp(0.0, 100.0) as u8)
    }
}

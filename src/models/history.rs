use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::models::FoxVariables;

pub struct HistoryDataSet<T> {
    pub date_time: DateTime<Utc>,
    pub data: T,
}

pub struct DeviceHistory {
    pub(crate) series: HashMap<FoxVariables, Vec<HistoryDataSet<f64>>>,
}

impl DeviceHistory {
    /// Convenience: get the raw f64 time series for a parameter (if present).
    pub fn get(&self, p: FoxVariables) -> Option<&[HistoryDataSet<f64>]> {
        self.series.get(&p).map(|v| v.as_slice())
    }

    /// Convenience for percent-like parameters.
    /// You can adjust rounding/clamping rules to your needs.
    pub fn get_u8_percent(&self, p: FoxVariables) -> Option<Vec<HistoryDataSet<u8>>> {
        let src = self.series.get(&p)?;
        Some(
            src.iter()
                .map(|x| HistoryDataSet {
                    date_time: x.date_time,
                    data: x.data.round().clamp(0.0, 100.0) as u8,
                })
                .collect(),
        )
    }
}

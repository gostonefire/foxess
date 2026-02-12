//! The settings module contains the `SettingsData` struct and related types,
//! which are used to store and manage the current state of settings data.

use std::collections::HashMap;
use crate::models::FoxSettings;

/// A struct representing a single data point for a setting.
///
/// This struct wraps a `String` value, representing the current value of a specific setting.
///
/// # Fields
/// - `0` (`String`): The string representation of the setting's value.
/// # Example
/// ```rust
/// use foxess::SettingsDataPoint;
///
/// let setting = SettingsDataPoint("42.5".to_string());
/// println!("Value: {}", setting.0);
/// ```
pub struct SettingsDataPoint (pub String);


/// A structure that maintains the current state of settings data.
///
/// The `SettingsData` struct is used to store and manage the latest data
/// associated with various settings. The data is stored as a mapping
/// between `FoxSettings` keys and their corresponding `SettingsDataPoint`.
///
/// # Fields
///
/// * `data_points` - A `HashMap` where:
///     - The key is of type `FoxSettings`, representing the setting being tracked.
///     - The value is a `SettingsDataPoint`, representing the latest value associated
///       with the corresponding setting.
///
/// # Visibility
///
/// The `data_points` field is marked with `pub(crate)`, meaning it is publicly accessible
/// within the same crate but is not exposed publicly outside of it.
/// # Example
/// ```rust,no_run
/// use std::collections::HashMap;
/// use foxess::{SettingsData, FoxSettings, SettingsDataPoint};
///
/// # let mut data_points = HashMap::new();
/// # data_points.insert(FoxSettings::MinSocOnGrid, SettingsDataPoint("10".to_string()));
/// # let instance = unsafe { std::mem::transmute::<HashMap<FoxSettings, SettingsDataPoint>, SettingsData>(data_points) };
///
/// let value = instance.get(FoxSettings::MinSocOnGrid);
/// if let Some(v) = value {
///     println!("Min SoC: {}", v);
/// }
/// ```
pub struct SettingsData {
    pub(crate) data_points: HashMap<FoxSettings, SettingsDataPoint>,
}

impl SettingsData {
    /// Retrieves the string value of a setting associated with the given `FoxSettings key.
    ///
    /// # Arguments
    /// * `p` - A `FoxSettings` instance used as the key to look up the desired data point.
    ///
    /// # Returns
    /// * `Option<String>` -
    ///   * `Some(String)` if the key exists in the data points.
    ///   * `None` if the key does not exist.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use std::collections::HashMap;
    /// # use foxess::{SettingsData, FoxSettings, SettingsDataPoint};
    /// # let mut data_points = HashMap::new();
    /// # data_points.insert(FoxSettings::MinSocOnGrid, SettingsDataPoint("10".to_string()));
    /// # let instance = unsafe { std::mem::transmute::<HashMap<FoxSettings, SettingsDataPoint>, SettingsData>(data_points) };
    /// let value = instance.get(FoxSettings::MinSocOnGrid);
    /// ```
    pub fn get(&self, p: FoxSettings) -> Option<String> {
        self.data_points.get(&p).map(|v| v.0.clone())
    }

    /// Retrieves the value of a setting parsed as an `f64` associated with the given `FoxSettings` key.
    ///
    /// # Arguments
    /// * `p` - A `FoxSettings` instance used as the key to look up the desired data point.
    ///
    /// # Returns
    /// * `Result<Option<f64>, core::num::ParseFloatError>` -
    ///   * `Ok(Some(f64))` if the key exists and the value was successfully parsed.
    ///   * `Ok(None)` if the key does not exist.
    ///   * `Err(core::num::ParseFloatError)` if the value exists but could not be parsed as an `f64`.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use std::collections::HashMap;
    /// # use foxess::{SettingsData, FoxSettings, SettingsDataPoint};
    /// # let mut data_points = HashMap::new();
    /// # data_points.insert(FoxSettings::MinSocOnGrid, SettingsDataPoint("10.5".to_string()));
    /// # let instance = unsafe { std::mem::transmute::<HashMap<FoxSettings, SettingsDataPoint>, SettingsData>(data_points) };
    /// match instance.get_f64(FoxSettings::MinSocOnGrid) {
    ///     Ok(Some(v)) => println!("Value: {}", v),
    ///     Ok(None) => println!("Not found"),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn get_f64(&self, p: FoxSettings) -> Result<Option<f64>, core::num::ParseFloatError> {
        self.data_points
            .get(&p)
            .map(|v| v.0.parse::<f64>())
            .transpose()
    }

    /// Retrieves the value of a setting as a percentage (0-100) associated with the given `FoxSettings` key.
    ///
    /// This function attempts to parse the value associated with the provided `FoxSettings` key as a `u8`,
    /// then clamps it between 0 and 100.
    ///
    /// # Arguments
    /// * `p` - A `FoxSettings` instance used as the key to look up the desired data point.
    ///
    /// # Returns
    /// * `Result<Option<u8>, core::num::ParseIntError>` -
    ///   * `Ok(Some(u8))` if the key exists and the value was successfully parsed and clamped.
    ///   * `Ok(None)` if the key does not exist.
    ///   * `Err(core::num::ParseIntError)` if the value exists but could not be parsed as a `u8`.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use std::collections::HashMap;
    /// # use foxess::{SettingsData, FoxSettings, SettingsDataPoint};
    /// # let mut data_points = HashMap::new();
    /// # data_points.insert(FoxSettings::MinSocOnGrid, SettingsDataPoint("10".to_string()));
    /// # let instance = unsafe { std::mem::transmute::<HashMap<FoxSettings, SettingsDataPoint>, SettingsData>(data_points) };
    /// match instance.get_u8_percent(FoxSettings::MinSocOnGrid) {
    ///     Ok(Some(v)) => println!("Value: {}%", v),
    ///     Ok(None) => println!("Not found"),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn get_u8_percent(&self, p: FoxSettings) -> Result<Option<u8>, core::num::ParseIntError> {
        self.data_points
            .get(&p)
            .map(|v| v.0.parse::<u8>().map(|v| v.clamp(0, 100)))
            .transpose()
    }
}
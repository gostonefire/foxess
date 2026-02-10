//! The variables module contains the `VariablesData` struct and related types,
//! which are used to store and manage the current state of variable data.

use std::collections::HashMap;
use crate::models::FoxVariables;

/// A struct representing a single data point for a variable.
///
/// This struct wraps a `f64` value, representing the recorded value of a specific variable.
///
/// # Fields
/// - `0` (`f64`): The numerical value of the variable.
/// # Example
/// ```rust
/// use foxess::VariableDataPoint;
///
/// let data_point = VariableDataPoint(42.5);
/// println!("Value: {}", data_point.0);
/// ```
pub struct VariableDataPoint (pub f64);


/// A structure that maintains the current state of variable data.
///
/// The `VariablesData` struct is used to store and manage the latest data
/// associated with various variables. The data is stored as a mapping
/// between `FoxVariables` keys and their corresponding `VariableDataPoint`.
///
/// # Fields
///
/// * `data_points` - A `HashMap` where:
///     - The key is of type `FoxVariables`, representing the variable being tracked.
///     - The value is a `VariableDataPoint`, representing the latest value associated
///       with the corresponding variable.
///
/// # Visibility
///
/// The `data_points` field is marked with `pub(crate)`, meaning it is publicly accessible
/// within the same crate but is not exposed publicly outside of it.
/// # Example
/// ```rust,no_run
/// use std::collections::HashMap;
/// use foxess::{VariablesData, FoxVariables, VariableDataPoint};
///
/// # let mut data_points = HashMap::new();
/// # data_points.insert(FoxVariables::GenerationPower, VariableDataPoint(1200.0));
/// # let instance = unsafe { std::mem::transmute::<HashMap<FoxVariables, VariableDataPoint>, VariablesData>(data_points) };
///
/// let value = instance.get(FoxVariables::GenerationPower);
/// if let Some(v) = value {
///     println!("Generation Power: {}W", v);
/// }
/// ```
pub struct VariablesData {
    pub(crate) data_points: HashMap<FoxVariables, VariableDataPoint>,
}

impl VariablesData {
    /// Retrieves the value of a variable associated with the given `FoxVariables` key.
    ///
    /// # Arguments
    /// * `p` - A `FoxVariables` instance used as the key to look up the desired data point.
    ///
    /// # Returns
    /// * `Option<f64>` -
    ///   * `Some(f64)` if the key exists in the data points.
    ///   * `None` if the key does not exist.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use std::collections::HashMap;
    /// # use foxess::{VariablesData, FoxVariables, VariableDataPoint};
    /// # let mut data_points = HashMap::new();
    /// # data_points.insert(FoxVariables::GenerationPower, VariableDataPoint(1200.0));
    /// # let instance = unsafe { std::mem::transmute::<HashMap<FoxVariables, VariableDataPoint>, VariablesData>(data_points) };
    /// let value = instance.get(FoxVariables::GenerationPower);
    /// ```
    pub fn get(&self, p: FoxVariables) -> Option<f64> {
        self.data_points.get(&p).map(|v| v.0)
    }

    /// Retrieves the value of a variable as a percentage (0-100) associated with the given `FoxVariables` key.
    ///
    /// This function processes the data associated with the provided `FoxVariables` key, ensures that it
    /// falls within the range of 0.0 to 100.0, rounds it to the nearest whole number, and converts it into
    /// an 8-bit unsigned integer (`u8`).
    ///
    /// # Arguments
    /// * `p` - A `FoxVariables` instance that serves as the key to retrieve data.
    ///
    /// # Returns
    /// * `Option<u8>` -
    ///   * `Some(u8)` if the specified key exists, containing the rounded and clamped value.
    ///   * `None` if the key does not exist.
    ///
    /// # Example
    /// ```rust, no_run
    /// # use std::collections::HashMap;
    /// # use foxess::{VariablesData, FoxVariables, VariableDataPoint};
    /// # let mut data_points = HashMap::new();
    /// # data_points.insert(FoxVariables::SoC, VariableDataPoint(95.4));
    /// # let instance = unsafe { std::mem::transmute::<HashMap<FoxVariables, VariableDataPoint>, VariablesData>(data_points) };
    /// if let Some(soc) = instance.get_u8_percent(FoxVariables::SoC) {
    ///     println!("Battery SoC: {}%", soc);
    /// }
    /// ```
    pub fn get_u8_percent(&self, p: FoxVariables) -> Option<u8> {
        self.data_points.get(&p).map(|v| v.0.round().clamp(0.0, 100.0) as u8)
    }
}

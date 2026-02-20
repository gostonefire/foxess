//! The history module contains the `VariablesDataHistory` struct, 
//! which is used to store and manage historical data associated with various variables.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::models::FoxVariables;

/// A struct representing a dataset with associated historical timestamp information.
///
/// This generic struct is used to store a piece of data along with the specific date and time
/// when the data is relevant or recorded.
///
/// # Type Parameters
/// - `T`: The type of the data being stored in the dataset.
///
/// # Fields
/// - `date_time` (`DateTime<Utc>`):
///   The UTC timestamp indicating the date and time when the data was recorded or is relevant.
/// - `data` (`T`):
///   The actual data being stored, which can be of any type (as specified by the generic parameter `T`).
///
/// # Example
/// ```rust
/// use chrono::Utc;
/// use chrono::DateTime;
/// use foxess::VariableDataSet;
///
/// let timestamp = Utc::now();
/// let dataset = VariableDataSet {
///     date_time: timestamp,
///     data: 56,
/// };
///
/// println!("Timestamp: {}, Data: {}", dataset.date_time, dataset.data);
/// ```
#[derive(Debug)]
pub struct VariableDataSet<T> {
    /// The UTC timestamp indicating the date and time when the data was recorded or is relevant.
    pub date_time: DateTime<Utc>,
    /// The actual data being stored.
    pub data: T,
}

/// A structure that maintains a historical record of variable data.
///
/// The `VariablesDataHistory` struct is used to store and manage historical data
/// associated with various variables. The historical data is stored as a mapping
/// between `FoxVariables` keys and their corresponding series of data points.
///
/// # Fields
///
/// * `series` - A `HashMap` where:
///     - The key is of type `FoxVariables`, representing the variable being tracked.
///     - The value is a `Vec` of `VariableDataSet<f64>`, representing a series of
///       historical data points associated with the corresponding variable.
///
/// # Visibility
///
/// The `series` field is marked with `pub(crate)`, meaning it is publicly accessible
/// within the same crate but is not exposed publicly outside of it.
#[derive(Debug)]
pub struct VariablesDataHistory {
    /// A `HashMap` where:
    /// - The key is of type `FoxVariables`, representing the variable being tracked.
    /// - The value is a `Vec` of `VariableDataSet<f64>`, representing a series of
    ///   historical data points associated with the corresponding variable.
    pub(crate) series: HashMap<FoxVariables, Vec<VariableDataSet<f64>>>,
}

impl VariablesDataHistory {

    /// Retrieves a reference to a slice of `VariableDataSet<f64>` associated with the given `FoxVariables` key.
    ///
    /// # Arguments
    /// * `p` - A `FoxVariables` instance used as the key to look up the desired dataset within the series.
    ///
    /// # Returns
    /// * `Option<&[VariableDataSet<f64>]>` -
    ///   * `Some(&[VariableDataSet<f64>])` if the key exists in the series and is associated with a dataset.
    ///   * `None` if the key does not exist in the series or if there is no associated dataset.
    ///
    /// # Example
    /// ```rust,no_run
    /// use std::collections::HashMap;
    /// use foxess::{VariablesDataHistory, FoxVariables};
    ///
    /// // Example of how to use the get method
    /// // In a real scenario, you would obtain an instance of VariablesDataHistory
    /// // from the client.
    /// # use foxess::VariableDataSet;
    /// # let mut series = HashMap::new();
    /// # series.insert(FoxVariables::SoC, vec![VariableDataSet { date_time: chrono::Utc::now(), data: 95.0 }]);
    /// # let instance = unsafe { std::mem::transmute::<HashMap<FoxVariables, Vec<VariableDataSet<f64>>>, VariablesDataHistory>(series) };
    ///
    /// let result = instance.get(FoxVariables::SoC);
    /// if let Some(data) = result {
    ///     for item in data {
    ///         println!("DateTime: {}, Data: {}", item.date_time, item.data);
    ///     }
    /// }
    /// ```
    pub fn get(&self, p: FoxVariables) -> Option<&[VariableDataSet<f64>]> {
        self.series.get(&p).map(|v| v.as_slice())
    }
    
    ///  Retrieves a reference to a slice of `VariableDataSet<u8>` associated with the given `FoxVariables` key.
    ///
    ///  This function processes the data associated with the provided `FoxVariables` key, ensures that it
    ///  falls within the range of 0 to 100, rounds it to the nearest whole number, and converts it into
    ///  an 8-bit unsigned integer (`u8`). The function wraps the output in an `Option`, returning `None`
    ///  if the key does not exist in the series or `Some(Vec<VariableDataSet<u8>>)` containing the processed data.
    /// 
    ///  # Arguments
    ///  * `p` - A `FoxVariables` instance that serves as the key to retrieve data from the series.
    ///
    ///  # Returns
    ///  * `Some(Vec<VariableDataSet<u8>>)` if the specified key exists in the series, containing data sets
    ///     where each value is rounded, clamped between 0 and 100, and converted to `u8`.
    ///  * `None` if the key does not exist in the series.
    ///
    ///  # Example
    ///  ```rust,no_run
    ///  # use std::collections::HashMap;
    ///  # use foxess::{VariablesDataHistory, FoxVariables, VariableDataSet};
    ///  # let mut series = HashMap::new();
    ///  # series.insert(FoxVariables::SoC, vec![VariableDataSet { date_time: chrono::Utc::now(), data: 95.0 }]);
    ///  # let instance = unsafe { std::mem::transmute::<HashMap<FoxVariables, Vec<VariableDataSet<f64>>>, VariablesDataHistory>(series) };
    ///  let result = instance.get_u8_percent(FoxVariables::SoC);
    ///  match result {
    ///     Some(data) => {
    ///         for item in data {
    ///             println!("DateTime: {}, Data: {}", item.date_time, item.data);
    ///         }
    ///     },
    ///     None => {
    ///         println!("Key not found in series.");
    ///     },
    ///  }
    /// ```
    pub fn get_u8_percent(&self, p: FoxVariables) -> Option<Vec<VariableDataSet<u8>>> {
        let src = self.series.get(&p)?;
        Some(
            src.iter()
                .map(|x| VariableDataSet {
                    date_time: x.date_time,
                    data: x.data.round().clamp(0.0, 100.0) as u8,
                })
                .collect(),
        )
    }
}

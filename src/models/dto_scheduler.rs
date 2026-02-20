//! Request DTOs for FoxESS API operations.
//!

use serde::{Deserialize};

/// Work mode information including available modes and their units.
#[derive(Deserialize)]
pub struct WorkMode {
    /// List of available work modes as strings.
    #[serde(rename = "enumList")]
    pub enum_list: Vec<String>,
    /// Unit of measurement for the work mode values.
    pub unit: String,
    /// Precision for values in this work mode.
    pub precision: f64,
}

/// A numeric range with minimum and maximum values.
#[derive(Deserialize)]
pub struct Range {
    /// Minimum value in the range.
    pub min: f64,
    /// Maximum value in the range.
    pub max: f64,
}

/// Detailed structure for a parameter including unit, precision, and range.
#[derive(Deserialize)]
pub struct Structure {
    /// Unit of measurement for the parameter.
    pub unit: String,
    /// Precision of the parameter value.
    pub precision: f64,
    /// Valid range of the parameter value.
    pub range: Range,
}

/// Metadata properties defining the constraints for scheduler parameters.
#[derive(Deserialize)]
pub struct Properties {
    /// Start minute constraints.
    #[serde(rename = "startminute")]
    pub start_minute: Structure,
    /// Feed-in power (or charge/discharge power) constraints.
    #[serde(rename = "fdpwr")]
    pub fd_pwr: Structure,
    /// End hour constraints.
    #[serde(rename = "endhour")]
    pub end_hour: Structure,
    /// End minute constraints.
    #[serde(rename = "endminute")]
    pub end_minute: Structure,
    /// Feed-in State of Charge (SoC) constraints.
    #[serde(rename = "fdsoc")]
    pub fd_soc: Structure,
    /// Start hour constraints.
    #[serde(rename = "starthour")]
    pub start_hour: Structure,
    /// Work mode constraints and options.
    #[serde(rename = "workmode")]
    pub work_mode: WorkMode,
    /// Minimum State of Charge (SoC) when on grid.
    #[serde(rename = "minsocongrid")]
    pub min_soc_on_grid: Structure,
    /// Maximum State of Charge (SoC).
    #[serde(rename = "maxsoc")]
    pub max_soc: Structure,
}

/// Additional parameters for specific work modes or settings.
#[derive(Deserialize)]
pub struct ExtraParam {
    /// Feed-in power limit.
    #[serde(rename = "fdPwr")]
    pub fd_pwr: Option<f64>,
    /// Minimum State of Charge (SoC) when on grid.
    #[serde(rename = "minSocOnGrid")]
    pub min_soc_on_grid: Option<f64>,
    /// Feed-in State of Charge (SoC) limit.
    #[serde(rename = "fdSoc")]
    pub fd_soc: Option<f64>,
    /// Maximum State of Charge (SoC).
    #[serde(rename = "maxSoc")]
    pub max_soc: Option<f64>,
    /// Import power limit from the grid.
    #[serde(rename = "importLimit")]
    pub import_limit: Option<f64>,
    /// Export power limit to the grid.
    #[serde(rename = "exportLimit")]
    pub export_limit: Option<f64>,
    /// Photovoltaic (PV) power limit.
    #[serde(rename = "pvLimit")]
    pub pv_limit: Option<f64>,
    /// Reactive power settings.
    #[serde(rename = "reactivePower")]
    pub reactive_power: Option<f64>,
}

/// A scheduled task group defining a work mode for a specific time window.
#[derive(Deserialize)]
pub struct Group {
    /// End hour of the scheduled window.
    #[serde(rename = "endHour")]
    pub end_hour: i64,
    /// Work mode to be used during this time window.
    #[serde(rename = "workMode")]
    pub work_mode: String,
    /// Start hour of the scheduled window.
    #[serde(rename = "startHour")]
    pub start_hour: i64,
    /// Additional parameters specific to this group.
    #[serde(rename = "extraParam")]
    pub extra_param: Option<ExtraParam>,
    /// Start minute of the scheduled window.
    #[serde(rename = "startMinute")]
    pub start_minute: i64,
    /// End minute of the scheduled window.
    #[serde(rename = "endMinute")]
    pub end_minute: i64,
}

/// Information about a time-series based scheduler configuration.
#[derive(Deserialize)]
pub struct TimeSeriesInfo {
    /// Whether the scheduler is enabled (1) or disabled (0).
    pub enable: i64,
    /// Maximum number of schedule groups allowed.
    #[serde(rename = "maxGroupCount")]
    pub max_group_count: i64,
    /// List of scheduled groups.
    pub groups: Vec<Group>,
    /// Metadata properties for the schedule.
    pub properties: Properties,
}

/// Result of a scheduler time series query from the FoxESS API.
#[derive(Deserialize)]
pub struct SchedulerTimeSeriesResult {
    /// The actual scheduler information.
    pub result: TimeSeriesInfo,
}



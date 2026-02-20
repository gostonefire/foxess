//! Exported API models for the FoxESS scheduler.
//!
//! This module contains types that represent the scheduler state and configuration
//! in the API, providing a more convenient representation compared to the raw DTOs.

use crate::FoxWorkModes;

/// Work mode information including available modes and their units.
#[derive(Debug)]
pub struct WorkMode {
    /// List of available work modes.
    pub enum_list: Vec<FoxWorkModes>,
    /// Unit of measurement for the work mode values.
    pub unit: String,
    /// Precision for values in this work mode.
    pub precision: f64,
}

/// A numeric range with minimum and maximum values.
#[derive(Debug)]
pub struct Range {
    /// Minimum value in the range.
    pub min: f64,
    /// Maximum value in the range.
    pub max: f64,
}

/// Detailed structure for a parameter including unit, precision, and range.
#[derive(Debug)]
pub struct MetaData {
    /// Unit of measurement for the parameter.
    pub unit: String,
    /// Precision of the parameter value.
    pub precision: f64,
    /// Valid range of the parameter value.
    pub range: Range,
}

/// Metadata properties defining the constraints for scheduler parameters.
#[derive(Debug)]
pub struct Properties {
    /// Start minute constraints.
    pub start_minute: MetaData,
    /// Feed-in power (or charge/discharge power) constraints.
    pub fd_pwr: MetaData,
    /// End hour constraints.
    pub end_hour: MetaData,
    /// End minute constraints.
    pub end_minute: MetaData,
    /// Feed-in State of Charge (SoC) constraints.
    pub fd_soc: MetaData,
    /// Start hour constraints.
    pub start_hour: MetaData,
    /// Work mode constraints and options.
    pub work_mode: WorkMode,
    /// Minimum State of Charge (SoC) when on grid.
    pub min_soc_on_grid: MetaData,
    /// Maximum State of Charge (SoC).
    pub max_soc: MetaData,
}

/// Additional parameters for specific work modes or settings.
#[derive(Debug)]
pub struct ExtraParam {
    /// Feed-in power limit.
    pub fd_pwr: Option<f64>,
    /// Minimum State of Charge (SoC) when on grid.
    pub min_soc_on_grid: Option<f64>,
    /// Feed-in State of Charge (SoC) limit.
    pub fd_soc: Option<f64>,
    /// Maximum State of Charge (SoC).
    pub max_soc: Option<f64>,
    /// Import power limit from the grid.
    pub import_limit: Option<f64>,
    /// Export power limit to the grid.
    pub export_limit: Option<f64>,
    /// Photovoltaic (PV) power limit.
    pub pv_limit: Option<f64>,
    /// Reactive power settings.
    pub reactive_power: Option<f64>,
}

/// A scheduled task group defining a work mode for a specific time window.
#[derive(Debug)]
pub struct Group {
    /// End hour of the scheduled window.
    pub end_hour: i64,
    /// Work mode to be used during this time window.
    pub work_mode: FoxWorkModes,
    /// Start hour of the scheduled window.
    pub start_hour: i64,
    /// Additional parameters specific to this group.
    pub extra_param: Option<ExtraParam>,
    /// Start minute of the scheduled window.
    pub start_minute: i64,
    /// End minute of the scheduled window.
    pub end_minute: i64,
}

/// Information about a time-series based scheduler configuration.
#[derive(Debug)]
pub struct TimeSeriesData {
    /// Whether the scheduler is enabled (1) or disabled (0).
    pub enable: i64,
    /// Maximum number of schedule groups allowed.
    pub max_group_count: i64,
    /// List of scheduled groups.
    pub groups: Vec<Group>,
    /// Metadata properties for the schedule.
    pub properties: Properties,
}

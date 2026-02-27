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
    /// The FC/FDPwr defines the maximum charging or discharging power allowed when the system is in Force Charge/Force Discharge Mode.
    /// The maximum AC input power the system can draw from the grid to charge the battery in Force Charge.
    /// The maximum AC output power the system is allowed to deliver during Forced Discharge.
    pub fd_pwr: Option<f64>,
    /// Minimum State of Charge (SoC) when on grid.
    pub min_soc_on_grid: Option<f64>,
    /// The FC/FDSoC is the SoC level set for the system when operating in Force Charge/Force Discharge Mode.
    /// Once the battery reaches this SoC, the system will automatically stop charging or discharging.
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
    /// Start hour of the scheduled window.
    pub start_hour: i64,
    /// Start minute of the scheduled window.
    pub start_minute: i64,
    /// End hour of the scheduled window.
    pub end_hour: i64,
    /// End minute of the scheduled window.
    pub end_minute: i64,
    /// Work mode to be used during this time window.
    pub work_mode: FoxWorkModes,
    /// Additional parameters specific to this group.
    pub extra_param: Option<ExtraParam>,
}

/// Information about a time-segments based scheduler configuration.
#[derive(Debug)]
pub struct TimeSegmentsData {
    /// Whether the scheduler is enabled (1) or disabled (0).
    pub enable: i64,
    /// Maximum number of schedule groups allowed.
    pub max_group_count: i64,
    /// List of scheduled groups.
    pub groups: Vec<Group>,
    /// Metadata properties for the schedule.
    pub properties: Properties,
}

/// Data to set as time-segments for the scheduler
pub struct TimeSegmentsDataRequest {
    /// * 'false' - Parameters not provided in `extraParam` remain unchanged. This is the default value if not provided.
    /// * 'true' - Parameters not provided in `extraParam` are restored to system defaults.
    pub is_default: Option<bool>,
    /// List of groups to schedule.
    pub groups: Vec<Group>,
}
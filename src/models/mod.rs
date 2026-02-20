pub(crate) mod history;
pub(crate) mod fox_variables;
pub(crate) mod variables;
pub(crate) mod dto;
pub(crate) mod settings;
pub(crate) mod fox_settings;
pub(crate) mod dto_scheduler;
pub(crate) mod fox_scheduler;
pub(crate) mod scheduler;

pub use history::{VariableDataSet, VariablesDataHistory};
pub use fox_variables::id::FoxVariables;
pub use fox_settings::id::FoxSettings;
pub use fox_scheduler::id::FoxWorkModes;
pub use variables::{VariableDataPoint, VariablesData, AvailableVariables, VariableInfo};
pub use settings::{SettingsData, SettingsDataPoint};
pub use scheduler::{TimeSeriesData, Group, ExtraParam, Properties, MetaData, Range, WorkMode};


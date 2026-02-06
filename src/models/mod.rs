pub mod history;
pub mod fox_variables;
pub mod realtime;
pub(crate) mod dto;
pub mod fox_settings;
pub mod settings;

pub use history::{HistoryDataSet, DeviceHistory};
pub use fox_variables::FoxVariables;
pub use fox_settings::FoxSettings;
pub use realtime::{VariableDataPoint, DeviceRealTime};
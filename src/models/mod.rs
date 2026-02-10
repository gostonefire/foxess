pub mod history;
pub mod fox_variables;
pub mod variables;
pub(crate) mod dto;
pub mod fox_settings;
pub mod settings;

pub use history::{VariableDataSet, VariablesDataHistory};
pub use fox_variables::FoxVariables;
pub use fox_settings::FoxSettings;
pub use variables::{VariableDataPoint, VariablesData};
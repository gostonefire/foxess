pub mod history;
pub mod parameter;
pub mod realtime;

pub(crate) mod dto;

pub use history::{DataSet, DeviceHistory};
pub use parameter::FoxParameter;
pub use realtime::{DataPoint, DeviceRealTime};
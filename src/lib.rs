mod models;
mod error;
mod client;

#[cfg(all(feature = "async", feature = "blocking"))]
compile_error!("Features 'async' and 'blocking' are mutually exclusive. Enable only one.");

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("Enable one of the features: 'async' (default) or 'blocking'.");

pub use client::Fox;
pub use models::FoxVariables;
pub use models::FoxSettings;
pub use models::{ExportLimit, MinSocOnGrid, MaxSoc, WorkMode, MaxSetChargeCurrent};
pub use models::{PvPower, LoadsPower, SoC, SoH};
pub use error::FoxError;

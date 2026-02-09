mod models;
mod error;
mod client;

#[cfg(all(feature = "async", feature = "blocking"))]
compile_error!("Features 'async' and 'blocking' are mutually exclusive. Enable only one.");

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("Enable one of the features: 'async' (default) or 'blocking'.");

pub use client::Fox;
pub use error::FoxError;
pub use models::FoxVariables;
pub use models::FoxSettings;

pub mod fox_settings {
    pub use crate::models::fox_settings::{
        ExportLimit,
        MinSocOnGrid,
        MaxSoc,
        WorkMode,
        MaxSetChargeCurrent,
        SettingSpec,
        SettableSettingSpec,
    };
}

pub mod fox_variables {
    pub use crate::models::fox_variables::{
        PvPower,
        LoadsPower,
        SoC,
        SoH,
        BatTemperature,
        VariableSpec,
    };
}

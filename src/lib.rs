#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
//! # Foxess API client library
//!
//! The foxess crate implements a subset of available [FoxESS Cloud APIs].
//!
//! Its purpose is mainly focused on APIs that help in executing automatic scheduling of battery charging and battery discharging (self-use) given external data such as tariffs from e.g.:
//! * [Nordpool] in the Nordic European region or some other supplier of daily tariffs.
//! * Weather temperature forecast data to estimate household power consumption
//! * Weather cloud forecast data and sun incidence calculations to estimate PV power production
//! * Etc. depending on level of ambition/precision in estimates
//!
//! The APIs are tested for a Fox H3 model SK-HWR-12, and although the FoxESS Cloud APIs are general, settings and variables are not guaranteed to be fully supported by all inverters.
//! ## License
//! This library comes with a standard [MIT license]
//!
//! ## Usage Overview
//! Note down your inverter serial number, can be found from within the FoxCloud2.0 app or the [FoxESS Cloud V2 site] web site.
//!
//! Get an API key, it can be retrieved from the [FoxESS Cloud V1 site] under User Profile/API Management.
//!
//! Decide whether to use the blocking or non-blocking feature in cargo.toml dependencies
//! ```toml
//! [dependencies]
//! // Non-blocking (async)
//! foxess = “0.x.y”
//!
//! // Non-blocking (async) if you want clarity
//! foxess = { version = “0.x.y”, features = ["async"] }
//!
//! // Blocking
//! foxess = { version = “0.x.y”, default-features = false, features = [“blocking”] }
//! ```
//! ## Examples
//! ### Non-blocking request for battery State of Charge
//! ```rust,no_run
//! use foxess::Fox;
//! use foxess::fox_variables::SoC;
//!
//! # let rt = tokio::runtime::Runtime::new().unwrap();
//! # rt.block_on(async {
//! let api_key = "my_api_key";
//! let sn = "my_inverter_sn";
//!
//! let fox = Fox::new(api_key, sn, 30).unwrap();
//! let soc = fox.get_variable_typed::<SoC>().await.unwrap();
//!
//! println!("Current battery State of Charge: {}%", soc);
//! # });
//! ```
//! ### Non-blocking request for one-day PV power (Photo Voltaic), loads power (household load) and battery SoC
//! ```rust,no_run
//! use std::ops::Add;
//! use chrono::{TimeZone, Utc, Local, Duration};
//! use foxess::{Fox, FoxVariables};
//!
//! # let rt = tokio::runtime::Runtime::new().unwrap();
//! # rt.block_on(async {
//! let api_key = "my_api_key";
//! let sn = "my_inverter_sn";
//!
//! let start = Local.with_ymd_and_hms(2026, 2, 1, 0, 0, 0).unwrap().with_timezone(&Utc);
//! let end = start.add(Duration::days(1));
//! let variables = vec![FoxVariables::PvPower, FoxVariables::LoadsPower, FoxVariables::SoC];
//!
//! let fox = Fox::new(api_key, sn, 30).unwrap();
//! let history = fox.get_variables_history(start, end, variables).await.unwrap();
//!
//! let pv_history = history.get(FoxVariables::PvPower);
//! let loads_history = history.get(FoxVariables::LoadsPower);
//! let soc_history = history.get_u8_percent(FoxVariables::SoC);
//! let soh_history = history.get_u8_percent(FoxVariables::SOH);
//!
//! // Expect these to have values
//! assert!(pv_history.is_some());
//! assert!(loads_history.is_some());
//! assert!(soc_history.is_some());
//!
//! // Expect this to be None
//! assert!(soh_history.is_none());
//!
//! if let Some(first) = pv_history.unwrap().first() {
//!     println!("PV power: {} W at {:?}", first.data, first.date_time);
//! }
//! # });
//! ```
//!
//! [MIT license]: https://github.com/gostonefire/foxess/blob/main/LICENSE
//! [FoxESS Cloud APIs]: https://www.foxesscloud.com/public/i18n/en/OpenApiDocument.html
//! [Nordpool]: https://data.nordpoolgroup.com/auction/day-ahead/prices
//! [FoxESS Cloud V2 site]: https://www.foxesscloud.com/v2
//! [FoxESS Cloud V1 site]: https://www.foxesscloud.com/user/center
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
pub use models::{AvailableVariables, VariablesData, VariableDataSet, VariablesDataHistory, VariableDataPoint};
pub use models::{SettingsData, SettingsDataPoint};

pub mod fox_settings {
    //! This module re-exports the fox_settings module from the models module.
    //!

    pub use crate::models::fox_settings::builtins::{
        ExportLimit,
        MinSocOnGrid,
        MaxSoc,
        WorkMode,
        MaxSetChargeCurrent,
    };
    
    pub use crate::models::fox_settings::spec::SettingSpec;
    pub use crate::models::fox_settings::settable_spec::SettableSettingSpec;
}

pub mod fox_variables {
    //! This module re-exports the fox_variables module from the models module.
    //!

    pub use crate::models::fox_variables::builtins::{
        PvPower,
        LoadsPower,
        SoC,
        SoH,
        BatTemperature,
        RunningState,
    };

    pub use crate::models::fox_variables::spec::VariableSpec;
}

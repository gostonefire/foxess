//! This module hosts the [FoxSettings] enumeration mainly used when requesting several settings from FoxESS cloud at once.
//!
//! FoxESS Cloud currently supports many settings, and the foxess crate implements the currently available ones.
//! More might be added over time, hence the [FoxSettings] enumeration is derived non-exhaustive.
//!

use core::str::FromStr;

/// Defines the `FoxSettings` enum + `as_str()` + `FromStr` from a single registry.
///
/// Usage:
/// ```rust,ignore
/// define_fox_settings! {
///     [ActivePowerLimit, "ActivePowerLimit", "The limit on the inverter’s active power output"],
///     [ECOMode, "ECOMode", "Enables or configures energy-saving mode, typically optimizing battery charge/discharge based on time-of-use or self-consumption strategy"],
///     [EpsOutPut, "EpsOutPut", "Controls whether the EPS (Emergency Power Supply) backup output is enabled during grid outages"],
///     [ExportLimit, "ExportLimit", "Enables or disables the inverter’s grid export power limiting function"],
/// }
/// ```
macro_rules! define_fox_settings {
    (
        $(
            [$variant:ident, $key:literal, $doc:literal]
        ),+ $(,)?
    ) => {
        /// An enumeration representing implemented settings from FoxESS cloud, i.e., a subset from available settings.
        ///
        /// These settings can be retrieved or set on FoxESS devices.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum FoxSettings {
            $(
                #[doc = $doc]
                $variant,
            )+
        }

        impl FoxSettings {
            /// Returns the string representation of the `FoxSettings` enum variant.
            ///
            /// This string matches the key used by the FoxESS cloud API.
            ///
            /// # Returns
            /// * `&'static str` - The string representation of the setting.
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $key,
                    )+
                }
            }
        }

        impl FromStr for FoxSettings {
            type Err = ();

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $key => Ok(Self::$variant),
                    )+
                    _ => Err(()),
                }
            }
        }
    };
}

// ---- registry ---------------------------------------------------------------

define_fox_settings! {
    [ActivePowerLimit, "ActivePowerLimit", "The limit on the inverter’s active power output"],
    [ECOMode, "ECOMode", "Enables or configures energy-saving mode, typically optimizing battery charge/discharge based on time-of-use or self-consumption strategy"],
    [EpsOutPut, "EpsOutPut", "Controls whether the EPS (Emergency Power Supply) backup output is enabled during grid outages"],
    [ExportLimit, "ExportLimit", "Enables or disables the inverter’s grid export power limiting function"],
    [ExportLimitPower, "ExportLimitPower", "Defines the maximum allowed power (in watts or percent) that can be exported to the grid"],
    [GridCode, "GridCode", "Selects the country- or utility-specific grid compliance standard the inverter must follow"],
    [GroundProtection, "GroundProtection", "Enables ground fault detection and protection functionality"],
    [MaxSetChargeCurrent, "MaxSetChargeCurrent", "The maximum allowed battery charging current set for the inverter"],
    [MaxSetDischargeCurrent, "MaxSetDischargeCurrent", "The maximum allowed battery discharging current set for the inverter"],
    [MaxSoc, "MaxSoc", "The maximum battery state of charge (SOC) the system will charge up to"],
    [Meter1Enable, "Meter1Enable", "Enables communication with and use of the primary external energy meter"],
    [Meter2Enable, "Meter2Enable", "Enables communication with and use of a secondary external energy meter"],
    [MinSoc, "MinSoc", "The minimum battery state of charge (SOC) allowed during normal operation"],
    [MinSocOnGrid, "MinSocOnGrid", "The minimum battery state of charge (SOC) maintained while the grid is available"],
    [SysSwitch, "SysSwitch", "Master system enable/disable switch for inverter operation"],
    [WorkMode, "WorkMode", "Defines the inverter’s operating mode (e.g., self-consumption, feed-in priority, backup, or time-of-use mode)"],
}
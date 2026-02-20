//! This module hosts the [FoxWorkModes] enumeration mainly used when managing scheduler work modes.
//!
//! More might be added over time, hence the [FoxWorkModes] enumeration is derived non-exhaustive.
//!

use std::str::FromStr;

/// An enumeration representing the available scheduler work modes from FoxESS cloud.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FoxWorkModes {
    /// Forces the battery to discharge.
    ForceDischarge,
    /// Peak shaving mode.
    PeakShaving,
    /// Feed-in mode.
    Feedin,
    /// Backup mode.
    Backup,
    /// Self-use mode.
    SelfUse,
    /// Forces the battery to charge.
    ForceCharge,
    /// Unknown work mode.
    Unknown,
}

impl FoxWorkModes {
    /// Returns the string representation of the `FoxWorkModes` enum variant.
    ///
    /// This string matches the key used by the FoxESS cloud API.
    ///
    /// # Returns
    /// * `&'static str` - The string representation of the work mode.
    pub const fn as_str(&self) -> &'static str {
        match self {
            FoxWorkModes::ForceDischarge => "ForceDischarge",
            FoxWorkModes::PeakShaving => "PeakShaving",
            FoxWorkModes::Feedin => "Feedin",
            FoxWorkModes::Backup => "Backup",
            FoxWorkModes::SelfUse => "SelfUse",
            FoxWorkModes::ForceCharge => "ForceCharge",
            FoxWorkModes::Unknown => "Unknown",
        }
    }
}

impl FromStr for FoxWorkModes {
    type Err = ();

    /// Parses a string into a `FoxWorkModes` enum variant.
    ///
    /// # Arguments
    /// * `s` - A string slice representing the work mode.
    ///
    /// # Returns
    /// * `Result<Self, Self::Err>` - The parsed work mode or an error if the string is invalid.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ForceDischarge" => Ok(FoxWorkModes::ForceDischarge),
            "PeakShaving" => Ok(FoxWorkModes::PeakShaving),
            "Feedin" => Ok(FoxWorkModes::Feedin),
            "Backup" => Ok(FoxWorkModes::Backup),
            "SelfUse" => Ok(FoxWorkModes::SelfUse),
            "ForceCharge" => Ok(FoxWorkModes::ForceCharge),
            _ => Ok(FoxWorkModes::Unknown),
        }
    }
}
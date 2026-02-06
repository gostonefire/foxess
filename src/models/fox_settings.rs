use std::str::FromStr;

// Available settings from FoxESS cloud:
// * ExportLimit
// * MinSoc
// * MinSocOnGrid
// * MaxSoc
// * GridCode
// * WorkMode
// * ActivePowerLimit
// * ExportLimitPower
// * EpsOutPut
// * MaxSetChargeCurrent
// * MaxSetDischargeCurrent
// * ECOMode
// * Meter1Enable
// * Meter2Enable
// * SysSwitch
// * GroundProtection

const EXPORT_LIMIT: &str = "ExportLimit";
const MIN_SOC_ON_GRID: &str = "MinSocOnGrid";
const MAX_SOC: &str = "MaxSoc";
const WORK_MODE: &str = "WorkMode";
const MAX_SET_CHARGE_CURRENT: &str = "MaxSetChargeCurrent";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoxSettings {
    ExportLimit,
    MinSocOnGrid,
    MaxSoc,
    WorkMode,
    MaxSetChargeCurrent,
}

impl FoxSettings {
    /// Returns the string representation of the FoxSettings enum variant
    /// 
    pub const fn as_str(&self) -> &'static str {
        match self {
            FoxSettings::ExportLimit => EXPORT_LIMIT,
            FoxSettings::MinSocOnGrid => MIN_SOC_ON_GRID,
            FoxSettings::MaxSoc => MAX_SOC,
            FoxSettings::WorkMode => WORK_MODE,
            FoxSettings::MaxSetChargeCurrent => MAX_SET_CHARGE_CURRENT,
        }
    }
    
    /// Returns true if the setting is allowed to be set
    /// 
    pub fn set_allowed(&self) -> bool {
        matches!(self, FoxSettings::MinSocOnGrid | FoxSettings::MaxSoc)
    }
}

impl FromStr for FoxSettings {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            EXPORT_LIMIT => Ok(FoxSettings::ExportLimit),
            MIN_SOC_ON_GRID => Ok(FoxSettings::MinSocOnGrid),
            MAX_SOC => Ok(FoxSettings::MaxSoc),
            WORK_MODE => Ok(FoxSettings::WorkMode),
            MAX_SET_CHARGE_CURRENT => Ok(FoxSettings::MaxSetChargeCurrent),
            _ => Err(()),
        }
    }
}
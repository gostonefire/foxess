use std::str::FromStr;

const PV_POWER: &str = "pvPower";
const LOADS_POWER: &str = "loadsPower";
const SOC: &str = "SoC";
const SOH: &str = "SOH";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoxVariables {
    PvPower,
    LoadsPower,
    SoC,
    SoH,
}

impl FoxVariables {
    pub const fn as_str(&self) -> &'static str {
        match self {
            FoxVariables::PvPower => PV_POWER,
            FoxVariables::LoadsPower => LOADS_POWER,
            FoxVariables::SoC => SOC,
            FoxVariables::SoH => SOH,
        }
    }
}
impl FromStr for FoxVariables {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PV_POWER => Ok(FoxVariables::PvPower),
            LOADS_POWER => Ok(FoxVariables::LoadsPower),
            SOC => Ok(FoxVariables::SoC),
            SOH => Ok(FoxVariables::SoH),
            _ => Err(()),
        }
    }
}
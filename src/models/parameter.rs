use std::str::FromStr;

const PV_POWER: &str = "pvPower";
const LOADS_POWER: &str = "loadsPower";
const SOC: &str = "SoC";
const SOH: &str = "SOH";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoxParameter {
    PvPower,
    LoadsPower,
    SoC,
    SoH,
}

impl FoxParameter {
    pub const fn as_str(&self) -> &'static str {
        match self {
            FoxParameter::PvPower => PV_POWER,
            FoxParameter::LoadsPower => LOADS_POWER,
            FoxParameter::SoC => SOC,
            FoxParameter::SoH => SOH,
        }
    }
}
impl FromStr for FoxParameter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PV_POWER => Ok(FoxParameter::PvPower),
            LOADS_POWER => Ok(FoxParameter::LoadsPower),
            SOC => Ok(FoxParameter::SoC),
            SOH => Ok(FoxParameter::SoH),
            _ => Err(()),
        }
    }
}
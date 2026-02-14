//! This module hosts the [FoxVariables] enumeration mainly used when requesting several values from FoxESS cloud at once.
//! 
//! FoxESS Cloud currently supports many variables, and the foxess crate implements the currently available ones.
//! More might be added over time, hence the [FoxVariables] enumeration is derived non-exhaustive.
//! 

use core::str::FromStr;

/// Defines the `FoxVariables` enum + `as_str()` + `FromStr` from a single registry.
///
/// Usage:
/// ```rust,ignore
/// define_fox_variables! {
///     [AmbientTemperation, "ambientTemperation", "AmbientTemperature, unit: ℃"],
///     [BatChargePower, "batChargePower", "Charge Power, unit: kW"],
///     [BatCurrent, "batCurrent", "BatCurrent, unit: A"],
///     [BatDischargePower, "batDischargePower", "Discharge Power, unit: kW"],
/// }
/// ```
macro_rules! define_fox_variables {
    (
        $(
            [$variant:ident, $key:literal, $doc:literal]
        ),+ $(,)?
    ) => {
        /// An enumeration representing the available real-time variables from FoxESS cloud.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum FoxVariables {
            $(
                #[doc = $doc]
                $variant,
            )+
        }

        impl FoxVariables {
            /// Returns the string representation of the `FoxVariables` enum variant.
            ///
            /// This string matches the key used by the FoxESS cloud API.
            ///
            /// # Returns
            /// * `&'static str` - The string representation of the variable.
            #[inline]
            pub const fn as_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $key,
                    )+
                }
            }
        }

        impl FromStr for FoxVariables {
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

define_fox_variables! {
    [AmbientTemperation, "ambientTemperation", "AmbientTemperature, unit: ℃"],
    [BatChargePower, "batChargePower", "Charge Power, unit: kW"],
    [BatCurrent, "batCurrent", "BatCurrent, unit: A"],
    [BatDischargePower, "batDischargePower", "Discharge Power, unit: kW"],
    [BatStatus, "batStatus", "Battery Status"],
    [BatStatusV2, "batStatusV2", "Battery Status Name"],
    [BatTemperature, "batTemperature", "batTemperature, unit: ℃"],
    [BatVolt, "batVolt", "BatVolt, unit: V"],
    [BoostTemperation, "boostTemperation", "BoostTemperature, unit: ℃"],
    [ChargeEnergyToTal, "chargeEnergyToTal", "Total charge energy, unit: kWh"],
    [ChargeTemperature, "chargeTemperature", "ChargeTemperature, unit: ℃"],
    [CurrentFault, "currentFault", "The current error code is reported"],
    [CurrentFaultCount, "currentFaultCount", "The number of errors"],
    [DischargeEnergyToTal, "dischargeEnergyToTal", "Total discharge energy, unit: kWh"],
    [DspTemperature, "dspTemperature", "DSPTemperature, unit: ℃"],
    [EnergyThroughput, "energyThroughput", "Battery throughput, unit: kWh"],
    [EpsCurrentR, "epsCurrentR", "EPS-RCurrent, unit: A"],
    [EpsCurrentS, "epsCurrentS", "EPS-SCurrent, unit: A"],
    [EpsCurrentT, "epsCurrentT", "EPS-TCurrent, unit: A"],
    [EpsPower, "epsPower", "EPSPower, unit: kW"],
    [EpsPowerR, "epsPowerR", "EPS-RPower, unit: kW"],
    [EpsPowerS, "epsPowerS", "EPS-SPower, unit: kW"],
    [EpsPowerT, "epsPowerT", "EPS-TPower, unit: kW"],
    [EpsVoltR, "epsVoltR", "EPS-RVolt, unit: V"],
    [EpsVoltS, "epsVoltS", "EPS-SVolt, unit: V"],
    [EpsVoltT, "epsVoltT", "EPS-TVolt, unit: V"],
    [Feedin, "feedin", "The total energy of the feeder, unit: kWh"],
    [Feedin2, "feedin2", "Total feed network electricity consumption for Meter 2, unit: kWh"],
    [FeedinPower, "feedinPower", "Feed-in Power, unit: kW"],
    [Generation, "generation", "Cumulative power generation, unit: kWh"],
    [GenerationPower, "generationPower", "Output Power, unit: kW"],
    [GridConsumption, "gridConsumption", "Total grid electricity consumption, unit: kWh"],
    [GridConsumption2, "gridConsumption2", "Total electricity consumption of Meter 2, unit: kWh"],
    [GridConsumptionPower, "gridConsumptionPower", "GridConsumption Power, unit: kW"],
    [InvBatCurrent, "invBatCurrent", "InvBatCurrent, unit: A"],
    [InvBatPower, "invBatPower", "invBatPower, unit: kW"],
    [InvBatVolt, "invBatVolt", "InvBatVolt, unit: V"],
    [InvTemperation, "invTemperation", "InvTemperation, unit: ℃"],
    [Loads, "loads", "Load power consumption, unit: kWh"],
    [LoadsPower, "loadsPower", "Load Power, unit: kW"],
    [LoadsPowerR, "loadsPowerR", "LoadsRPower, unit: kW"],
    [LoadsPowerS, "loadsPowerS", "LoadsSPower, unit: kW"],
    [LoadsPowerT, "loadsPowerT", "LoadsTPower, unit: kW"],
    [MaxChargeCurrent, "maxChargeCurrent", "Maximum charge current, unit: A"],
    [MaxDischargeCurrent, "maxDischargeCurrent", "Maximum discharge current, unit: A"],
    [MeterPower, "meterPower", "MeterPower, unit: kW"],
    [MeterPower2, "meterPower2", "Meter2Power, unit: kW"],
    [MeterPowerR, "meterPowerR", "MeterRPower, unit: kW"],
    [MeterPowerS, "meterPowerS", "MeterSPower, unit: kW"],
    [MeterPowerT, "meterPowerT", "MeterTPower, unit: kW"],
    [PowerFactor, "PowerFactor", "PowerFactor"],
    [Pv10Current, "pv10Current", "PV10Current, unit: A"],
    [Pv10Power, "pv10Power", "PV10Power, unit: kW"],
    [Pv10Volt, "pv10Volt", "PV10Volt, unit: V"],
    [Pv11Current, "pv11Current", "PV11Current, unit: A"],
    [Pv11Power, "pv11Power", "PV11Power, unit: kW"],
    [Pv11Volt, "pv11Volt", "PV11Volt, unit: V"],
    [Pv12Current, "pv12Current", "PV12Current, unit: A"],
    [Pv12Power, "pv12Power", "PV12Power, unit: kW"],
    [Pv12Volt, "pv12Volt", "PV12Volt, unit: V"],
    [Pv13Current, "pv13Current", "PV13Current, unit: A"],
    [Pv13Power, "pv13Power", "PV13Power, unit: kW"],
    [Pv13Volt, "pv13Volt", "PV13Volt, unit: V"],
    [Pv14Current, "pv14Current", "PV14Current, unit: A"],
    [Pv14Power, "pv14Power", "PV14Power, unit: kW"],
    [Pv14Volt, "pv14Volt", "PV14Volt, unit: V"],
    [Pv15Current, "pv15Current", "PV15Current, unit: A"],
    [Pv15Power, "pv15Power", "PV15Power, unit: kW"],
    [Pv15Volt, "pv15Volt", "PV15Volt, unit: V"],
    [Pv16Current, "pv16Current", "PV16Current, unit: A"],
    [Pv16Power, "pv16Power", "PV16Power, unit: kW"],
    [Pv16Volt, "pv16Volt", "PV16Volt, unit: V"],
    [Pv17Current, "pv17Current", "PV17Current, unit: A"],
    [Pv17Power, "pv17Power", "PV17Power, unit: kW"],
    [Pv17Volt, "pv17Volt", "PV17Volt, unit: V"],
    [Pv18Current, "pv18Current", "PV18Current, unit: A"],
    [Pv18Power, "pv18Power", "PV18Power, unit: kW"],
    [Pv18Volt, "pv18Volt", "PV18Volt, unit: V"],
    [Pv19Current, "pv19Current", "PV19Current, unit: A"],
    [Pv19Power, "pv19Power", "PV19Power, unit: kW"],
    [Pv19Volt, "pv19Volt", "PV19Volt, unit: V"],
    [Pv1Current, "pv1Current", "PV1Current, unit: A"],
    [Pv1Power, "pv1Power", "PV1Power, unit: kW"],
    [Pv1Volt, "pv1Volt", "PV1Volt, unit: V"],
    [Pv20Current, "pv20Current", "PV20Current, unit: A"],
    [Pv20Power, "pv20Power", "PV20Power, unit: kW"],
    [Pv20Volt, "pv20Volt", "PV20Volt, unit: V"],
    [Pv21Current, "pv21Current", "PV21Current, unit: A"],
    [Pv21Power, "pv21Power", "PV21Power, unit: kW"],
    [Pv21Volt, "pv21Volt", "PV21Volt, unit: V"],
    [Pv22Current, "pv22Current", "PV22Current, unit: A"],
    [Pv22Power, "pv22Power", "PV22Power, unit: kW"],
    [Pv22Volt, "pv22Volt", "PV22Volt, unit: V"],
    [Pv23Current, "pv23Current", "PV23Current, unit: A"],
    [Pv23Power, "pv23Power", "PV23Power, unit: kW"],
    [Pv23Volt, "pv23Volt", "PV23Volt, unit: V"],
    [Pv24Current, "pv24Current", "PV24Current, unit: A"],
    [Pv24Power, "pv24Power", "PV24Power, unit: kW"],
    [Pv24Volt, "pv24Volt", "PV24Volt, unit: V"],
    [Pv2Current, "pv2Current", "PV2Current, unit: A"],
    [Pv2Power, "pv2Power", "PV2Power, unit: kW"],
    [Pv2Volt, "pv2Volt", "PV2Volt, unit: V"],
    [Pv3Current, "pv3Current", "PV3Current, unit: A"],
    [Pv3Power, "pv3Power", "PV3Power, unit: kW"],
    [Pv3Volt, "pv3Volt", "PV3Volt, unit: V"],
    [Pv4Current, "pv4Current", "PV4Current, unit: A"],
    [Pv4Power, "pv4Power", "PV4Power, unit: kW"],
    [Pv4Volt, "pv4Volt", "PV4Volt, unit: V"],
    [Pv5Current, "pv5Current", "PV5Current, unit: A"],
    [Pv5Power, "pv5Power", "PV5Power, unit: kW"],
    [Pv5Volt, "pv5Volt", "PV5Volt, unit: V"],
    [Pv6Current, "pv6Current", "PV6Current, unit: A"],
    [Pv6Power, "pv6Power", "PV6Power, unit: kW"],
    [Pv6Volt, "pv6Volt", "PV6Volt, unit: V"],
    [Pv7Current, "pv7Current", "PV7Current, unit: A"],
    [Pv7Power, "pv7Power", "PV7Power, unit: kW"],
    [Pv7Volt, "pv7Volt", "PV7Volt, unit: V"],
    [Pv8Current, "pv8Current", "PV8Current, unit: A"],
    [Pv8Power, "pv8Power", "PV8Power, unit: kW"],
    [Pv8Volt, "pv8Volt", "PV8Volt, unit: V"],
    [Pv9Current, "pv9Current", "PV9Current, unit: A"],
    [Pv9Power, "pv9Power", "PV9Power, unit: kW"],
    [Pv9Volt, "pv9Volt", "PV9Volt, unit: V"],
    [PVEnergyTotal, "PVEnergyTotal", "Photovoltaic power generation, unit: kWh"],
    [PvPower, "pvPower", "PVPower, unit: kW"],
    [RCurrent, "RCurrent", "RCurrent, unit: A"],
    [ReactivePower, "ReactivePower", "ReactivePower, unit: kVar"],
    [ResidualEnergy, "ResidualEnergy", "Battery Residual Energy, unit: 0.01kWh"],
    [RFreq, "RFreq", "RFreq, unit: Hz"],
    [RPower, "RPower", "RPower, unit: kW"],
    [RunningState, "runningState", "Running State"],
    [RVolt, "RVolt", "RVolt, unit: V"],
    [SCurrent, "SCurrent", "SCurrent, unit: A"],
    [SFreq, "SFreq", "SFreq, unit: Hz"],
    [SoC, "SoC", "SoC, unit: %"],
    [SOH, "SOH", "SOH, unit: %"],
    [SPower, "SPower", "SPower, unit: kW"],
    [SVolt, "SVolt", "SVolt, unit: V"],
    [TCurrent, "TCurrent", "TCurrent, unit: A"],
    [TFreq, "TFreq", "TFreq, unit: Hz"],
    [TodayYield, "todayYield", "Today’s power generation, unit: kW"],
    [TPower, "TPower", "TPower, unit: kW"],
    [TVolt, "TVolt", "TVolt, unit: V"],
}

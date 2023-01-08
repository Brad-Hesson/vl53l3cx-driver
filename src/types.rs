pub use crate::bindings::{
    VL53LX_AdditionalData_t, VL53LX_DeviceInfo_t, VL53LX_MultiRangingData_t,
    VL53LX_TargetRangeData_t, VL53LX_Version_t,
};
use ::num_enum::{IntoPrimitive, TryFromPrimitive};
use ::static_assertions::{assert_eq_align, assert_eq_size};
use heapless::Vec;

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DistanceMode {
    Short = 1,
    Medium = 2,
    Long = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum Vl53lxError {
    CalibrationWarning = -1,
    MinClipped = -2,
    Undefined = -3,
    InvalidParams = -4,
    NotSupported = -5,
    RangeError = -6,
    TimeOut = -7,
    ModeNotSupported = -8,
    BufferTooSmall = -9,
    CommsBufferTooSmall = -10,
    GpioNotExisting = -11,
    GpioFunctionalityNotSupported = -12,
    ControlInterface = -13,
    InvalidCommand = -14,
    DivisionByZero = -15,
    RefSpadInit = -16,
    GphSyncCheckFail = -17,
    StreamCountCheckFail = -18,
    GphIdCheckFail = -19,
    ZoneStreamCountCheckFail = -20,
    ZoneGphIdCheckFail = -21,
    XtalkExtractionNoSampleFail = -22,
    XtalkExtractionSigmaLimitFail = -23,
    OffsetCalNoSampleFail = -24,
    OffsetCalNoSpadsEnabledFail = -25,
    ZoneCalNoSampleFail = -26,
    TuningParmKeyMismatch = -27,
    WarningRefSpadCharNotEnoughSpads = -28,
    WarningRefSpadCharRateTooHigh = -29,
    WarningRefSpadCharRateTooLow = -30,
    WarningOffsetCalMissingSamples = -31,
    WarningOffsetCalSigmaTooHigh = -32,
    WarningOffsetCalRateTooHigh = -33,
    WarningOffsetCalSpadCountTooLow = -34,
    WarningZoneCalMissingSamples = -35,
    WarningZoneCalSigmaTooHigh = -36,
    WarningZoneCalRateTooHigh = -37,
    WarningXtalkMissingSamples = -38,
    WarningXtalkNoSamplesForGradient = -39,
    WarningXtalkSigmaLimitForGradient = -40,
    NotImplemented = -41,
    PlatformSpecificStart = -60,
}
assert_eq_size!(Result<(), Vl53lxError>, i8);
assert_eq_align!(Result<(), Vl53lxError>, i8);
#[test]
fn vl53lxerror_ok_is_0i8() {
    use ::core::mem::transmute;
    assert_eq!(Ok(()) as Result<(), Vl53lxError>, unsafe { transmute(0i8) });
}

#[derive(Debug)]
pub struct MultiRangingData {
    pub stream_count: u8,
    pub xtalk_changed: bool,
    pub effective_spad_rtn: f32,
    pub range_data: Vec<RangeData, 4>,
}
impl From<VL53LX_MultiRangingData_t> for MultiRangingData {
    fn from(value: VL53LX_MultiRangingData_t) -> Self {
        let mut range_data = Vec::new();
        for i in 0..value.NumberOfObjectsFound {
            unsafe { range_data.push_unchecked(value.RangeData[i as usize].into()) };
        }
        Self {
            stream_count: value.StreamCount,
            xtalk_changed: value.HasXtalkValueChanged == 1,
            effective_spad_rtn: value.EffectiveSpadRtnCount as f32 / 256.,
            range_data,
        }
    }
}

#[derive(Debug)]
pub struct RangeData {
    pub range_min_mm: i16,
    pub range_mm: i16,
    pub range_max_mm: i16,
    pub sigma_mm: f32,
    pub signal_rate_rtn_mega_cps: f32,
    pub ambient_rate_rtn_mega_cps: f32,
    pub range_status: RangeStatus,
    pub extended_range: bool,
}
impl From<VL53LX_TargetRangeData_t> for RangeData {
    fn from(value: VL53LX_TargetRangeData_t) -> Self {
        Self {
            range_min_mm: value.RangeMinMilliMeter,
            range_mm: value.RangeMilliMeter,
            range_max_mm: value.RangeMaxMilliMeter,
            sigma_mm: value.SigmaMilliMeter as f32 / 65536.,
            signal_rate_rtn_mega_cps: value.SignalRateRtnMegaCps as f32 / 65536.,
            ambient_rate_rtn_mega_cps: value.AmbientRateRtnMegaCps as f32 / 65536.,
            range_status: value.RangeStatus.try_into().unwrap(),
            extended_range: value.ExtendedRange == 1,
        }
    }
}

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum RangeStatus {
    RangeValid = 0,
    SigmaFail = 1,
    SignalFail = 2,
    RangeValidMinRangeClipped = 3,
    OutofboundsFail = 4,
    HardwareFail = 5,
    RangeValidNoWrapCheckFail = 6,
    WrapTargetFail = 7,
    ProcessingFail = 8,
    XtalkSignalFail = 9,
    SyncronisationInt = 10,
    RangeValidMergedPulse = 11,
    TargetPresentLackOfSignal = 12,
    MinRangeFail = 13,
    RangeInvalid = 14,
    None = 255,
}

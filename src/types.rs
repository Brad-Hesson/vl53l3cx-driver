pub use crate::bindings::{
    VL53LX_AdditionalData_t, VL53LX_DeviceInfo_t, VL53LX_MultiRangingData_t,
    VL53LX_TargetRangeData_t, VL53LX_Version_t,
};
use ::num_enum::{IntoPrimitive, TryFromPrimitive};
use ::static_assertions::{assert_eq_align, assert_eq_size};

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

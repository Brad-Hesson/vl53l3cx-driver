use core::fmt::Display;

use crate::bindings::{
    VL53LX_DeviceInfo_t, VL53LX_MultiRangingData_t, VL53LX_TargetRangeData_t, VL53LX_UserRoi_t,
    VL53LX_Version_t,
};
use ::num_enum::{IntoPrimitive, TryFromPrimitive};
use ::static_assertions::{assert_eq_align, assert_eq_size};
use heapless::Vec;
use measurements::Length;
use serde::{Deserialize, Serialize};

/// Defines all possible Distance modes for the device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum DistanceMode {
    Short = 1,
    Medium = 2,
    Long = 3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum Error {
    /// Warning invalid calibration data may be in used VL53LX_InitData() VL53LX_GetOffsetCalibrationData VL53LX_SetOffsetCalibrationData
    CalibrationWarning = -1,
    /// Warning parameter passed was clipped to min before to be applied
    MinClipped = -2,
    /// Unqualified error
    Undefined = -3,
    /// Parameter passed is invalid or out of range
    InvalidParams = -4,
    /// Function is not supported in current mode or configuration
    NotSupported = -5,
    /// Device report a ranging error interrupt status
    RangeError = -6,
    /// Aborted due to time out
    TimeOut = -7,
    /// Asked mode is not supported by the device
    ModeNotSupported = -8,
    BufferTooSmall = -9,
    /// Supplied buffer is larger than I2C supports
    CommsBufferTooSmall = -10,
    /// User tried to setup a non-existing GPIO pin
    GpioNotExisting = -11,
    /// Unsupported GPIO functionality
    GpioFunctionalityNotSupported = -12,
    /// Error reported from IO functions
    ControlInterface = -13,
    /// The command is not allowed in the current device state (power down)
    InvalidCommand = -14,
    /// In the function a division by zero occurs
    DivisionByZero = -15,
    /// Error during reference SPAD initialization
    RefSpadInit = -16,
    /// GPH sync interrupt check fail - API out of sync with device
    GphSyncCheckFail = -17,
    /// Stream count check fail - API out of sync with device
    StreamCountCheckFail = -18,
    /// GPH ID check fail - API out of sync with device
    GphIdCheckFail = -19,
    /// Zone dynamic config stream count check failed - API out of sync
    ZoneStreamCountCheckFail = -20,
    /// Zone dynamic config GPH ID check failed - API out of sync
    ZoneGphIdCheckFail = -21,
    /// Thrown when run_xtalk_extraction fn has 0 successful samples when using the full array to sample the xtalk. In this case there is not enough information to generate new Xtalk parm info. The function will exit and leave the current xtalk parameters unaltered
    XtalkExtractionNoSampleFail = -22,
    /// Thrown when run_xtalk_extraction fn has found that the avg sigma estimate of the full array xtalk sample is > than the maximal limit allowed. In this case the xtalk sample is too noisy for measurement. The function will exit and leave the current xtalk parameters unaltered.
    XtalkExtractionSigmaLimitFail = -23,
    /// Thrown if there one of stages has no valid offset calibration samples. A fatal error calibration not valid
    OffsetCalNoSampleFail = -24,
    /// Thrown if there one of stages has zero effective SPADS Traps the case when MM1 SPADs is zero. A fatal error calibration not valid
    OffsetCalNoSpadsEnabledFail = -25,
    /// Thrown if then some of the zones have no valid samples A fatal error calibration not valid
    ZoneCalNoSampleFail = -26,
    /// Thrown if the tuning file key table version does not match with expected value. The driver expects the key table version to match the compiled default version number in the define #VL53LX_TUNINGPARM_KEY_TABLE_VERSION_DEFAULT
    TuningParmKeyMismatch = -27,
    /// Thrown if there are less than 5 good SPADs are available.
    WarningRefSpadCharNotEnoughSpads = -28,
    /// Thrown if the final reference rate is greater than the upper reference rate limit - default is 40 Mcps. Implies a minimum Q3 (x10) SPAD (5) selected
    WarningRefSpadCharRateTooHigh = -29,
    /// Thrown if the final reference rate is less than the lower reference rate limit - default is 10 Mcps. Implies maximum Q1 (x1) SPADs selected
    WarningRefSpadCharRateTooLow = -30,
    /// Thrown if there is less than the requested number of valid samples.
    WarningOffsetCalMissingSamples = -31,
    /// Thrown if the offset calibration range sigma estimate is greater than 8.0 mm. This is the recommended min value to yield a stable offset measurement
    WarningOffsetCalSigmaTooHigh = -32,
    /// Thrown when VL53LX_run_offset_calibration() peak rate is greater than that 50.0Mcps. This is the recommended max rate to avoid pile-up influencing the offset measurement
    WarningOffsetCalRateTooHigh = -33,
    /// Thrown when VL53LX_run_offset_calibration() when one of stages range has less that 5.0 effective SPADS. This is the recommended min value to yield a stable offset
    WarningOffsetCalSpadCountTooLow = -34,
    /// Thrown if one of more of the zones have less than the requested number of valid samples
    WarningZoneCalMissingSamples = -35,
    /// Thrown if one or more zones have sigma estimate value greater than 8.0 mm. This is the recommended min value to yield a stable offset measurement
    WarningZoneCalSigmaTooHigh = -36,
    /// Thrown if one of more zones have peak rate higher than that 50.0Mcps. This is the recommended max rate to avoid pile-up influencing the offset measurement
    WarningZoneCalRateTooHigh = -37,
    /// Thrown to notify that some of the xtalk samples did not yield valid ranging pulse data while attempting to measure the xtalk signal in vl53lx_run_xtalk_extract(). This can signify any of the zones are missing samples, for further debug information the xtalk_results struct should be referred to. This warning is for notification only, xtalk pulse and shape have still been generated
    WarningXtalkMissingSamples = -38,
    /// Thrown to notify that some of the xtalk samples used for gradient generation did not yield valid ranging pulse data while attempting to measure the xtalk signal in vl53lx_run_xtalk_extract(). This can signify that any one of the zones 0-3 yielded no successful samples. xtalk_results struct should be referred to for further debug info. This warning is for notification only, the xtalk pulse and shape have still been generated.
    WarningXtalkNoSamplesForGradient = -39,
    /// Thrown to notify that some of the xtalk samples used for gradient generation did not pass the sigma limit check while attempting to measure the xtalk signal in vl53lx_run_xtalk_extract(). This can signify that any one of the zones 0-3 yielded an avg sigma_mm value > the limit. The xtalk_results struct should be referred to for further debug info. This warning is for notification only, the xtalk pulse and shape have still been generated.
    WarningXtalkSigmaLimitForGradient = -40,
    /// Tells requested functionality has not been implemented yet or not compatible with the device
    NotImplemented = -41,
    /// Tells the starting code for platform
    PlatformSpecificStart = -60,
}
assert_eq_size!(Result<(), Error>, i8);
assert_eq_align!(Result<(), Error>, i8);
#[test]
fn vl53lxerror_ok_is_0i8() {
    use ::core::mem::transmute;
    assert_eq!(Ok(()) as Result<(), Error>, unsafe { transmute(0i8) });
}

/// Structure for storing the set of range results.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiRangingData {
    /// 8-bit Stream Count.
    pub stream_count: u8,
    /// Set to true if a new Xtalk value has been computed whilst smudge correction mode enable by with VL53LX_SmudgeCorrectionEnable() function is either VL53LX_SMUDGE_CORRECTION_CONTINUOUS or VL53LX_SMUDGE_CORRECTION_SINGLE.
    pub xtalk_changed: bool,
    /// Return the effective SPAD count for the return signal.
    pub effective_spad_rtn: f32,
    /// Range data each target distance
    pub range_data: Vec<TargetRangeData, 4>,
}
impl From<VL53LX_MultiRangingData_t> for MultiRangingData {
    fn from(value: VL53LX_MultiRangingData_t) -> Self {
        let mut range_data = Vec::new();
        for i in 0..value.NumberOfObjectsFound {
            unsafe { range_data.push_unchecked(value.RangeData[i as usize].into()) };
        }
        Self {
            stream_count: value.StreamCount,
            xtalk_changed: value.HasXtalkValueChanged != 0,
            effective_spad_rtn: value.EffectiveSpadRtnCount as f32 / 256.,
            range_data,
        }
    }
}

/// One Range measurement data for each target.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TargetRangeData {
    /// Tells what is the minimum detection distance of the object in current setup and environment conditions (Filled when applicable)
    pub range_min: Length,
    /// Range distance in millimeter. This should be between RangeMinMilliMeter and RangeMaxMilliMeter
    pub range: Length,
    /// Tells what is the maximum detection distance of the object in current setup and environment conditions (Filled when applicable)
    pub range_max: Length,
    /// Return the Sigma value in millimeter
    pub range_sigma: Length,
    /// Return signal rate (MCPS). This is effectively a measure of target reflectance.
    pub signal_rate_rtn_mega_cps: f32,
    /// Return ambient rate (MCPS). This is effectively a measure of the ambient light.
    pub ambient_rate_rtn_mega_cps: f32,
    /// Range Status for the current measurement. This is device dependent.
    pub range_status: RangeStatus,
    /// Extended range flag for the current measurement. True means timings A&B are combined to increase the maximum distance range.
    pub extended_range: bool,
}
impl From<VL53LX_TargetRangeData_t> for TargetRangeData {
    fn from(value: VL53LX_TargetRangeData_t) -> Self {
        Self {
            range_min: Length::from_millimeters(value.RangeMinMilliMeter as f64),
            range: Length::from_millimeters(value.RangeMilliMeter as f64),
            range_max: Length::from_millimeters(value.RangeMaxMilliMeter as f64),
            range_sigma: Length::from_millimeters(value.SigmaMilliMeter as f64 / 65536.),
            signal_rate_rtn_mega_cps: value.SignalRateRtnMegaCps as f32 / 65536.,
            ambient_rate_rtn_mega_cps: value.AmbientRateRtnMegaCps as f32 / 65536.,
            range_status: value.RangeStatus.try_into().unwrap(),
            extended_range: value.ExtendedRange != 0,
        }
    }
}

/// The Range Status is the quality of the latest ranging.
#[derive(Debug, Clone, Copy, TryFromPrimitive, Serialize, Deserialize)]
#[repr(u8)]
pub enum RangeStatus {
    /// The Range is valid.
    RangeValid = 0,
    /// Sigma Fail.
    SigmaFail = 1,
    /// Signal fail.
    SignalFail = 2,
    /// Target is below minimum detection threshold.
    RangeValidMinRangeClipped = 3,
    /// Phase out of valid limits - different to a wrap exit.
    OutofboundsFail = 4,
    /// Hardware fail.
    HardwareFail = 5,
    /// The Range is valid but the wraparound check has not been done.
    RangeValidNoWrapCheckFail = 6,
    /// Wrapped target - no matching phase in other VCSEL period timing.
    WrapTargetFail = 7,
    /// Internal algo underflow or overflow in lite ranging.
    ProcessingFail = 8,
    /// Specific to lite ranging.
    XtalkSignalFail = 9,
    /// 1st interrupt when starting ranging in back to back mode. Ignore data.
    SyncronisationInt = 10,
    /// All Range ok but object is result of multiple pulses merging together. Used by RQL for merged pulse detection.
    RangeValidMergedPulse = 11,
    /// Used by RQL as different to phase fail.
    TargetPresentLackOfSignal = 12,
    /// Unexpected error in SPAD Array.
    MinRangeFail = 13,
    /// lld returned valid range but negative value.
    RangeInvalid = 14,
    /// No Update.
    None = 255,
}

#[derive(Debug, Clone, Copy)]
pub struct DeviceInfo {
    pub product_type: u8,
    pub product_revision_major: u8,
    pub product_revision_minor: u8,
}
impl From<VL53LX_DeviceInfo_t> for DeviceInfo {
    fn from(value: VL53LX_DeviceInfo_t) -> Self {
        Self {
            product_type: value.ProductType,
            product_revision_major: value.ProductRevisionMajor,
            product_revision_minor: value.ProductRevisionMinor,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Version {
    pub revision: u32,
    pub major: u8,
    pub minor: u8,
    pub build: u8,
}
impl From<VL53LX_Version_t> for Version {
    fn from(value: VL53LX_Version_t) -> Self {
        Self {
            revision: value.revision,
            major: value.major,
            minor: value.minor,
            build: value.build,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Roi {
    pub top_left_x: u8,
    pub top_left_y: u8,
    pub bottom_right_x: u8,
    pub bottom_right_y: u8,
}
impl Roi {
    pub fn new(tl_x: u8, tl_y: u8, br_x: u8, br_y: u8) -> Result<Self, RoiError> {
        if [tl_x, tl_y, br_x, br_y].iter().any(|c| *c > 15) {
            return Err(RoiError::CoordsOutOfBounds);
        }
        if tl_y < br_y || br_x < tl_x {
            return Err(RoiError::WrongCoordOrder);
        }
        if tl_y - br_y < 3 || br_x - tl_x < 3 {
            return Err(RoiError::RoiTooSmall);
        }
        Ok(Self {
            top_left_x: tl_x,
            top_left_y: tl_y,
            bottom_right_x: br_x,
            bottom_right_y: br_y,
        })
    }
    pub fn centered(size: u8) -> Result<Self, RoiError> {
        if size % 2 != 0 {
            return Err(RoiError::SizeNotEven);
        }
        let half = size / 2;
        let low = 8 - half;
        let high = 7 + half;
        Self::new(low, high, high, low)
    }
    pub fn get_size(&self) -> (u8, u8) {
        (
            self.bottom_right_x - self.top_left_x + 1,
            self.top_left_y - self.bottom_right_y + 1,
        )
    }
}
impl From<Roi> for VL53LX_UserRoi_t {
    fn from(value: Roi) -> Self {
        Self {
            TopLeftX: value.top_left_x,
            TopLeftY: value.top_left_y,
            BotRightX: value.bottom_right_x,
            BotRightY: value.bottom_right_y,
        }
    }
}
impl From<VL53LX_UserRoi_t> for Roi {
    fn from(value: VL53LX_UserRoi_t) -> Self {
        Self {
            top_left_x: value.TopLeftX,
            top_left_y: value.TopLeftY,
            bottom_right_x: value.BotRightX,
            bottom_right_y: value.BotRightY,
        }
    }
}
impl Display for Roi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for y in 0..=15 {
            for x in 0..=15 {
                if x >= self.top_left_x
                    && x <= self.bottom_right_x
                    && y >= self.bottom_right_y
                    && y <= self.top_left_y
                {
                    f.write_str("# ")?;
                } else {
                    f.write_str(". ")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RoiError {
    CoordsOutOfBounds,
    RoiTooSmall,
    WrongCoordOrder,
    SizeNotEven,
}

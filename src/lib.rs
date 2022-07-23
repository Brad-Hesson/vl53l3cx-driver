#![no_std]
#![feature(c_variadic, type_changing_struct_update)]

mod bindings;
mod defaults;
mod wrapper;

pub use crate::bindings::{
    VL53LX_AdditionalData_t, VL53LX_DeviceInfo_t, VL53LX_MultiRangingData_t,
    VL53LX_TargetRangeData_t, VL53LX_Version_t,
};
use crate::wrapper::vl53lx_platform_user_data::VL53LX_Dev_t;
use ::core::{convert::Infallible, marker::PhantomData, ptr};
use ::embedded_hal::{
    blocking::{
        delay::{DelayMs, DelayUs},
        i2c::{Read, Write},
    },
    digital::v2::OutputPin,
};

pub struct VL53L3CX<'a, STATE, I2C, XSHUT, DELAY> {
    xshut_pin: XSHUT,
    dev_t: VL53LX_Dev_t<'a>,
    i2c: PhantomData<I2C>,
    delay: PhantomData<DELAY>,
    _state: STATE,
}
macro_rules! result {
    ($code:expr) => {
        match unsafe { $code } {
            0 => Ok(()),
            code => Err(unsafe { ::core::mem::transmute(code) }),
        }
    };
}
pub struct Enabled;
pub struct Disabled;
// -------------------------- Disabled ----------------------------
impl<'a, I2C, XSHUT, DELAY> VL53L3CX<'a, Disabled, I2C, XSHUT, DELAY>
where
    I2C: Read + Write + 'a,
    XSHUT: OutputPin<Error = Infallible>,
    DELAY: DelayUs<u32> + DelayMs<u32> + 'a,
{
    pub fn new(i2c_address: u8, xshut_pin: XSHUT) -> Self {
        Self {
            dev_t: VL53LX_Dev_t {
                Data: Default::default(),
                i2c_p: ptr::null_mut::<I2C>(),
                delay_p: ptr::null_mut::<DELAY>(),
                i2c_address,
            },
            xshut_pin,
            i2c: PhantomData,
            delay: PhantomData,
            _state: Disabled,
        }
    }
    pub fn into_enabled(
        mut self,
        i2c: &mut I2C,
        delay: &mut DELAY,
    ) -> Result<VL53L3CX<'a, Enabled, I2C, XSHUT, DELAY>, (Self, Vl53lxError)> {
        self.xshut_pin
            .set_high()
            .expect("setting pin state is infallible");
        match self.with_i2c_and_delay(i2c, delay, |pdev| {
            result!(bindings::VL53LX_WaitDeviceBooted(pdev))?;
            result!(bindings::VL53LX_DataInit(pdev))
        }) {
            Ok(_) => Ok(VL53L3CX {
                _state: Enabled,
                ..self
            }),
            Err(e) => {
                self.xshut_pin
                    .set_low()
                    .expect("setting pin state is infallible");
                Err((self, e))
            }
        }
    }
}

// -------------------------- Enabled ----------------------------
impl<'a, I2C, XSHUT, DELAY> VL53L3CX<'a, Enabled, I2C, XSHUT, DELAY>
where
    I2C: Write + Read + 'a,
    XSHUT: OutputPin<Error = Infallible>,
    DELAY: DelayUs<u32> + DelayMs<u32> + 'a,
{
    pub fn into_disabled(mut self) -> VL53L3CX<'a, Disabled, I2C, XSHUT, DELAY> {
        self.xshut_pin
            .set_low()
            .expect("setting pin state is infallible");
        VL53L3CX {
            _state: Disabled,
            ..self
        }
    }
    pub fn read_byte(&mut self, i2c: &mut I2C, index: u16) -> Result<u8, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut data: u8 = 0;
            result!(bindings::VL53LX_RdByte(pdev, index, &mut data))?;
            Ok(data)
        })
    }
    pub fn get_product_revision(&mut self, i2c: &mut I2C) -> Result<(u8, u8), Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut major = 0u8;
            let mut minor = 0u8;
            result!(bindings::VL53LX_GetProductRevision(
                pdev, &mut major, &mut minor
            ))?;
            Ok((major, minor))
        })
    }
    pub fn get_device_info(&mut self, i2c: &mut I2C) -> Result<VL53LX_DeviceInfo_t, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut dev_info = VL53LX_DeviceInfo_t::default();
            result!(bindings::VL53LX_GetDeviceInfo(pdev, &mut dev_info))?;
            Ok(dev_info)
        })
    }
    pub fn get_uid(&mut self, i2c: &mut I2C, delay: &mut DELAY) -> Result<u64, Vl53lxError> {
        self.with_i2c_and_delay(i2c, delay, |pdev| {
            let mut id = 0u64;
            result!(bindings::VL53LX_GetUID(pdev, &mut id))?;
            Ok(id)
        })
    }
    pub fn set_device_address(&mut self, i2c: &mut I2C, address: u8) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            result!(bindings::VL53LX_SetDeviceAddress(pdev, address))
        })
    }
}
// TODO: Correctly arrange functions by whether they require enabled or disabled or either one
// TODO: Write tests using a dummy i2c and delay device to verify that the required pointers and only the required pointers are being loaded
// -------------------------- Any ----------------------------
impl<'a, STATE, I2C, XSHUT, DELAY> VL53L3CX<'a, STATE, I2C, XSHUT, DELAY>
where
    I2C: Read + Write + 'a,
    XSHUT: OutputPin<Error = Infallible>,
    DELAY: DelayUs<u32> + DelayMs<u32> + 'a,
{
    pub fn get_version(&mut self, i2c: &mut I2C) -> Result<VL53LX_Version_t, Vl53lxError> {
        self.with_i2c(i2c, |_| {
            let mut version = VL53LX_Version_t::default();
            result!(bindings::VL53LX_GetVersion(&mut version))?;
            Ok(version)
        })
    }
    pub fn set_distance_mode(
        &mut self,
        i2c: &mut I2C,
        mode: DistanceMode,
    ) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            result!(bindings::VL53LX_SetDistanceMode(pdev, mode.into()))
        })
    }
    pub fn get_distance_mode(&mut self, i2c: &mut I2C) -> Result<DistanceMode, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut mode = 0u8;
            result!(bindings::VL53LX_GetDistanceMode(pdev, &mut mode))?;
            Ok(mode.into())
        })
    }

    pub fn set_measurement_timing_budget_ms(
        &mut self,
        i2c: &mut I2C,
        ms: u32,
    ) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            result!(bindings::VL53LX_SetMeasurementTimingBudgetMicroSeconds(
                pdev,
                ms * 1000,
            ))
        })
    }
    pub fn get_measurement_timing_budget_ms(&mut self, i2c: &mut I2C) -> Result<u32, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut ms = 0u32;
            result!(bindings::VL53LX_GetMeasurementTimingBudgetMicroSeconds(
                pdev, &mut ms
            ))?;
            Ok(ms / 1000)
        })
    }
    pub fn start_measurement(&mut self, i2c: &mut I2C) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| result!(bindings::VL53LX_StartMeasurement(pdev)))
    }
    pub fn stop_measurement(&mut self, i2c: &mut I2C) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| result!(bindings::VL53LX_StopMeasurement(pdev)))
    }
    pub fn clear_interrupt_and_start_measurement(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            result!(bindings::VL53LX_ClearInterruptAndStartMeasurement(pdev))
        })
    }
    pub fn get_measurement_data_ready(&mut self, i2c: &mut I2C) -> Result<bool, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut data = 0u8;
            result!(bindings::VL53LX_GetMeasurementDataReady(pdev, &mut data))?;
            Ok(data == 1)
        })
    }
    pub fn wait_measurement_data_ready(
        &mut self,
        i2c: &mut I2C,
        delay: &mut DELAY,
    ) -> Result<(), Vl53lxError> {
        self.with_i2c_and_delay(i2c, delay, |pdev| {
            result!(bindings::VL53LX_WaitMeasurementDataReady(pdev))
        })
    }
    pub fn get_multiranging_data(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<VL53LX_MultiRangingData_t, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut data = VL53LX_MultiRangingData_t::default();
            result!(bindings::VL53LX_GetMultiRangingData(pdev, &mut data))?;
            Ok(data)
        })
    }
    pub fn get_additional_data(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<VL53LX_AdditionalData_t, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut data = VL53LX_AdditionalData_t::default();
            result!(bindings::VL53LX_GetAdditionalData(pdev, &mut data))?;
            Ok(data)
        })
    }
    #[inline]
    fn with_i2c<F, T>(&mut self, i2c: &mut I2C, f: F) -> Result<T, Vl53lxError>
    where
        F: FnOnce(&mut VL53LX_Dev_t) -> Result<T, Vl53lxError>,
    {
        self.dev_t.i2c_p = i2c;
        let res = f(&mut self.dev_t);
        self.dev_t.i2c_p = ptr::null_mut::<I2C>();
        res
    }
    #[inline]
    fn with_i2c_and_delay<F, T>(
        &mut self,
        i2c: &mut I2C,
        delay: &mut DELAY,
        f: F,
    ) -> Result<T, Vl53lxError>
    where
        F: FnOnce(&mut VL53LX_Dev_t) -> Result<T, Vl53lxError>,
    {
        self.dev_t.delay_p = delay;
        let result = self.with_i2c(i2c, f);
        self.dev_t.delay_p = ptr::null_mut::<DELAY>();
        result
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum DistanceMode {
    Short = 1,
    Medium = 2,
    Long = 3,
}
impl From<u8> for DistanceMode {
    fn from(n: u8) -> Self {
        unsafe { ::core::mem::transmute(n) }
    }
}
impl From<DistanceMode> for u8 {
    fn from(mode: DistanceMode) -> Self {
        unsafe { ::core::mem::transmute(mode) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[cfg(test)]
mod Vl53lxErrorTest {
    use super::*;
    #[test]
    fn ok_into_0i8() {
        assert_eq!(Ok(()) as Result<(), Vl53lxError>, unsafe {
            ::core::mem::transmute(0i8)
        });
    }
    #[test]
    fn size_of_result() {
        assert_eq!(::core::mem::size_of::<Result<(), Vl53lxError>>(), 1);
    }
}

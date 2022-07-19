#![no_std]
#![feature(c_variadic)]

mod defaults;
mod driver;
mod wrapper;

use crate::driver::Hardware;
use ::core::{convert::Infallible, ptr};
use ::cty::c_void;
use ::embedded_hal::{
    blocking::{
        delay::DelayUs,
        i2c::{Read, Write},
    },
    digital::v2::OutputPin,
};

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use crate::bindings::{
    VL53LX_AdditionalData_t, VL53LX_DeviceInfo_t, VL53LX_MultiRangingData_t,
    VL53LX_TargetRangeData_t, VL53LX_Version_t,
};
use crate::bindings::{VL53LX_Dev_t, VL53LX_Error};

pub struct Enabled;
pub struct Disabled;
pub struct VL53L3CX<STATE, I2C, XSHUT, DELAY>
where
    I2C: Write + Read,
    XSHUT: OutputPin<Error = Infallible>,
    DELAY: DelayUs<u32>,
{
    dev_t: VL53LX_Dev_t,
    hardware: Hardware<I2C, XSHUT, DELAY>,
    _state: STATE,
}
impl<I2C, XSHUT, DELAY> VL53L3CX<Disabled, I2C, XSHUT, DELAY>
where
    I2C: Write + Read,
    XSHUT: OutputPin<Error = Infallible>,
    DELAY: DelayUs<u32>,
{
    pub fn new(i2c: I2C, i2c_address: u8, xshut_pin: XSHUT) -> Self {
        Self {
            hardware: Hardware::<I2C, XSHUT, DELAY> {
                i2c_address,
                i2c,
                xshut_pin,
                delay_p: ptr::null_mut(),
            },
            dev_t: VL53LX_Dev_t {
                hardware_p: ptr::null_mut(),
                read_f: Some(Hardware::<I2C, XSHUT, DELAY>::read),
                write_f: Some(Hardware::<I2C, XSHUT, DELAY>::write),
                wait_us_f: Some(Hardware::<I2C, XSHUT, DELAY>::wait_us),
                Data: Default::default(),
            },
            _state: Disabled,
        }
    }
    pub fn into_enabled(mut self) -> VL53L3CX<Enabled, I2C, XSHUT, DELAY> {
        self.hardware
            .xshut_pin
            .set_high()
            .expect("setting pin state is infallible");
        VL53L3CX {
            dev_t: self.dev_t,
            hardware: self.hardware,
            _state: Enabled,
        }
    }
}
impl<I2C, XSHUT, DELAY> VL53L3CX<Enabled, I2C, XSHUT, DELAY>
where
    I2C: Write + Read,
    XSHUT: OutputPin<Error = Infallible>,
    DELAY: DelayUs<u32>,
{
    pub fn read_byte(&mut self, index: u16) -> Result<u8, Error> {
        let mut data: u8 = 0;
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_RdByte(pdev, index, &mut data) })?;
        Ok(data)
    }
    pub fn get_product_revision(&mut self) -> Result<(u8, u8), Error> {
        let mut major = 0u8;
        let mut minor = 0u8;
        self.with_pdev(|pdev| unsafe {
            bindings::VL53LX_GetProductRevision(pdev, &mut major, &mut minor)
        })?;
        Ok((major, minor))
    }
    pub fn get_device_info(&mut self) -> Result<VL53LX_DeviceInfo_t, Error> {
        let mut dev_info = VL53LX_DeviceInfo_t::default();
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_GetDeviceInfo(pdev, &mut dev_info) })?;
        Ok(dev_info)
    }
    pub fn get_uid(&mut self, delay: &mut DELAY) -> Result<u64, Error> {
        let mut id = 0u64;
        self.with_delay(delay, |pdev| unsafe {
            bindings::VL53LX_GetUID(pdev, &mut id)
        })?;
        Ok(id)
    }
    pub fn set_device_address(&mut self, address: u8) -> Result<(), Error> {
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_SetDeviceAddress(pdev, address) })?;
        Ok(())
    }
    pub fn data_init(&mut self, delay: &mut DELAY) -> Result<(), Error> {
        self.with_delay(delay, |pdev| unsafe { bindings::VL53LX_DataInit(pdev) })?;
        Ok(())
    }
    pub fn wait_device_booted(&mut self, delay: &mut DELAY) -> Result<(), Error> {
        self.with_delay(delay, |pdev| unsafe {
            bindings::VL53LX_WaitDeviceBooted(pdev)
        })?;
        Ok(())
    }
}
impl<STATE, I2C, XSHUT, DELAY> VL53L3CX<STATE, I2C, XSHUT, DELAY>
where
    I2C: Write + Read,
    XSHUT: OutputPin<Error = Infallible>,
    DELAY: DelayUs<u32>,
{
    pub fn get_version(&mut self) -> Result<VL53LX_Version_t, Error> {
        let mut version = VL53LX_Version_t::default();
        self.with_pdev(|_| unsafe { bindings::VL53LX_GetVersion(&mut version) })?;
        Ok(version)
    }
    pub fn set_distance_mode(&mut self, mode: DistanceMode) -> Result<(), Error> {
        let mode_num = match mode {
            DistanceMode::Short => 1,
            DistanceMode::Medium => 2,
            DistanceMode::Long => 3,
        };
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_SetDistanceMode(pdev, mode_num) })?;
        Ok(())
    }
    pub fn get_distance_mode(&mut self) -> Result<DistanceMode, Error> {
        let mut mode = 0u8;
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_GetDistanceMode(pdev, &mut mode) })?;
        Ok(match mode {
            1 => DistanceMode::Short,
            2 => DistanceMode::Medium,
            3 => DistanceMode::Long,
            _ => unimplemented!(),
        })
    }
    pub fn set_measurement_timing_budget_ms(&mut self, ms: u32) -> Result<(), Error> {
        self.with_pdev(|pdev| unsafe {
            bindings::VL53LX_SetMeasurementTimingBudgetMicroSeconds(pdev, ms * 1000)
        })?;
        Ok(())
    }
    pub fn get_measurement_timing_budget_ms(&mut self) -> Result<u32, Error> {
        let mut ms = 0u32;
        self.with_pdev(|pdev| unsafe {
            bindings::VL53LX_GetMeasurementTimingBudgetMicroSeconds(pdev, &mut ms)
        })?;
        Ok(ms / 1000)
    }
    pub fn start_measurement(&mut self) -> Result<(), Error> {
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_StartMeasurement(pdev) })?;
        Ok(())
    }
    pub fn stop_measurement(&mut self) -> Result<(), Error> {
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_StopMeasurement(pdev) })?;
        Ok(())
    }
    pub fn clear_interrupt_and_start_measurement(&mut self) -> Result<(), Error> {
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_ClearInterruptAndStartMeasurement(pdev) })?;
        Ok(())
    }
    pub fn get_measurement_data_ready(&mut self) -> Result<bool, Error> {
        let mut data = 0u8;
        self.with_pdev(|pdev| unsafe {
            bindings::VL53LX_GetMeasurementDataReady(pdev, &mut data)
        })?;
        Ok(data == 1)
    }
    pub fn wait_measurement_data_ready(&mut self, delay: &mut DELAY) -> Result<(), Error> {
        self.with_delay(delay, |pdev| unsafe {
            bindings::VL53LX_WaitMeasurementDataReady(pdev)
        })?;
        Ok(())
    }
    pub fn get_multiranging_data(&mut self) -> Result<VL53LX_MultiRangingData_t, Error> {
        let mut data = VL53LX_MultiRangingData_t::default();
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_GetMultiRangingData(pdev, &mut data) })?;
        Ok(data)
    }
    pub fn get_additional_data(&mut self) -> Result<VL53LX_AdditionalData_t, Error> {
        let mut data = VL53LX_AdditionalData_t::default();
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_GetAdditionalData(pdev, &mut data) })?;
        Ok(data)
    }
    fn with_pdev<F>(&mut self, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut VL53LX_Dev_t) -> VL53LX_Error,
    {
        self.dev_t.hardware_p = ptr::addr_of_mut!(self.hardware) as *mut c_void;
        let res = match f(&mut self.dev_t) {
            0 => Ok(()),
            status => Err(Error::from(status)),
        };
        self.dev_t.hardware_p = ptr::null_mut();
        res
    }
    fn with_delay<F>(&mut self, delay: &mut DELAY, f: F) -> Result<(), Error>
    where
        F: FnMut(&mut VL53LX_Dev_t) -> VL53LX_Error,
    {
        self.hardware.delay_p = delay;
        let result = self.with_pdev(f);
        self.hardware.delay_p = ptr::null_mut();
        result
    }
}

#[derive(Debug)]
pub enum DistanceMode {
    Short,
    Medium,
    Long,
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    None,
    CalibrationWarning,
    MinClipped,
    Undefined,
    InvalidParams,
    NotSupported,
    RangeError,
    TimeOut,
    ModeNotSupported,
    BufferTooSmall,
    CommsBufferTooSmall,
    GpioNotExisting,
    GpioFunctionalityNotSupported,
    ControlInterface,
    InvalidCommand,
    DivisionByZero,
    RefSpadInit,
    GphSyncCheckFail,
    StreamCountCheckFail,
    GphIdCheckFail,
    ZoneStreamCountCheckFail,
    ZoneGphIdCheckFail,
    XtalkExtractionNoSampleFail,
    XtalkExtractionSigmaLimitFail,
    OffsetCalNoSampleFail,
    OffsetCalNoSpadsEnabledFail,
    ZoneCalNoSampleFail,
    TuningParmKeyMismatch,
    WarningRefSpadCharNotEnoughSpads,
    WarningRefSpadCharRateTooHigh,
    WarningRefSpadCharRateTooLow,
    WarningOffsetCalMissingSamples,
    WarningOffsetCalSigmaTooHigh,
    WarningOffsetCalRateTooHigh,
    WarningOffsetCalSpadCountTooLow,
    WarningZoneCalMissingSamples,
    WarningZoneCalSigmaTooHigh,
    WarningZoneCalRateTooHigh,
    WarningXtalkMissingSamples,
    WarningXtalkNoSamplesForGradient,
    WarningXtalkSigmaLimitForGradient,
    NotImplemented,
    PlatformSpecificStart,
}
impl From<VL53LX_Error> for Error {
    fn from(e: VL53LX_Error) -> Self {
        match e {
            0 => Error::None,
            -1 => Error::CalibrationWarning,
            -2 => Error::MinClipped,
            -3 => Error::Undefined,
            -4 => Error::InvalidParams,
            -5 => Error::NotSupported,
            -6 => Error::RangeError,
            -7 => Error::TimeOut,
            -8 => Error::ModeNotSupported,
            -9 => Error::BufferTooSmall,
            -10 => Error::CommsBufferTooSmall,
            -11 => Error::GpioNotExisting,
            -12 => Error::GpioFunctionalityNotSupported,
            -13 => Error::ControlInterface,
            -14 => Error::InvalidCommand,
            -15 => Error::DivisionByZero,
            -16 => Error::RefSpadInit,
            -17 => Error::GphSyncCheckFail,
            -18 => Error::StreamCountCheckFail,
            -19 => Error::GphIdCheckFail,
            -20 => Error::ZoneStreamCountCheckFail,
            -21 => Error::ZoneGphIdCheckFail,
            -22 => Error::XtalkExtractionNoSampleFail,
            -23 => Error::XtalkExtractionSigmaLimitFail,
            -24 => Error::OffsetCalNoSampleFail,
            -25 => Error::OffsetCalNoSpadsEnabledFail,
            -26 => Error::ZoneCalNoSampleFail,
            -27 => Error::TuningParmKeyMismatch,
            -28 => Error::WarningRefSpadCharNotEnoughSpads,
            -29 => Error::WarningRefSpadCharRateTooHigh,
            -30 => Error::WarningRefSpadCharRateTooLow,
            -31 => Error::WarningOffsetCalMissingSamples,
            -32 => Error::WarningOffsetCalSigmaTooHigh,
            -33 => Error::WarningOffsetCalRateTooHigh,
            -34 => Error::WarningOffsetCalSpadCountTooLow,
            -35 => Error::WarningZoneCalMissingSamples,
            -36 => Error::WarningZoneCalSigmaTooHigh,
            -37 => Error::WarningZoneCalRateTooHigh,
            -38 => Error::WarningXtalkMissingSamples,
            -39 => Error::WarningXtalkNoSamplesForGradient,
            -40 => Error::WarningXtalkSigmaLimitForGradient,
            -41 => Error::NotImplemented,
            -60 => Error::PlatformSpecificStart,
            _ => unimplemented!(),
        }
    }
}

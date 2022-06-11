#![no_std]
#![feature(c_variadic)]

mod defaults;
mod driver;
mod wrapper;

use core::{convert::Infallible, ptr};
use cty::c_void;
use driver::Hardware;
use stm32l4xx_hal::{
    delay::Delay,
    i2c,
    prelude::{
        _embedded_hal_blocking_i2c_Read as Read, _embedded_hal_blocking_i2c_Write as Write, *,
    },
};

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub use bindings::VL53LX_MultiRangingData_t;
pub use bindings::VL53LX_TargetRangeData_t;
use bindings::{VL53LX_Dev_t, VL53LX_Error};

pub struct VL53L3CX<I2C, XSHUT> {
    dev_t: VL53LX_Dev_t,
    hardware: Hardware<I2C, XSHUT>,
}
impl<I2C, XSHUT> VL53L3CX<I2C, XSHUT>
where
    I2C: Write<Error = i2c::Error> + Read<Error = i2c::Error>,
    XSHUT: OutputPin<Error = Infallible>,
{
    pub fn new(i2c: I2C, i2c_address: u8, xshut_pin: XSHUT) -> Self {
        let hardware = Hardware::<I2C, XSHUT> {
            i2c_address,
            i2c,
            xshut_pin,
            delay_p: ptr::null_mut(),
        };
        let mut _self = Self {
            hardware,
            dev_t: VL53LX_Dev_t {
                hardware_p: ptr::null_mut(),
                read_f: Some(Hardware::<I2C, XSHUT>::read),
                write_f: Some(Hardware::<I2C, XSHUT>::write),
                wait_us_f: Some(Hardware::<I2C, XSHUT>::wait_us),
                Data: Default::default(),
            },
        };
        _self.dev_t.hardware_p = ptr::addr_of_mut!(_self.hardware) as *mut c_void;
        _self
    }
    pub fn enable(&mut self) {
        self.hardware
            .xshut_pin
            .set_high()
            .expect("setting pin state is infallible");
    }
    pub fn read_byte(&mut self, index: u16) -> Result<u8, Error> {
        let mut data: u8 = 0;
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_RdByte(pdev, index, &mut data) })?;
        Ok(data)
    }
    pub fn get_uid(&mut self, delay: &mut Delay) -> Result<u64, Error> {
        let mut id = 0u64;
        self.with_delay(delay, |pdev| unsafe {
            bindings::VL53LX_GetUID(pdev, &mut id)
        })?;
        Ok(id)
    }
    pub fn wait_device_booted(&mut self, delay: &mut Delay) -> Result<(), Error> {
        self.with_delay(delay, |pdev| unsafe {
            bindings::VL53LX_WaitDeviceBooted(pdev)
        })?;
        Ok(())
    }
    pub fn data_init(&mut self, delay: &mut Delay) -> Result<(), Error> {
        self.with_delay(delay, |pdev| unsafe { bindings::VL53LX_DataInit(pdev) })?;
        Ok(())
    }
    pub fn start_measurement(&mut self) -> Result<(), Error> {
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_StartMeasurement(pdev) })?;
        Ok(())
    }
    pub fn wait_measurement_data_ready(&mut self, delay: &mut Delay) -> Result<(), Error> {
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
    pub fn get_measurement_data_ready(&mut self) -> Result<bool, Error> {
        let mut data = 0u8;
        self.with_pdev(|pdev| unsafe {
            bindings::VL53LX_GetMeasurementDataReady(pdev, &mut data)
        })?;
        Ok(data == 1)
    }
    pub fn set_measurement_timing_budget_ms(&mut self, ms: u32) -> Result<(), Error> {
        self.with_pdev(|pdev| unsafe {
            bindings::VL53LX_SetMeasurementTimingBudgetMicroSeconds(pdev, ms * 1000)
        })?;
        Ok(())
    }
    pub fn set_distance_mode(&mut self, mode: u8) -> Result<(), Error> {
        self.with_pdev(|pdev| unsafe { bindings::VL53LX_SetDistanceMode(pdev, mode) })?;
        Ok(())
    }
    fn with_pdev<F>(&mut self, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut VL53LX_Dev_t) -> VL53LX_Error,
    {
        match f(&mut self.dev_t) {
            0 => Ok(()),
            status => Err(Error::from(status)),
        }
    }
    fn with_delay<F>(&mut self, delay: &mut Delay, f: F) -> Result<(), Error>
    where
        F: FnMut(&mut VL53LX_Dev_t) -> VL53LX_Error,
    {
        let hardware_p = self.dev_t.hardware_p as *mut Hardware<I2C, XSHUT>;
        let hardware = unsafe { hardware_p.as_mut() }.unwrap();
        hardware.delay_p = delay;
        let result = self.with_pdev(f);
        hardware.delay_p = ptr::null_mut();
        result
    }
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

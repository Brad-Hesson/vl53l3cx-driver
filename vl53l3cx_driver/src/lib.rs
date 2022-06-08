#![no_std]

mod defaults;
mod wrapper;

use core::{convert::Infallible, ptr, slice};
use cty::c_void;
#[allow(unused_imports)]
use rtt_target::rprintln;
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

pub struct VL53L3CX<I2C, XSHUT>
where
    I2C: Write + Read,
{
    dev_t: VL53LX_Dev_t,
    hardware: Hardware<I2C, XSHUT>,
}

impl<I2C, XSHUT> VL53L3CX<I2C, XSHUT>
where
    I2C: Write<Error = i2c::Error> + Read<Error = i2c::Error>,
    XSHUT: OutputPin<Error = Infallible>,
{
    pub fn new(i2c: I2C, i2c_address: u8, xshut_p: XSHUT) -> Self {
        let hardware = Hardware::<I2C, XSHUT> {
            i2c_address,
            i2c,
            xshut_p,
            delay: ptr::null_mut(),
        };
        let mut _self = Self {
            hardware,
            dev_t: VL53LX_Dev_t {
                hardware: ptr::null_mut(),
                read_f: Some(Hardware::<I2C, XSHUT>::read),
                write_f: Some(Hardware::<I2C, XSHUT>::write),
                wait_us_f: Some(Hardware::<I2C, XSHUT>::wait_us),
                Data: Default::default(),
            },
        };
        _self.dev_t.hardware = ptr::addr_of_mut!(_self.hardware) as *mut c_void;
        _self
    }
    pub fn enable(&mut self) {
        self.hardware
            .xshut_p
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
    pub fn with_pdev<F>(&mut self, mut f: F) -> Result<(), Error>
    where
        F: FnMut(&mut VL53LX_Dev_t) -> VL53LX_Error,
    {
        match f(&mut self.dev_t) {
            0 => Ok(()),
            status => Err(Error::from(status)),
        }
    }
    pub fn with_delay<F>(&mut self, delay: &mut Delay, f: F) -> Result<(), Error>
    where
        F: FnMut(&mut VL53LX_Dev_t) -> VL53LX_Error,
    {
        let hw = unsafe { &mut *(self.dev_t.hardware as *mut Hardware<I2C, XSHUT>) };
        hw.delay = delay;
        self.with_pdev(f)?;
        hw.delay = ptr::null_mut();
        Ok(())
    }
}

struct Hardware<I2C, XSHUT>
where
    I2C: Write + Read,
{
    i2c_address: u8,
    i2c: I2C,
    pub xshut_p: XSHUT,
    delay: *mut Delay,
}

impl<I2C, XSHUT> Hardware<I2C, XSHUT>
where
    I2C: Write<Error = i2c::Error> + Read<Error = i2c::Error>,
{
    unsafe extern "C" fn read(
        pdev: *mut VL53LX_Dev_t,
        index: u16,
        data: *mut u8,
        count: u32,
    ) -> VL53LX_Error {
        let _self = &mut *((*pdev).hardware as *mut Self);
        let buffer = [(index >> 8) as u8, index as u8];
        match _self.i2c.write(_self.i2c_address / 2, &buffer) {
            Err(e) => {
                rprintln!("Writing Index: {:?}", e);
                return -13;
            }
            Ok(_) => {}
        };
        let mut buffer = slice::from_raw_parts_mut(data, count as usize);
        match _self.i2c.read(_self.i2c_address / 2, &mut buffer) {
            Err(e) => {
                rprintln!("Reading Value: {:?}", e);
                return -13;
            }
            Ok(_) => return 0,
        };
    }
    unsafe extern "C" fn write(
        pdev: *mut VL53LX_Dev_t,
        index: u16,
        data: *mut u8,
        count: u32,
    ) -> VL53LX_Error {
        let _self = &mut *((*pdev).hardware as *mut Self);
        let mut buffer = [0u8; 256];
        buffer[0] = (index >> 8) as u8;
        buffer[1] = index as u8;
        let mut i = 2;
        for byte in slice::from_raw_parts(data, count as usize) {
            buffer[i] = *byte;
            i += 1;
        }
        let buffer_slice = slice::from_raw_parts(&buffer as *const u8, (count + 2) as usize);
        match _self.i2c.write(_self.i2c_address / 2, buffer_slice) {
            Err(_) => return -13,
            Ok(_) => return 0,
        }
    }
    unsafe extern "C" fn wait_us(pdev: *mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
        let _self = &mut *((*pdev).hardware as *mut Self);
        match _self.delay.as_mut() {
            None => panic!("wait function requires delay to be loaded"),
            Some(delay) => {
                delay.delay_us(count);
                0
            }
        }
    }
}

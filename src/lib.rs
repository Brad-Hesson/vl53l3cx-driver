#![cfg_attr(not(feature = "std"), no_std)]
#![feature(c_variadic, type_changing_struct_update)]

mod bindings;
mod hardware;
mod maybe_box;
mod types;
mod wrapper;

use crate::maybe_box::MaybeBox;
pub use crate::types::{DistanceMode, Error, MultiRangingData, RangeStatus, TargetRangeData, Roi};
use crate::{bindings::*, wrapper::vl53lx_platform_user_data::VL53LX_Dev_t};
use ::core::{fmt::Debug, marker::PhantomData, ptr};
use ::embedded_hal::{
    blocking::{
        delay::{DelayMs, DelayUs},
        i2c::{Read, Write},
    },
    digital::v2::OutputPin,
};
use core::time::Duration;
pub use measurements::Length;

pub struct Vl53l3cx<'a, I2C, XSHUT, DELAY> {
    xshut_pin: XSHUT,
    dev_t: MaybeBox<VL53LX_Dev_t<'a>>,
    _i2c: PhantomData<I2C>,
    _delay: PhantomData<DELAY>,
}
impl<'a, I2C, XSHUT, DELAY> Debug for Vl53l3cx<'a, I2C, XSHUT, DELAY> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Vl53l3cx").finish()
    }
}
impl<'a, I2C, XSHUT, DELAY> Vl53l3cx<'a, I2C, XSHUT, DELAY>
where
    I2C: Read + Write + 'a,
    DELAY: DelayUs<u32> + DelayMs<u32> + 'a,
    XSHUT: OutputPin,
    <XSHUT as OutputPin>::Error: Debug,
{
    /// Contruct the sensor struct.
    pub fn new(i2c_address: u8, xshut_pin: XSHUT) -> Self {
        Self {
            dev_t: MaybeBox::new(VL53LX_Dev_t {
                Data: unsafe { ::core::mem::zeroed() },
                i2c_pointer: ptr::null_mut::<I2C>(),
                delay_pointer: ptr::null_mut::<DELAY>(),
                i2c_address,
            }),
            xshut_pin,
            _i2c: PhantomData,
            _delay: PhantomData,
        }
    }
    /// Enable the sensor.
    pub fn enable(&mut self, i2c: &mut I2C, delay: &mut DELAY) -> Result<(), Error> {
        self.xshut_pin
            .set_high()
            .expect("setting pin state is infallible");
        self.with_i2c_and_delay(i2c, delay, |pdev| {
            result(unsafe { VL53LX_WaitDeviceBooted(pdev) })?;
            result(unsafe { VL53LX_DataInit(pdev) })
        })
    }
    /// Disable the sensor.
    pub fn disable(&mut self) {
        self.xshut_pin
            .set_low()
            .expect("setting pin state is infallible");
    }
    /// Release the pin contained in the sensor struct.
    pub fn release(self) -> XSHUT {
        self.xshut_pin
    }

    /// Reads the Product Revision for a for given device.
    pub fn get_product_revision(&mut self) -> Result<(u8, u8), Error> {
        let mut major = 0u8;
        let mut minor = 0u8;
        result(unsafe { VL53LX_GetProductRevision(self.dev_t.as_mut(), &mut major, &mut minor) })?;
        Ok((major, minor))
    }
    /// Reads the Device information for given Device.
    pub fn get_device_info(&mut self) -> Result<VL53LX_DeviceInfo_t, Error> {
        let mut dev_info = VL53LX_DeviceInfo_t::default();
        result(unsafe { VL53LX_GetDeviceInfo(self.dev_t.as_mut(), &mut dev_info) })?;
        Ok(dev_info)
    }
    /// Reads the Device unique identifier.
    pub fn get_uid(&mut self, i2c: &mut I2C, delay: &mut DELAY) -> Result<u64, Error> {
        self.with_i2c_and_delay(i2c, delay, |pdev| {
            let mut id = 0u64;
            result(unsafe { VL53LX_GetUID(pdev, &mut id) })?;
            Ok(id)
        })
    }
    /// Set new device address. After completion the device will answer to the new address programmed. This function should be called when several devices are used in parallel before start programming the sensor. When a single device us used, there is no need to call this function.
    pub fn set_device_address(&mut self, i2c: &mut I2C, address: u8) -> Result<(), Error> {
        self.with_i2c(i2c, |pdev| {
            result(unsafe { VL53LX_SetDeviceAddress(pdev, address) })
        })?;
        self.dev_t.i2c_address = address;
        Ok(())
    }
    /// Set the distance mode to be used for the next ranging. The modes Short, Medium and Long are used to optimize the ranging accuracy in a specific range of distance. The user select one of these modes to select the distance range.
    pub fn set_distance_mode(&mut self, mode: DistanceMode) -> Result<(), Error> {
        result(unsafe { VL53LX_SetDistanceMode(self.dev_t.as_mut(), mode.into()) })
    }
    /// Get the distance mode used for the next ranging.
    pub fn get_distance_mode(&mut self) -> Result<DistanceMode, Error> {
        let mut mode = 0u8;
        result(unsafe { VL53LX_GetDistanceMode(self.dev_t.as_mut(), &mut mode) })?;
        Ok(mode.try_into().unwrap())
    }
    /// Defines the maximum time allowed by the user to the device to run a full ranging sequence for the current mode (ranging, histogram, ASL ...)
    pub fn set_measurement_timing_budget(&mut self, duration: Duration) -> Result<(), Error> {
        result(unsafe {
            VL53LX_SetMeasurementTimingBudgetMicroSeconds(
                self.dev_t.as_mut(),
                duration.as_micros() as u32,
            )
        })
    }
    /// Returns the programmed the maximum time allowed by the user to the device to run a full ranging sequence for the current mode (ranging, histogram, ...)
    pub fn get_measurement_timing_budget(&mut self) -> Result<Duration, Error> {
        let mut micros = 0u32;
        result(unsafe {
            VL53LX_GetMeasurementTimingBudgetMicroSeconds(self.dev_t.as_mut(), &mut micros)
        })?;
        Ok(Duration::from_micros(micros as u64))
    }
    /// Start device measurement. Started measurement will depend on distance parameter set through VL53LX_SetDistanceMode()
    pub fn start_measurement(&mut self, i2c: &mut I2C) -> Result<(), Error> {
        self.with_i2c(i2c, |pdev| result(unsafe { VL53LX_StartMeasurement(pdev) }))
    }
    /// Stop device measurement. Will set the device in standby mode at end of current measurement. Not necessary in single mode as device shall return automatically in standby mode at end of measurement.
    pub fn stop_measurement(&mut self, i2c: &mut I2C) -> Result<(), Error> {
        self.with_i2c(i2c, |pdev| result(unsafe { VL53LX_StopMeasurement(pdev) }))
    }
    /// Clear the Interrupt flag and start new measurement.
    pub fn clear_interrupt_and_start_measurement(&mut self, i2c: &mut I2C) -> Result<(), Error> {
        self.with_i2c(i2c, |pdev| {
            result(unsafe { VL53LX_ClearInterruptAndStartMeasurement(pdev) })
        })
    }
    /// This function indicate that a measurement data is ready. This function is used for non-blocking capture.
    pub fn get_measurement_data_ready(&mut self, i2c: &mut I2C) -> Result<bool, Error> {
        self.with_i2c(i2c, |pdev| {
            let mut data = 0u8;
            result(unsafe { VL53LX_GetMeasurementDataReady(pdev, &mut data) })?;
            Ok(data == 1)
        })
    }
    /// Wait for measurement data ready. Blocking function.
    pub fn wait_measurement_data_ready(
        &mut self,
        i2c: &mut I2C,
        delay: &mut DELAY,
    ) -> Result<(), Error> {
        self.with_i2c_and_delay(i2c, delay, |pdev| {
            result(unsafe { VL53LX_WaitMeasurementDataReady(pdev) })
        })
    }
    /// Get data from last successful Ranging measurement.
    pub fn get_multiranging_data(&mut self, i2c: &mut I2C) -> Result<MultiRangingData, Error> {
        self.with_i2c(i2c, |pdev| {
            let mut data = VL53LX_MultiRangingData_t::default();
            result(unsafe { VL53LX_GetMultiRangingData(pdev, &mut data) })?;
            Ok(data.into())
        })
    }
    /// Get additional data from last successful Ranging measurement.
    pub fn get_additional_data(&mut self) -> Result<VL53LX_AdditionalData_t, Error> {
        let mut data = VL53LX_AdditionalData_t::default();
        result(unsafe { VL53LX_GetAdditionalData(self.dev_t.as_mut(), &mut data) })?;
        Ok(data)
    }
    pub fn set_roi(&mut self, roi: Roi) -> Result<(), Error> {
        result(unsafe { VL53LX_SetUserROI(self.dev_t.as_mut(), &mut roi.into()) })?;
        Ok(())
    }
    pub fn get_roi(&mut self) -> Result<Roi, Error> {
        let mut roi = VL53LX_UserRoi_t::default();
        result(unsafe { VL53LX_GetUserROI(self.dev_t.as_mut(), &mut roi) })?;
        Ok(roi.into())
    }
    #[inline]
    fn with_i2c<F, T>(&mut self, i2c: &mut I2C, f: F) -> Result<T, Error>
    where
        F: FnOnce(&mut VL53LX_Dev_t) -> Result<T, Error>,
    {
        self.dev_t.i2c_pointer = i2c;
        let result = f(self.dev_t.as_mut());
        self.dev_t.i2c_pointer = ptr::null_mut::<I2C>();
        result
    }
    #[inline]
    fn with_i2c_and_delay<F, T>(
        &mut self,
        i2c: &mut I2C,
        delay: &mut DELAY,
        f: F,
    ) -> Result<T, Error>
    where
        F: FnOnce(&mut VL53LX_Dev_t) -> Result<T, Error>,
    {
        self.dev_t.delay_pointer = delay;
        let result = self.with_i2c(i2c, f);
        self.dev_t.delay_pointer = ptr::null_mut::<DELAY>();
        result
    }
}
#[inline(always)]
fn result(code: i8) -> Result<(), Error> {
    match code {
        0 => Ok(()),
        code => Err(code.try_into().expect("got invalid error code")),
    }
}

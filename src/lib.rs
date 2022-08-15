#![cfg_attr(not(feature = "std"), no_std)]
#![feature(c_variadic, type_changing_struct_update)]

mod bindings;
mod hardware;
mod maybe_box;
mod types;
mod wrapper;

use crate::maybe_box::MaybeBox;
use crate::{bindings::*, types::Vl53lxError, wrapper::vl53lx_platform_user_data::VL53LX_Dev_t};
pub use crate::{
    bindings::{
        VL53LX_AdditionalData_t, VL53LX_DeviceInfo_t, VL53LX_MultiRangingData_t,
        VL53LX_TargetRangeData_t, VL53LX_Version_t,
    },
    types::DistanceMode,
};
use ::core::{fmt::Debug, marker::PhantomData, ptr};
use ::embedded_hal::{
    blocking::{
        delay::{DelayMs, DelayUs},
        i2c::{Read, Write},
    },
    digital::v2::OutputPin,
};

pub struct VL53L3CX<'a, STATE, I2C, XSHUT, DELAY> {
    xshut_pin: XSHUT,
    pub dev_t: MaybeBox<VL53LX_Dev_t<'a>>,
    _i2c: PhantomData<I2C>,
    _delay: PhantomData<DELAY>,
    _state: STATE,
}
impl<'a, STATE: Debug, I2C, XSHUT, DELAY> Debug for VL53L3CX<'a, STATE, I2C, XSHUT, DELAY> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VL53L3CX")
            .field("state", &self._state)
            .finish()
    }
}
#[derive(Debug)]
pub struct Enabled;
#[derive(Debug)]
pub struct Disabled;
impl<'a, I2C, XSHUT, DELAY> VL53L3CX<'a, Disabled, I2C, XSHUT, DELAY>
where
    I2C: Read + Write + 'a,
    DELAY: DelayUs<u32> + DelayMs<u32> + 'a,
{
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
            _state: Disabled,
        }
    }
    pub fn into_enabled(
        mut self,
        i2c: &mut I2C,
        delay: &mut DELAY,
    ) -> Result<VL53L3CX<'a, Enabled, I2C, XSHUT, DELAY>, (Self, Vl53lxError)>
    where
        XSHUT: OutputPin,
        <XSHUT as OutputPin>::Error: Debug,
    {
        self.xshut_pin
            .set_high()
            .expect("setting pin state is infallible");
        match self.with_i2c_and_delay(i2c, delay, |pdev| {
            result(unsafe { VL53LX_WaitDeviceBooted(pdev) })?;
            result(unsafe { VL53LX_DataInit(pdev) })
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
impl<'a, I2C, XSHUT, DELAY> VL53L3CX<'a, Enabled, I2C, XSHUT, DELAY>
where
    I2C: Write + Read + 'a,
    DELAY: DelayUs<u32> + DelayMs<u32> + 'a,
{
    pub fn into_disabled(mut self) -> VL53L3CX<'a, Disabled, I2C, XSHUT, DELAY>
    where
        XSHUT: OutputPin,
        <XSHUT as OutputPin>::Error: Debug,
    {
        self.xshut_pin
            .set_low()
            .expect("setting pin state is infallible");
        VL53L3CX {
            _state: Disabled,
            ..self
        }
    }
    pub fn get_product_revision(&mut self) -> Result<(u8, u8), Vl53lxError> {
        let mut major = 0u8;
        let mut minor = 0u8;
        result(unsafe { VL53LX_GetProductRevision(self.dev_t.as_mut(), &mut major, &mut minor) })?;
        Ok((major, minor))
    }
    pub fn get_device_info(&mut self) -> Result<VL53LX_DeviceInfo_t, Vl53lxError> {
        let mut dev_info = VL53LX_DeviceInfo_t::default();
        result(unsafe { VL53LX_GetDeviceInfo(self.dev_t.as_mut(), &mut dev_info) })?;
        Ok(dev_info)
    }
    pub fn get_uid(&mut self, i2c: &mut I2C, delay: &mut DELAY) -> Result<u64, Vl53lxError> {
        self.with_i2c_and_delay(i2c, delay, |pdev| {
            let mut id = 0u64;
            result(unsafe { VL53LX_GetUID(pdev, &mut id) })?;
            Ok(id)
        })
    }
    pub fn set_device_address(&mut self, i2c: &mut I2C, address: u8) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            result(unsafe { VL53LX_SetDeviceAddress(pdev, address) })
        })?;
        self.dev_t.i2c_address = address;
        Ok(())
    }
    pub fn set_distance_mode(&mut self, mode: DistanceMode) -> Result<(), Vl53lxError> {
        result(unsafe { VL53LX_SetDistanceMode(self.dev_t.as_mut(), mode.into()) })
    }
    pub fn get_distance_mode(&mut self) -> Result<DistanceMode, Vl53lxError> {
        let mut mode = 0u8;
        result(unsafe { VL53LX_GetDistanceMode(self.dev_t.as_mut(), &mut mode) })?;
        Ok(mode.try_into().unwrap())
    }
    pub fn set_measurement_timing_budget_ms(&mut self, ms: u32) -> Result<(), Vl53lxError> {
        result(unsafe {
            VL53LX_SetMeasurementTimingBudgetMicroSeconds(self.dev_t.as_mut(), ms * 1000)
        })
    }
    pub fn get_measurement_timing_budget_ms(&mut self) -> Result<u32, Vl53lxError> {
        let mut ms = 0u32;
        result(unsafe {
            VL53LX_GetMeasurementTimingBudgetMicroSeconds(self.dev_t.as_mut(), &mut ms)
        })?;
        Ok(ms / 1000)
    }
    pub fn start_measurement(&mut self, i2c: &mut I2C) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| result(unsafe { VL53LX_StartMeasurement(pdev) }))
    }
    pub fn stop_measurement(&mut self, i2c: &mut I2C) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| result(unsafe { VL53LX_StopMeasurement(pdev) }))
    }
    pub fn clear_interrupt_and_start_measurement(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<(), Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            result(unsafe { VL53LX_ClearInterruptAndStartMeasurement(pdev) })
        })
    }
    pub fn get_measurement_data_ready(&mut self, i2c: &mut I2C) -> Result<bool, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut data = 0u8;
            result(unsafe { VL53LX_GetMeasurementDataReady(pdev, &mut data) })?;
            Ok(data == 1)
        })
    }
    pub fn wait_measurement_data_ready(
        &mut self,
        i2c: &mut I2C,
        delay: &mut DELAY,
    ) -> Result<(), Vl53lxError> {
        self.with_i2c_and_delay(i2c, delay, |pdev| {
            result(unsafe { VL53LX_WaitMeasurementDataReady(pdev) })
        })
    }
    pub fn get_multiranging_data(
        &mut self,
        i2c: &mut I2C,
    ) -> Result<VL53LX_MultiRangingData_t, Vl53lxError> {
        self.with_i2c(i2c, |pdev| {
            let mut data = VL53LX_MultiRangingData_t::default();
            result(unsafe { VL53LX_GetMultiRangingData(pdev, &mut data) })?;
            Ok(data)
        })
    }
    pub fn get_additional_data(&mut self) -> Result<VL53LX_AdditionalData_t, Vl53lxError> {
        let mut data = VL53LX_AdditionalData_t::default();
        result(unsafe { VL53LX_GetAdditionalData(self.dev_t.as_mut(), &mut data) })?;
        Ok(data)
    }
}
impl<'a, STATE, I2C, XSHUT, DELAY> VL53L3CX<'a, STATE, I2C, XSHUT, DELAY>
where
    I2C: Read + Write + 'a,
    DELAY: DelayUs<u32> + DelayMs<u32> + 'a,
{
    #[inline]
    fn with_i2c<F, T>(&mut self, i2c: &mut I2C, f: F) -> Result<T, Vl53lxError>
    where
        F: FnOnce(&mut VL53LX_Dev_t) -> Result<T, Vl53lxError>,
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
    ) -> Result<T, Vl53lxError>
    where
        F: FnOnce(&mut VL53LX_Dev_t) -> Result<T, Vl53lxError>,
    {
        self.dev_t.delay_pointer = delay;
        let result = self.with_i2c(i2c, f);
        self.dev_t.delay_pointer = ptr::null_mut::<DELAY>();
        result
    }
}
#[inline(always)]
fn result(code: i8) -> Result<(), Vl53lxError> {
    match code {
        0 => Ok(()),
        code => Err(code.try_into().expect("got invalid error code")),
    }
}

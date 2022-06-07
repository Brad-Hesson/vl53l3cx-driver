#![no_std]

mod defaults;
mod wrapper;

use core::{convert::Infallible, ptr, slice};
use cty::c_void;
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

pub struct VL53L3CX<I2C, XSHUT>
where
    I2C: Write + Read,
{
    dev_t: bindings::VL53LX_Dev_t,
    hardware: Hardware<I2C, XSHUT>,
}

impl<'a, I2C, XSHUT> VL53L3CX<I2C, XSHUT>
where
    I2C: Write<Error = i2c::Error> + Read<Error = i2c::Error>,
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
                Data: Default::default(),
                read_f: Some(Hardware::<I2C, XSHUT>::read),
                write_f: Some(Hardware::<I2C, XSHUT>::write),
                wait_us_f: Some(Hardware::<I2C, XSHUT>::wait_us),
            },
        };
        _self.dev_t.hardware = ptr::addr_of_mut!(_self.hardware) as *mut c_void;
        _self
    }
    pub fn enable(&mut self)
    where
        XSHUT: OutputPin<Error = Infallible>,
    {
        self.hardware
            .xshut_p
            .set_high()
            .expect("setting pin state is infallible");
    }
    pub fn read_byte(&mut self, index: u16) -> u8 {
        let pdev = &mut self.dev_t;
        let mut data: u8 = 0;
        unsafe { bindings::VL53LX_RdByte(pdev, index, &mut data) };
        data
    }
    pub fn get_uid(&mut self, delay: &mut Delay) -> u64 {
        let pdev = &mut self.dev_t;
        let mut hw = unsafe { &mut *((*pdev).hardware as *mut Hardware<I2C, XSHUT>) };
        hw.delay = delay;
        let mut id = 0u64;
        unsafe { bindings::VL53LX_GetUID(pdev, &mut id) };
        hw.delay = ptr::null_mut();
        id
    }
    pub fn wait_device_booted(&mut self, delay: &mut Delay) {
        let pdev = &mut self.dev_t;
        let mut hw = unsafe { &mut *((*pdev).hardware as *mut Hardware<I2C, XSHUT>) };
        hw.delay = delay;
        unsafe { bindings::VL53LX_WaitDeviceBooted(pdev) };
        hw.delay = ptr::null_mut();
    }
    pub fn data_init(&mut self, delay: &mut Delay) {
        let pdev = &mut self.dev_t;
        let mut hw = unsafe { &mut *((*pdev).hardware as *mut Hardware<I2C, XSHUT>) };
        hw.delay = delay;
        unsafe { bindings::VL53LX_DataInit(pdev) };
        hw.delay = ptr::null_mut();
    }
    pub fn start_measurement(&mut self) {
        let pdev = &mut self.dev_t;
        unsafe { bindings::VL53LX_StartMeasurement(pdev) };
    }
    pub fn wait_measurement_data_ready(&mut self) {
        let pdev = &mut self.dev_t;
        unsafe { bindings::VL53LX_WaitMeasurementDataReady(pdev) };
    }
    pub fn get_multiranging_data(&mut self) -> bindings::VL53LX_MultiRangingData_t {
        let pdev = &mut self.dev_t;
        let mut data = bindings::VL53LX_MultiRangingData_t::default();
        unsafe { bindings::VL53LX_GetMultiRangingData(pdev, &mut data) };
        data
    }
    pub fn get_measurement_data_ready(&mut self) -> bool {
        let pdev = &mut self.dev_t;
        let mut data = 0u8;
        unsafe { bindings::VL53LX_GetMeasurementDataReady(pdev, &mut data) };
        data == 1
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

impl<'a, I2C, XSHUT> Hardware<I2C, XSHUT>
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
        _self.i2c.write(_self.i2c_address / 2, &buffer).unwrap();
        let mut buffer = slice::from_raw_parts_mut(data, count as usize);
        _self.i2c.read(_self.i2c_address / 2, &mut buffer).unwrap();
        Default::default()
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
        _self
            .i2c
            .write(_self.i2c_address / 2, buffer_slice)
            .unwrap();
        Default::default()
    }
    unsafe extern "C" fn wait_us(pdev: *mut VL53LX_Dev_t, count: u32) {
        let _self = &mut *((*pdev).hardware as *mut Self);
        let pdelay = _self.delay;
        if pdelay.is_null() {
            panic!("wait function requires delay to be loaded")
        }
        (*pdelay).delay_us(count);
    }
}

use crate::{bindings::VL53LX_DevData_t, Vl53lxError};
use ::embedded_hal::blocking::{
    delay::{DelayMs, DelayUs},
    i2c::{Read, Write},
};

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VL53LX_Dev_t<'a> {
    pub Data: VL53LX_DevData_t,
    pub i2c_address: u8,
    pub i2c_p: *mut (dyn I2C + 'a),
    pub delay_p: *mut (dyn Delay + 'a),
}

#[allow(non_camel_case_types)]
pub type VL53LX_DEV<'a> = *mut VL53LX_Dev_t<'a>;
pub trait Delay: DelayUs<u32> + DelayMs<u32> {}
impl<T> Delay for T where T: DelayUs<u32> + DelayMs<u32> {}

pub trait I2C {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Vl53lxError>;
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Vl53lxError>;
}
impl<T> I2C for T
where
    T: Read + Write,
{
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Vl53lxError> {
        self.read(address, buffer)
            .or(Err(Vl53lxError::ControlInterface))
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Vl53lxError> {
        self.write(address, bytes)
            .or(Err(Vl53lxError::ControlInterface))
    }
}

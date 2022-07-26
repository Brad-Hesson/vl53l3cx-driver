use crate::bindings::VL53LX_DevData_t;
use crate::hardware::{Delay, I2C};

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VL53LX_Dev_t<'a> {
    pub Data: VL53LX_DevData_t,
    pub i2c_address: u8,
    pub i2c_pointer: *mut (dyn I2C + 'a),
    pub delay_pointer: *mut (dyn Delay + 'a),
}

#[allow(non_camel_case_types)]
pub type VL53LX_DEV<'a> = *mut VL53LX_Dev_t<'a>;

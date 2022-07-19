use crate::bindings::VL53LX_DevData_t;
use crate::hardware::Hardwary;

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct VL53LX_Dev_t {
    pub hardware_p: *mut dyn Hardwary,
    pub Data: VL53LX_DevData_t,
}

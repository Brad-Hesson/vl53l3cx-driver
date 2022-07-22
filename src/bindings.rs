#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
use crate::wrapper::vl53lx_platform_user_data::VL53LX_DEV;
use crate::VL53LX_Dev_t;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

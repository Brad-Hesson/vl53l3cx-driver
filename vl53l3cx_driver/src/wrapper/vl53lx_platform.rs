#![allow(dead_code)]
#![allow(unused_variables)]

use crate::bindings::{VL53LX_Dev_t, VL53LX_Error};
use core::ptr;

// use rtt_target::rprintln;

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WriteMulti(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
    count: u32,
) -> VL53LX_Error {
    // rprintln!("VL53LX_WriteMulti");
    let write_f = (*pdev).write_f.unwrap();
    write_f(pdev, index, pdata, count)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_ReadMulti(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
    count: u32,
) -> VL53LX_Error {
    // rprintln!("VL53LX_ReadMulti");
    let read_f = (*pdev).read_f.unwrap();
    read_f(pdev, index, pdata, count)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WrByte(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    mut data: u8,
) -> VL53LX_Error {
    // rprintln!("VL53LX_WrByte");
    let write_f = (*pdev).write_f.unwrap();
    write_f(pdev, index, &mut data, 1)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WrWord(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    data: u16,
) -> VL53LX_Error {
    // rprintln!("VL53LX_WrWord");
    let write_f = (*pdev).write_f.unwrap();
    let ptr = ptr::addr_of!(data);
    write_f(pdev, index, ptr as *mut u8, 2)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WrDWord(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    data: u32,
) -> VL53LX_Error {
    // rprintln!("VL53LX_WrDWord");
    let write_f = (*pdev).write_f.unwrap();
    let ptr = ptr::addr_of!(data);
    write_f(pdev, index, ptr as *mut u8, 4)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_RdByte(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
) -> VL53LX_Error {
    // rprintln!("VL53LX_RdByte");
    let read_f = (*pdev).read_f.unwrap();
    read_f(pdev, index, pdata, 1)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_RdWord(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u16,
) -> VL53LX_Error {
    // rprintln!("VL53LX_RdWord");
    let read_f = (*pdev).read_f.unwrap();
    read_f(pdev, index, pdata as *mut u8, 2)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_RdDWord(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u32,
) -> VL53LX_Error {
    // rprintln!("VL53LX_RdDWord");
    let read_f = (*pdev).read_f.unwrap();
    read_f(pdev, index, pdata as *mut u8, 4)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WaitUs(pdev: *mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
    // rprintln!("VL53LX_WaitUs");
    let wait_us_f = (*pdev).wait_us_f.unwrap();
    wait_us_f(pdev, count);
    Default::default()
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WaitMs(pdev: *mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
    // rprintln!("VL53LX_WaitMs");
    let wait_us_f = (*pdev).wait_us_f.unwrap();
    wait_us_f(pdev, count * 1000);
    Default::default()
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WaitValueMaskEx(
    pdev: *mut VL53LX_Dev_t,
    timeout_ms: u32,
    index: u16,
    value: u8,
    mask: u8,
    poll_delay_ms: u32,
) -> VL53LX_Error {
    let mut buffer = 0u8;
    for _ in 0..(timeout_ms / poll_delay_ms) {
        VL53LX_RdByte(pdev, index, &mut buffer);
        if buffer & mask == value {
            break;
        }
        VL53LX_WaitMs(pdev, poll_delay_ms);
    }
    // rprintln!("VL53LX_WaitMs");
    // let wait_us_f = (*pdev).wait_us_f.unwrap();
    // wait_us_f(pdev, count * 1000);
    Default::default()
}

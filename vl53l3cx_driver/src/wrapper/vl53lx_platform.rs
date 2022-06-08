#![allow(dead_code)]
#![allow(unused_variables)]

use crate::bindings::{VL53LX_Dev_t, VL53LX_Error};
use core::ptr;
use core::slice;

use rtt_target::{rprint, rprintln};
const DEBUG: bool = false;

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WriteMulti(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
    count: u32,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WriteMulti");
    }
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
    let read_f = (*pdev).read_f.unwrap();
    let s = read_f(pdev, index, pdata, count);
    if DEBUG {
        rprint!("VL53LX_ReadMulti: 0x{:04X} => ", index);
        for b in slice::from_raw_parts_mut(pdata, count as usize) {
            rprint!("0x{:02X} ", b);
        }
        rprintln!();
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WrByte(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    mut data: u8,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WrByte: 0x{:04X} <= 0x{:02X}", index, data);
    }
    let write_f = (*pdev).write_f.unwrap();
    write_f(pdev, index, &mut data, 1)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WrWord(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    data: u16,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WrWord: 0x{:04X} <= 0x{:04X}", index, data);
    }
    let write_f = (*pdev).write_f.unwrap();
    write_f(pdev, index, ptr::addr_of!(data) as *mut u8, 2)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WrDWord(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    data: u32,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WrDWord: 0x{:04X} <= 0x{:08X}", index, data);
    }
    let write_f = (*pdev).write_f.unwrap();
    write_f(pdev, index, ptr::addr_of!(data) as *mut u8, 4)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_RdByte(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_RdByte: 0x{:04X}", index);
    }
    let read_f = (*pdev).read_f.unwrap();
    read_f(pdev, index, pdata, 1)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_RdWord(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u16,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_RdWord: 0x{:04X}", index);
    }
    let read_f = (*pdev).read_f.unwrap();
    read_f(pdev, index, pdata as *mut u8, 2)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_RdDWord(
    pdev: *mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u32,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_RdDWord: 0x{:04X}", index);
    }
    let read_f = (*pdev).read_f.unwrap();
    read_f(pdev, index, pdata as *mut u8, 4)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WaitUs(pdev: *mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WaitUs: {}us", count);
    }
    let wait_us_f = (*pdev).wait_us_f.unwrap();
    wait_us_f(pdev, count)
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_WaitMs(pdev: *mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WaitMs: {}ms", count);
    }
    let wait_us_f = (*pdev).wait_us_f.unwrap();
    wait_us_f(pdev, count * 1000)
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
    if DEBUG {
        rprintln!("VL53LX_WaitValueMaskEx");
    }
    let mut buffer = 0u8;
    for _ in 0..(timeout_ms / poll_delay_ms) {
        match VL53LX_RdByte(pdev, index, &mut buffer) {
            0 => {}
            status => return status,
        };
        if buffer & mask == value {
            return 0;
        }
        match VL53LX_WaitMs(pdev, poll_delay_ms) {
            0 => {}
            status => return status,
        };
    }
    -7
}

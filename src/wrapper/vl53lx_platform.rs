use crate::bindings::VL53LX_Error;
use crate::VL53LX_Dev_t;
use core::slice;
use rtt_target::{rprint, rprintln};

const DEBUG: bool = false;

#[no_mangle]
pub extern "C" fn VL53LX_WriteMulti(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
    count: u32,
) -> VL53LX_Error {
    let data = unsafe { slice::from_raw_parts(pdata, count as usize) };
    if DEBUG {
        rprint!("VL53LX_WriteMulti: 0x{:04X} <= ", index);
        for b in data {
            rprint!("0x{:02X} ", b);
        }
        rprintln!();
    }
    unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .write(index, data)
}

#[no_mangle]
pub extern "C" fn VL53LX_ReadMulti(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
    count: u32,
) -> VL53LX_Error {
    if DEBUG {
        rprint!("VL53LX_ReadMulti: 0x{:04X} => ", index);
    }
    let data = unsafe { slice::from_raw_parts_mut(pdata, count as usize) };
    let s = unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .read(index, data);
    if DEBUG {
        for b in data {
            rprint!("0x{:02X} ", b);
        }
        rprintln!();
    }
    s
}

#[no_mangle]
pub extern "C" fn VL53LX_WrByte(pdev: &mut VL53LX_Dev_t, index: u16, data: u8) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WrByte: 0x{:04X} <= 0x{:02X}", index, data);
    }
    let data = unsafe { slice::from_raw_parts(&data as *const u8, 1) };
    unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .write(index, data)
}

#[no_mangle]
pub extern "C" fn VL53LX_WrWord(pdev: &mut VL53LX_Dev_t, index: u16, data: u16) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WrWord: 0x{:04X} <= 0x{:04X}", index, data);
    }
    let data = unsafe { slice::from_raw_parts(&data as *const u16 as *const u8, 2) };
    unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .write(index, data)
}

#[no_mangle]
pub extern "C" fn VL53LX_WrDWord(pdev: &mut VL53LX_Dev_t, index: u16, data: u32) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WrDWord: 0x{:04X} <= 0x{:08X}", index, data);
    }
    let data = unsafe { slice::from_raw_parts(&data as *const u32 as *const u8, 4) };
    unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .write(index, data)
}

#[no_mangle]
pub extern "C" fn VL53LX_RdByte(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_RdByte: 0x{:04X}", index);
    }
    let data = unsafe { slice::from_raw_parts_mut(pdata, 1) };
    unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .read(index, data)
}

#[no_mangle]
pub extern "C" fn VL53LX_RdWord(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u16,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_RdWord: 0x{:04X}", index);
    }
    let data = unsafe { slice::from_raw_parts_mut(pdata as *mut u8, 2) };
    unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .read(index, data)
}

#[no_mangle]
pub extern "C" fn VL53LX_RdDWord(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u32,
) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_RdDWord: 0x{:04X}", index);
    }
    let data = unsafe { slice::from_raw_parts_mut(pdata as *mut u8, 4) };
    unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .read(index, data)
}

#[no_mangle]
pub extern "C" fn VL53LX_WaitUs(pdev: &mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WaitUs: {}us", count);
    }
    unsafe { pdev.hardware_p.as_mut() }.unwrap().wait_us(count)
}

#[no_mangle]
pub extern "C" fn VL53LX_WaitMs(pdev: &mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
    if DEBUG {
        rprintln!("VL53LX_WaitMs: {}ms", count);
    }
    unsafe { pdev.hardware_p.as_mut() }
        .unwrap()
        .wait_us(count * 1000)
}

#[no_mangle]
pub extern "C" fn VL53LX_WaitValueMaskEx(
    pdev: &mut VL53LX_Dev_t,
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

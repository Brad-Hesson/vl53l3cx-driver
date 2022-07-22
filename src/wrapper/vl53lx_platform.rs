#![allow(improper_ctypes_definitions)]
use crate::{VL53LX_Dev_t, Vl53lxError};
use ::core::slice;
use rtt_target::{rprint, rprintln};

const DEBUG: bool = true;


#[no_mangle]
pub extern "C" fn VL53LX_WriteMulti(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *const u8,
    count: u32,
) -> Result<(), Vl53lxError> {
    let i2c = unsafe { pdev.i2c_p.as_mut() }.expect("tried to write to a null i2c pointer");
    let data = unsafe { slice::from_raw_parts(pdata, count as usize) };
    if DEBUG {
        rprint!("VL53LX_Write: [0x{:04X}] <= ", index);
        data.iter().for_each(|byte| rprint!("0x{:02X} ", byte));
        rprintln!();
    }
    let mut alloc = [0u8; 256];
    let buffer = &mut alloc[..data.len() + 2];
    buffer[0] = (index >> 8) as u8;
    buffer[1] = index as u8;
    buffer[2..].copy_from_slice(data);
    i2c.write(pdev.i2c_address / 2, buffer)
}

#[no_mangle]
pub extern "C" fn VL53LX_ReadMulti(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
    count: u32,
) -> Result<(), Vl53lxError> {
    if DEBUG {
        rprint!("VL53LX_Read: [0x{:04X}] => ", index);
    }
    let i2c = unsafe { pdev.i2c_p.as_mut() }.expect("tried to read from a null i2c pointer");
    let data = unsafe { slice::from_raw_parts_mut(pdata, count as usize) };
    let buffer = [(index >> 8) as u8, index as u8];
    i2c.write(pdev.i2c_address / 2, &buffer)?;
    let s = i2c.read(pdev.i2c_address / 2, data);
    if DEBUG {
        data.iter().for_each(|byte| rprint!("0x{:02X} ", byte));
        rprintln!();
    }
    s
}

#[no_mangle]
pub extern "C" fn VL53LX_WrByte(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    data: u8,
) -> Result<(), Vl53lxError> {
    VL53LX_WriteMulti(pdev, index, &data, 1)
}

#[no_mangle]
pub extern "C" fn VL53LX_WrWord(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    data: u16,
) -> Result<(), Vl53lxError> {
    VL53LX_WriteMulti(pdev, index, &data as *const u16 as *const u8, 2)
}

#[no_mangle]
pub extern "C" fn VL53LX_WrDWord(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    data: u32,
) -> Result<(), Vl53lxError> {
    VL53LX_WriteMulti(pdev, index, &data as *const u32 as *const u8, 4)
}

#[no_mangle]
pub extern "C" fn VL53LX_RdByte(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u8,
) -> Result<(), Vl53lxError> {
    VL53LX_ReadMulti(pdev, index, pdata, 1)
}

#[no_mangle]
pub extern "C" fn VL53LX_RdWord(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u16,
) -> Result<(), Vl53lxError> {
    VL53LX_ReadMulti(pdev, index, pdata as *mut u8, 2)
}

#[no_mangle]
pub extern "C" fn VL53LX_RdDWord(
    pdev: &mut VL53LX_Dev_t,
    index: u16,
    pdata: *mut u32,
) -> Result<(), Vl53lxError> {
    VL53LX_ReadMulti(pdev, index, pdata as *mut u8, 4)
}

#[no_mangle]
pub extern "C" fn VL53LX_WaitUs(pdev: &mut VL53LX_Dev_t, us: u32) -> Result<(), Vl53lxError> {
    if DEBUG {
        rprintln!("VL53LX_WaitUs: {}us", us);
    }
    let delay = unsafe { pdev.delay_p.as_mut() }.expect("tried to use a null delay pointer");
    delay.delay_us(us);
    Ok(())
}

#[no_mangle]
pub extern "C" fn VL53LX_WaitMs(pdev: &mut VL53LX_Dev_t, ms: u32) -> Result<(), Vl53lxError> {
    if DEBUG {
        rprintln!("VL53LX_WaitMs: {}Ms", ms);
    }
    let delay = unsafe { pdev.delay_p.as_mut() }.expect("tried to use a null delay pointer");
    delay.delay_ms(ms);
    Ok(())
}

#[no_mangle]
pub extern "C" fn VL53LX_WaitValueMaskEx(
    pdev: &mut VL53LX_Dev_t,
    timeout_ms: u32,
    index: u16,
    value: u8,
    mask: u8,
    poll_delay_ms: u32,
) -> Result<(), Vl53lxError> {
    if DEBUG {
        rprintln!("VL53LX_WaitValueMaskEx");
    }
    let mut buffer = 0u8;
    for _ in 0..(timeout_ms / poll_delay_ms) {
        VL53LX_RdByte(pdev, index, &mut buffer)?;
        if buffer & mask == value {
            return Ok(());
        }
        VL53LX_WaitMs(pdev, poll_delay_ms)?;
    }
    Err(Vl53lxError::TimeOut)
}

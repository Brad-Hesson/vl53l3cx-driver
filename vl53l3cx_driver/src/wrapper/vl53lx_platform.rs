#![allow(dead_code)]
#![allow(unused_variables)]

use crate::bindings::{VL53LX_Dev_t, VL53LX_Error};
use core::{ffi::VaListImpl, ptr, slice};
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
        rprint!("VL53LX_WriteMulti: 0x{:04X} <= ", index);
        for b in slice::from_raw_parts_mut(pdata, count as usize) {
            rprint!("0x{:02X} ", b);
        }
        rprintln!();
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
    if DEBUG {
        rprint!("VL53LX_ReadMulti: 0x{:04X} => ", index);
    }
    let read_f = (*pdev).read_f.unwrap();
    let s = read_f(pdev, index, pdata, count);
    if DEBUG {
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

#[no_mangle]
pub unsafe extern "C" fn VL53LX_clock() -> u32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_trace_print_module_function(
    module: u32,
    level: u32,
    function: u32,
    mut str: *mut u8,
    mut args: ...
) {
    if !!!DEBUG {
        return;
    }
    if Level::from(level) != Level::Vl53LxTraceLevelDebug {
        return;
    }
    while *str != 0 {
        let c = *str as char;
        if c == '%' {
            str = parse_format(str, &mut args);
        } else {
            rprint!("{}", c);
            str = str.add(1);
        }
    }
}

unsafe fn print_cstr(mut ptr: *mut u8) {
    while *ptr != 0 {
        rprint!("{}", *ptr as char);
        ptr = ptr.add(1);
    }
}

unsafe fn parse_format(mut ptr: *mut u8, args: &mut VaListImpl) -> *mut u8 {
    if *ptr.add(1) as char == '%' {
        rprint!("%");
        ptr = ptr.add(2);
    } else {
        loop {
            ptr = ptr.add(1);
            match *ptr as char {
                'u' => {
                    rprint!("{}", args.arg::<u16>());
                    ptr = ptr.add(1);
                    break;
                }
                'd' => {
                    rprint!("{}", args.arg::<i16>());
                    ptr = ptr.add(1);
                    break;
                }
                's' => {
                    print_cstr(args.arg::<*mut u8>());
                    ptr = ptr.add(1);
                    break;
                }
                'l' => {
                    rprint!("{}", args.arg::<u32>());
                    ptr = ptr.add(2);
                    break;
                }
                'X' => {
                    rprint!("%X");
                    ptr = ptr.add(1);
                    break;
                }
                '\0' => panic!("end of string while parsing format"),
                _ => {}
            }
        }
    }
    ptr
}

#[derive(Debug, PartialEq, Eq)]
enum Level {
    Vl53LxTraceLevelNone,
    Vl53LxTraceLevelErrors,
    Vl53LxTraceLevelWarning,
    Vl53LxTraceLevelInfo,
    Vl53LxTraceLevelDebug,
    Vl53LxTraceLevelAll,
    Vl53LxTraceLevelIgnore,
}

impl From<u32> for Level {
    fn from(v: u32) -> Self {
        match v {
            0x00000000 => Self::Vl53LxTraceLevelNone,
            0x00000001 => Self::Vl53LxTraceLevelErrors,
            0x00000002 => Self::Vl53LxTraceLevelWarning,
            0x00000004 => Self::Vl53LxTraceLevelInfo,
            0x00000008 => Self::Vl53LxTraceLevelDebug,
            0x00000010 => Self::Vl53LxTraceLevelAll,
            0x00000020 => Self::Vl53LxTraceLevelIgnore,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
enum Function {
    Vl53LxTraceFunctionNone,
    Vl53LxTraceFunctionI2C,
    Vl53LxTraceFunctionAll,
}

impl From<u32> for Function {
    fn from(v: u32) -> Self {
        match v {
            0x00000000 => Self::Vl53LxTraceFunctionNone,
            0x00000001 => Self::Vl53LxTraceFunctionI2C,
            0x7fffffff => Self::Vl53LxTraceFunctionAll,
            _ => unimplemented!(),
        }
    }
}
#[derive(Debug)]
enum Module {
    Vl53LxTraceModuleNone,
    Vl53LxTraceModuleApi,
    Vl53LxTraceModuleCore,
    Vl53LxTraceModuleProtected,
    Vl53LxTraceModuleHistogram,
    Vl53LxTraceModuleRegisters,
    Vl53LxTraceModulePlatform,
    Vl53LxTraceModuleNvm,
    Vl53LxTraceModuleCalibrationData,
    Vl53LxTraceModuleNvmData,
    Vl53LxTraceModuleHistogramData,
    Vl53LxTraceModuleRangeResultsData,
    Vl53LxTraceModuleXtalkData,
    Vl53LxTraceModuleOffsetData,
    Vl53LxTraceModuleDataInit,
    Vl53LxTraceModuleRefSpadChar,
    Vl53LxTraceModuleSpadRateMap,
}
impl From<u32> for Module {
    fn from(v: u32) -> Self {
        match v {
            0x00000000 => Self::Vl53LxTraceModuleNone,
            0x00000001 => Self::Vl53LxTraceModuleApi,
            0x00000002 => Self::Vl53LxTraceModuleCore,
            0x00000004 => Self::Vl53LxTraceModuleProtected,
            0x00000008 => Self::Vl53LxTraceModuleHistogram,
            0x00000010 => Self::Vl53LxTraceModuleRegisters,
            0x00000020 => Self::Vl53LxTraceModulePlatform,
            0x00000040 => Self::Vl53LxTraceModuleNvm,
            0x00000080 => Self::Vl53LxTraceModuleCalibrationData,
            0x00000100 => Self::Vl53LxTraceModuleNvmData,
            0x00000200 => Self::Vl53LxTraceModuleHistogramData,
            0x00000400 => Self::Vl53LxTraceModuleRangeResultsData,
            0x00000800 => Self::Vl53LxTraceModuleXtalkData,
            0x00001000 => Self::Vl53LxTraceModuleOffsetData,
            0x00002000 => Self::Vl53LxTraceModuleDataInit,
            0x00004000 => Self::Vl53LxTraceModuleRefSpadChar,
            0x00008000 => Self::Vl53LxTraceModuleSpadRateMap,
            _ => unimplemented!(),
        }
    }
}

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static _trace_level: u32 = Level::Vl53LxTraceLevelDebug as u32;

#[no_mangle]
pub unsafe extern "C" fn sprintf(fmt: *mut u8, mut _str: *mut u8, args: ...) -> u32 {
    // while *_str != 0 {
    //     let c = *_str as char;
    //     if c == '%' {
    //         _str = parse_format(_str, &mut args);
    //     } else {
    //         rprint!("{}", c);
    //         _str = _str.add(1);
    //     }
    // }
    0
}

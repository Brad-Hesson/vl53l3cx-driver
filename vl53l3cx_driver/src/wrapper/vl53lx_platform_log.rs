use core::ffi::VaListImpl;
use rtt_target::rprint;

const DEBUG: bool = false;

#[no_mangle]
pub unsafe extern "C" fn VL53LX_clock() -> u32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn VL53LX_trace_print_module_function(
    _module: u32,
    level: u32,
    _function: u32,
    str: *mut u8,
    mut args: ...
) {
    if !!!DEBUG {
        return;
    }
    if Level::from(level) != Level::Vl53LxTraceLevelDebug {
        return;
    }
    print_format_cstr(str, &mut args);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static _trace_level: u32 = 0x00000008;

#[no_mangle]
pub unsafe extern "C" fn sprintf(_fmt: *mut u8, _str: *mut u8, _args: ...) -> u32 {
    0
}

unsafe fn print_format_cstr(mut ptr: *mut u8, args: &mut VaListImpl) {
    while *ptr != 0 {
        let c = *ptr as char;
        if c == '%' {
            ptr = print_format_replacement(ptr, args);
        } else {
            rprint!("{}", c);
            ptr = ptr.add(1);
        }
    }
}

unsafe fn print_cstr(mut ptr: *mut u8) {
    while *ptr != 0 {
        rprint!("{}", *ptr as char);
        ptr = ptr.add(1);
    }
}

unsafe fn print_format_replacement(mut ptr: *mut u8, args: &mut VaListImpl) -> *mut u8 {
    if *ptr.add(1) as char == '%' {
        rprint!("%");
        ptr = ptr.add(2);
    } else {
        loop {
            ptr = ptr.add(1);
            match *ptr as char {
                'u' => {
                    rprint!("{}", args.arg::<u32>());
                    ptr = ptr.add(1);
                    break;
                }
                'd' => {
                    rprint!("{}", args.arg::<i32>());
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
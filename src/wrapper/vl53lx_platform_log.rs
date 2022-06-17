use core::{ffi::VaListImpl, fmt::Write, ops::Deref, ptr};
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
    src_p: *mut u8,
    mut args: ...
) {
    if !!!DEBUG {
        return;
    }
    if Level::from(level) != Level::Vl53LxTraceLevelDebug {
        return;
    }
    let mut dst_buf = [0u8; 256];
    let mut dst_p = ptr::addr_of_mut!(dst_buf) as *mut u8;
    let mut dst = CharPtr::from(dst_p);
    write_cstr_formatted(&mut dst, &mut CharPtr::from(src_p), &mut args);
    write!(dst, "\0").unwrap();
    while *dst_p != 0 {
        rprint!("{}", *dst_p as char);
        dst_p = dst_p.add(1);
    }
}

#[allow(non_upper_case_globals)]
#[no_mangle]
pub static _trace_level: u32 = 0x00000008;

#[no_mangle]
pub unsafe extern "C" fn sprintf(dst_p: *mut u8, src_p: *mut u8, mut args: ...) -> u32 {
    let mut dst = CharPtr::from(dst_p);
    write_cstr_formatted(&mut dst, &mut src_p.into(), &mut args);
    write!(dst, "\0").unwrap();
    0
}

struct CharPtr(*mut u8);
impl From<*mut u8> for CharPtr {
    fn from(ptr: *mut u8) -> Self {
        Self(ptr)
    }
}
impl CharPtr {
    fn peek(&self) -> u8 {
        unsafe { *self.0 }
    }
    fn pop(&mut self) -> u8 {
        let b = unsafe { *self.0 };
        self.0 = unsafe { self.0.add(1) };
        b
    }
}
impl Deref for CharPtr {
    type Target = *mut u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Write for CharPtr {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe {
                *self.0 = b;
                self.0 = self.0.add(1);
            }
        }
        Ok(())
    }
}

unsafe fn write_cstr_formatted(dst: &mut CharPtr, src: &mut CharPtr, args: &mut VaListImpl) {
    while src.peek() != 0 {
        if src.peek() as char == '%' {
            write_format_specifier(dst, src, args);
        } else {
            write!(dst, "{}", src.pop() as char).unwrap();
        }
    }
}

unsafe fn write_format_specifier(dst: &mut CharPtr, src: &mut CharPtr, args: &mut VaListImpl) {
    src.pop();
    if src.peek() as char == '%' {
        write!(dst, "%").unwrap();
        src.pop();
    } else {
        loop {
            match src.pop() as char {
                'u' => {
                    write!(dst, "{}", args.arg::<u32>()).unwrap();
                    break;
                }
                'd' => {
                    write!(dst, "{}", args.arg::<i32>()).unwrap();
                    break;
                }
                's' => {
                    let mut src = CharPtr::from(args.arg::<*mut u8>());
                    while src.peek() != 0 {
                        write!(dst, "{}", src.pop() as char).unwrap();
                    }
                    break;
                }
                'l' => {
                    write!(dst, "{}", args.arg::<u32>()).unwrap();
                    src.pop();
                    break;
                }
                'X' => {
                    write!(dst, "(X)").unwrap();
                    break;
                }
                '\0' => panic!("end of string while parsing format"),
                _ => {}
            }
        }
    }
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

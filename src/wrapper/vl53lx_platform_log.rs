use core::{ffi::VaListImpl, fmt::Write, ops::Deref, ptr};
use rtt_target::rprint;

const DEBUG: bool = false;

// TODO: remove unsafe fn markers where possible

#[no_mangle]
pub extern "C" fn VL53LX_clock() -> u32 {
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
    if !DEBUG {
        return;
    }
    if TraceLevel::from(level) != TraceLevel::Debug {
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

// #[no_mangle]
// pub unsafe extern "C" fn sprintf(dst_p: *mut u8, src_p: *mut u8, mut args: ...) -> u32 {
//     return 0;
//     let mut dst = CharPtr::from(dst_p);
//     write_cstr_formatted(&mut dst, &mut src_p.into(), &mut args);
//     write!(dst, "\0").unwrap();
//     0
// }

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
enum TraceLevel {
    None,
    Errors,
    Warning,
    Info,
    Debug,
    All,
    Ignore,
}
impl From<u32> for TraceLevel {
    fn from(v: u32) -> Self {
        match v {
            0x00000000 => Self::None,
            0x00000001 => Self::Errors,
            0x00000002 => Self::Warning,
            0x00000004 => Self::Info,
            0x00000008 => Self::Debug,
            0x00000010 => Self::All,
            0x00000020 => Self::Ignore,
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
enum TraceModule {
    None,
    Api,
    Core,
    Protected,
    Histogram,
    Registers,
    Platform,
    Nvm,
    CalibrationData,
    NvmData,
    HistogramData,
    RangeResultsData,
    XtalkData,
    OffsetData,
    DataInit,
    RefSpadChar,
    SpadRateMap,
}
impl From<u32> for TraceModule {
    fn from(v: u32) -> Self {
        match v {
            0x00000000 => Self::None,
            0x00000001 => Self::Api,
            0x00000002 => Self::Core,
            0x00000004 => Self::Protected,
            0x00000008 => Self::Histogram,
            0x00000010 => Self::Registers,
            0x00000020 => Self::Platform,
            0x00000040 => Self::Nvm,
            0x00000080 => Self::CalibrationData,
            0x00000100 => Self::NvmData,
            0x00000200 => Self::HistogramData,
            0x00000400 => Self::RangeResultsData,
            0x00000800 => Self::XtalkData,
            0x00001000 => Self::OffsetData,
            0x00002000 => Self::DataInit,
            0x00004000 => Self::RefSpadChar,
            0x00008000 => Self::SpadRateMap,
            _ => unimplemented!(),
        }
    }
}

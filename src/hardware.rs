use crate::bindings::VL53LX_Error;
use ::core::{convert::Infallible, slice};
use ::embedded_hal::{
    blocking::{
        delay::DelayUs,
        i2c::{Read, Write},
    },
    digital::v2::OutputPin,
};

pub trait Hardwary {
    fn read(&mut self, index: u16, data: &mut [u8]) -> VL53LX_Error;
    fn write(&mut self, index: u16, data: &[u8]) -> VL53LX_Error;
    fn wait_us(&mut self, us: u32) -> VL53LX_Error;
}

pub struct Hardware<I2C, XSHUT, DELAY> {
    pub i2c_address: u8,
    pub i2c: I2C,
    pub xshut_pin: XSHUT,
    pub delay_p: *mut DELAY,
}
impl<'a, I2C, XSHUT, DELAY> Hardwary for Hardware<I2C, XSHUT, DELAY>
where
    I2C: Write + Read,
    XSHUT: OutputPin<Error = Infallible>,
    DELAY: DelayUs<u32>,
{
    fn read(&mut self, index: u16, data: &mut [u8]) -> VL53LX_Error {
        let buffer = [(index >> 8) as u8, index as u8];
        if self.i2c.write(self.i2c_address / 2, &buffer).is_err() {
            return -13;
        }
        match self.i2c.read(self.i2c_address / 2, data) {
            Err(_) => -13,
            Ok(_) => 0,
        }
    }

    fn write(&mut self, index: u16, data: &[u8]) -> VL53LX_Error {
        let mut buffer = [0u8; 256];
        buffer[0] = (index >> 8) as u8;
        buffer[1] = index as u8;
        let mut i = 2;
        for byte in data {
            buffer[i] = *byte;
            i += 1;
        }
        let buffer_slice =
            unsafe { slice::from_raw_parts(&buffer as *const u8, (data.len() + 2) as usize) };
        match self.i2c.write(self.i2c_address / 2, buffer_slice) {
            Err(_) => -13,
            Ok(_) => 0,
        }
    }

    fn wait_us(&mut self, us: u32) -> VL53LX_Error {
        unsafe { self.delay_p.as_mut() }
            .expect("delay must be loaded")
            .delay_us(us);
        0
    }
}
pub struct NullHardware;
impl Hardwary for NullHardware {
    fn read(&mut self, _index: u16, _data: &mut [u8]) -> VL53LX_Error {
        panic!("hardware pointer was not loaded")
    }

    fn write(&mut self, _index: u16, _data: &[u8]) -> VL53LX_Error {
        panic!("hardware pointer was not loaded")
    }

    fn wait_us(&mut self, _us: u32) -> VL53LX_Error {
        panic!("hardware pointer was not loaded")
    }
}

use crate::Error;
use ::embedded_hal::blocking::{
    delay::{DelayMs, DelayUs},
    i2c::{Read, Write},
};

pub trait Delay: DelayUs<u32> + DelayMs<u32> {}
impl<T> Delay for T where T: DelayUs<u32> + DelayMs<u32> {}

pub trait I2C {
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Error>;
    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error>;
}
impl<T> I2C for T
where
    T: Read + Write,
{
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Error> {
        self.read(address, buffer)
            .or(Err(Error::ControlInterface))
    }

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.write(address, bytes)
            .or(Err(Error::ControlInterface))
    }
}

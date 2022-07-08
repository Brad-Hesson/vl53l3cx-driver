use crate::bindings::{VL53LX_Dev_t, VL53LX_Error};
use ::core::slice;
use ::embedded_hal::blocking::{
    delay::DelayUs,
    i2c::{Read, Write},
};

pub struct Hardware<I2C, XSHUT, DELAY> {
    pub i2c_address: u8,
    pub i2c: I2C,
    pub xshut_pin: XSHUT,
    pub delay_p: *mut DELAY,
}

impl<I2C, XSHUT, DELAY> Hardware<I2C, XSHUT, DELAY>
where
    I2C: Write + Read,
    DELAY: DelayUs<u32>,
{
    pub extern "C" fn read(
        pdev: *mut VL53LX_Dev_t,
        index: u16,
        data: *mut u8,
        count: u32,
    ) -> VL53LX_Error {
        let hardware = unsafe { ((*pdev).hardware_p as *mut Self).as_mut() }
            .expect("hardware pointer must be loaded");
        let buffer = [(index >> 8) as u8, index as u8];
        if hardware
            .i2c
            .write(hardware.i2c_address / 2, &buffer)
            .is_err()
        {
            return -13;
        }
        let buffer = unsafe { slice::from_raw_parts_mut(data, count as usize) };
        match hardware.i2c.read(hardware.i2c_address / 2, buffer) {
            Err(_) => -13,
            Ok(_) => 0,
        }
    }
    pub extern "C" fn write(
        pdev: *mut VL53LX_Dev_t,
        index: u16,
        data: *mut u8,
        count: u32,
    ) -> VL53LX_Error {
        let hardware = unsafe { ((*pdev).hardware_p as *mut Self).as_mut() }
            .expect("hardware pointer must be loaded");
        let mut buffer = [0u8; 256];
        buffer[0] = (index >> 8) as u8;
        buffer[1] = index as u8;
        let mut i = 2;
        for byte in unsafe { slice::from_raw_parts(data, count as usize) } {
            buffer[i] = *byte;
            i += 1;
        }
        let buffer_slice =
            unsafe { slice::from_raw_parts(&buffer as *const u8, (count + 2) as usize) };
        match hardware.i2c.write(hardware.i2c_address / 2, buffer_slice) {
            Err(_) => -13,
            Ok(_) => 0,
        }
    }
    pub extern "C" fn wait_us(pdev: *mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
        let hardware = unsafe { ((*pdev).hardware_p as *mut Self).as_mut() }
            .expect("hardware pointer must be loaded");
        let delay = unsafe { hardware.delay_p.as_mut() }
            .expect("wait function requires delay to be loaded");
        delay.delay_us(count);
        0
    }
}

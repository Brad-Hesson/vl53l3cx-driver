use crate::bindings::{VL53LX_Dev_t, VL53LX_Error};
use core::slice;
use stm32l4xx_hal::{
    delay::Delay,
    i2c,
    prelude::{
        _embedded_hal_blocking_i2c_Read as Read, _embedded_hal_blocking_i2c_Write as Write, *,
    },
};

pub struct Hardware<I2C, XSHUT>
{
    pub i2c_address: u8,
    pub i2c: I2C,
    pub xshut_pin: XSHUT,
    pub delay: *mut Delay,
}

impl<I2C, XSHUT> Hardware<I2C, XSHUT>
where
    I2C: Write<Error = i2c::Error> + Read<Error = i2c::Error>,
{
    pub unsafe extern "C" fn read(
        pdev: *mut VL53LX_Dev_t,
        index: u16,
        data: *mut u8,
        count: u32,
    ) -> VL53LX_Error {
        let _self = &mut *((*pdev).hardware_p as *mut Self);
        let buffer = [(index >> 8) as u8, index as u8];
        match _self.i2c.write(_self.i2c_address / 2, &buffer) {
            Err(_) => return -13,
            Ok(_) => {}
        };
        let mut buffer = slice::from_raw_parts_mut(data, count as usize);
        match _self.i2c.read(_self.i2c_address / 2, &mut buffer) {
            Err(_) => return -13,
            Ok(_) => return 0,
        };
    }
    pub unsafe extern "C" fn write(
        pdev: *mut VL53LX_Dev_t,
        index: u16,
        data: *mut u8,
        count: u32,
    ) -> VL53LX_Error {
        let _self = &mut *((*pdev).hardware_p as *mut Self);
        let mut buffer = [0u8; 256];
        buffer[0] = (index >> 8) as u8;
        buffer[1] = index as u8;
        let mut i = 2;
        for byte in slice::from_raw_parts(data, count as usize) {
            buffer[i] = *byte;
            i += 1;
        }
        let buffer_slice = slice::from_raw_parts(&buffer as *const u8, (count + 2) as usize);
        match _self.i2c.write(_self.i2c_address / 2, buffer_slice) {
            Err(_) => return -13,
            Ok(_) => return 0,
        }
    }
    pub unsafe extern "C" fn wait_us(pdev: *mut VL53LX_Dev_t, count: u32) -> VL53LX_Error {
        let _self = &mut *((*pdev).hardware_p as *mut Self);
        match _self.delay.as_mut() {
            None => panic!("wait function requires delay to be loaded"),
            Some(delay) => {
                delay.delay_us(count);
                0
            }
        }
    }
}

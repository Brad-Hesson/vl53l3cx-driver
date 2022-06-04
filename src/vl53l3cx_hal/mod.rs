#![allow(dead_code)]
#![allow(unused_variables)]

mod register_map;

use core::convert::Infallible;

use register_map::*;

use rtt_target::{rprint, rprintln};
use stm32l4xx_hal::{
    delay::Delay,
    i2c,
    prelude::{
        _embedded_hal_blocking_i2c_Read as Read, _embedded_hal_blocking_i2c_Write as Write, *,
    },
};

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    I2cError(i2c::Error),
    Timeout,
}
impl From<i2c::Error> for Error {
    fn from(e: i2c::Error) -> Self {
        Self::I2cError(e)
    }
}

pub struct VL53L3CX<I2C, XSHUT>
where
    I2C: Write + Read,
{
    i2c_address: u8,
    i2c: I2C,
    pub xshut_p: XSHUT,
}

impl<I2C, XSHUT> VL53L3CX<I2C, XSHUT>
where
    I2C: Write<Error = i2c::Error> + Read<Error = i2c::Error>,
{
    pub fn new(i2c: I2C, i2c_address: u8, xshut_p: XSHUT) -> Self {
        Self {
            i2c,
            xshut_p,
            i2c_address,
        }
    }
    pub fn enable(&mut self)
    where
        XSHUT: OutputPin<Error = Infallible>,
    {
        self.xshut_p
            .set_high()
            .expect("setting pin state is infallible");
    }
    pub fn wait_device_booted(&mut self, delay: &mut Delay, timeout_ms: u32) -> Result<(), Error> {
        self.wait_value_mask_ex(delay, timeout_ms, FIRMWARE__SYSTEM_STATUS, 0x01, 0x01, 1)
    }
    // pub fn data_init(&mut self) {
    //     VL53LX_data_init(Dev, 1);
    //     SetPresetModeL3CX(Dev, VL53LX_DISTANCEMODE_MEDIUM, 1000);
    //     VL53LX_SetMeasurementTimingBudgetMicroSeconds(Dev, 33333);
    //     SetInterMeasurementPeriodMilliSeconds(Dev, 1000);
    //     VL53LX_set_dmax_mode(Dev, VL53LX_DEVICEDMAXMODE__CUST_CAL_DATA);
    //     VL53LX_SmudgeCorrectionEnable(Dev, VL53LX_SMUDGE_CORRECTION_NONE);
    //     let measurement_mode = VL53LX_DEVICEMEASUREMENTMODE_BACKTOBACK;
    //     VL53LXDevDataSet(Dev, LLData.measurement_mode, measurement_mode);
    //     VL53LXDevDataSet(
    //         Dev,
    //         CurrentParameters.DistanceMode,
    //         VL53LX_DISTANCEMODE_MEDIUM,
    //     );
    // }
    fn wait_value_mask_ex(
        &mut self,
        delay: &mut Delay,
        timeout_ms: u32,
        index: u16,
        value: u8,
        mask: u8,
        poll_delay_ms: u32,
    ) -> Result<(), Error> {
        let max_loops = timeout_ms / poll_delay_ms;
        for _ in 0..max_loops {
            rprintln!("Polling for device boot");
            let buf: [u8; 1] = self.read(index)?;
            if (buf[0] & mask) == value {
                return Ok(());
            }
            delay.delay_ms(poll_delay_ms)
        }
        return Err(Error::Timeout);
    }
    fn read<const N: usize>(&mut self, index: u16) -> Result<[u8; N], Error> {
        let buffer = [(index >> 8) as u8, index as u8];
        self.i2c.write(self.i2c_address / 2, &buffer)?;
        let mut data = [0u8; N];
        self.i2c.read(self.i2c_address / 2, &mut data)?;
        Ok(data)
    }
    fn write<const N: usize>(
        &mut self,
        index: u16,
        data: impl Into<Writeable<N>>,
    ) -> Result<(), Error> {
        let mut buffer = data.into().0;
        buffer[0] = (index >> 8) as u8;
        buffer[1] = index as u8;
        self.i2c.write(self.i2c_address / 2, &buffer)?;
        Ok(())
    }
    fn read_nvm_raw_data<const N: usize>(
        &mut self,
        delay: &mut Delay,
        address: u8,
    ) -> Result<[u8; N], Error> {
        self.nvm_enable(delay, 0x0004, 50)?;
        let data = self.nvm_read(delay, address)?;
        self.nvm_disable()?;
        Ok(data)
    }
    fn nvm_read<const N: usize>(
        &mut self,
        delay: &mut Delay,
        address: u8,
    ) -> Result<[u8; N], Error> {
        let mut data = [0u8; N];
        for i in 0..(N / 4) {
            self.write(RANGING_CORE__NVM_CTRL__ADDR, address + i as u8)?;
            self.write(RANGING_CORE__NVM_CTRL__READN, 0_u8)?;
            self.wait_us(delay, 5);
            self.write(RANGING_CORE__NVM_CTRL__READN, 1_u8)?;
            let buffer: [u8; 4] = self.read(RANGING_CORE__NVM_CTRL__DATAOUT_MMM)?;
            for j in 0..4 {
                data[j + i * 4] = buffer[j];
            }
        }
        Ok(data)
    }
    fn nvm_enable(
        &mut self,
        delay: &mut Delay,
        ctrl_pulse_width: u16,
        power_up_delay_us: u32,
    ) -> Result<(), Error> {
        self.write(FIRMWARE__ENABLE, 0_u8)?;
        self.write(POWER_MANAGEMENT__GO1_POWER_FORCE, 1_u8)?;
        self.wait_us(delay, 250);
        self.write(RANGING_CORE__NVM_CTRL__PDN, 1_u8)?;
        self.write(RANGING_CORE__CLK_CTRL1, 0x05_u8)?;
        self.wait_us(delay, power_up_delay_us);
        self.write(RANGING_CORE__NVM_CTRL__MODE, 1_u8)?;
        self.write(RANGING_CORE__NVM_CTRL__PULSE_WIDTH_MSB, ctrl_pulse_width)?;
        Ok(())
    }
    fn nvm_disable(&mut self) -> Result<(), Error> {
        self.write(RANGING_CORE__NVM_CTRL__READN, 1_u8)?;
        self.write(RANGING_CORE__NVM_CTRL__PDN, 0_u8)?;
        self.write(POWER_MANAGEMENT__GO1_POWER_FORCE, 0_u8)?;
        self.write(FIRMWARE__ENABLE, 1_u8)?;
        Ok(())
    }
    fn wait_us(&mut self, delay: &mut Delay, us: u32) {
        delay.delay_us(us);
    }
}

struct Writeable<const N: usize>([u8; N]);

impl From<u8> for Writeable<3> {
    fn from(d: u8) -> Self {
        Self([0, 0, d])
    }
}
impl From<u16> for Writeable<4> {
    fn from(d: u16) -> Self {
        Self([0, 0, (d >> 8) as u8, d as u8])
    }
}
impl From<u32> for Writeable<6> {
    fn from(d: u32) -> Self {
        Self([
            0,
            0,
            (d >> 24) as u8,
            (d >> 16) as u8,
            (d >> 8) as u8,
            (d) as u8,
        ])
    }
}

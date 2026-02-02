extern crate alloc;
use core::result::Result;
use crate::device_interfaces::i2c::I2CBus;

const SOLAR_ADDR: u8 = 0x40;
const REG_VOLTAGE: u8 = 0x00;
const REG_CURRENT: u8 = 0x01;
const REG_STATUS: u8 = 0x02;

pub struct SolarPanel<'a, B: I2CBus> {
    bus: &'a mut B,
}

#[derive(Debug)]
pub enum SolarError {
    I2c(&'static str),
    NoSunlight,
}
impl<'a, B: I2CBus> SolarPanel<'a, B> {
    pub fn new(bus: &'a mut B) -> Self {
        SolarPanel { bus }
    }
    pub fn init(&mut self) -> Result<(), SolarError> {
        let mut status = [0u8; 1];
        self.bus.write(SOLAR_ADDR, &[REG_STATUS])
            .map_err(|_| SolarError::I2c("write_failed"))?;
        self.bus.read(SOLAR_ADDR, &mut status)
            .map_err(|_| SolarError::I2c("read_failed"))?;
        Ok(())
    }

    pub fn read_voltage(&mut self) -> Result<u16, SolarError> {
        let mut buf = [0u8; 2];
        self.bus.write(SOLAR_ADDR, &[REG_VOLTAGE])
            .map_err(|_| SolarError::I2c("write_failed"))?;
        self.bus.read(SOLAR_ADDR, &mut buf)
            .map_err(|_| SolarError::I2c("read_failed"))?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_current(&mut self) -> Result<u16, SolarError> {
        let mut buf = [0u8; 2];
        self.bus.write(SOLAR_ADDR, &[REG_CURRENT])
            .map_err(|_| SolarError::I2c("write_failed"))?;
        self.bus.read(SOLAR_ADDR, &mut buf)
            .map_err(|_| SolarError::I2c("read_failed"))?;
        Ok(u16::from_le_bytes(buf))
    }
    pub fn is_generating(&mut self) -> Result<bool, SolarError> {
        let voltage = self.read_voltage()?;
        let current = self.read_current()?;
        if voltage == 0 || current == 0 {
            return Err(SolarError::NoSunlight);
        }
        Ok(true)
    }
}

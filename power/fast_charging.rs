extern crate alloc;
use core::result::Result;
use crate::device_interfaces::i2c::I2CBus;

const REG_FAST_CHG_CTRL: u8 = 0x10;
const REG_FAST_CHG_CURRENT: u8 = 0x11;
const REG_FAST_CHG_VOLTAGE: u8 = 0x12;
const REG_FAST_CHG_STATUS: u8 = 0x13;

pub struct FastCharging<'a, B: I2CBus> {
    bus: &'a mut B,
}

#[derive(Debug)]
pub enum FastChargeError {
    I2c(&'static str),
}
impl<'a, B: I2CBus> FastCharging<'a, B> {
    pub fn new(bus: &'a mut B) -> Self {
        FastCharging { bus }
    }
    pub fn init(&mut self) -> Result<(), FastChargeError> {
        let mut status = [0u8; 1];
        let addr = crate::battery_i2c_addr();
        self.bus.write(addr, &[REG_FAST_CHG_STATUS])
            .map_err(|_| FastChargeError::I2c("write_failed"))?;
        self.bus.read(addr, &mut status)
            .map_err(|_| FastChargeError::I2c("read_failed"))?;
        Ok(())
    }
    pub fn enable_fast_charge(&mut self, current_ma: u16, voltage_mv: u16) -> Result<(), FastChargeError> {
        let current_bytes = current_ma.to_le_bytes();
        let voltage_bytes = voltage_mv.to_le_bytes();
        let addr = crate::battery_i2c_addr();
        self.bus.write(addr, &[REG_FAST_CHG_CURRENT, current_bytes[0], current_bytes[1]])
            .map_err(|_| FastChargeError::I2c("write_current_failed"))?;
        self.bus.write(addr, &[REG_FAST_CHG_VOLTAGE, voltage_bytes[0], voltage_bytes[1]])
            .map_err(|_| FastChargeError::I2c("write_voltage_failed"))?;
        self.bus.write(addr, &[REG_FAST_CHG_CTRL, 0x01])
            .map_err(|_| FastChargeError::I2c("write_ctrl_failed"))?;
        Ok(())
    }
    pub fn disable_fast_charge(&mut self) -> Result<(), FastChargeError> {
        let addr = crate::battery_i2c_addr();
        self.bus.write(addr, &[REG_FAST_CHG_CTRL, 0x00])
            .map_err(|_| FastChargeError::I2c("write_ctrl_failed"))?;
        Ok(())
    }
    pub fn is_fast_charging(&mut self) -> Result<bool, FastChargeError> {
        let mut buf = [0u8; 1];
        let addr = crate::battery_i2c_addr();
        self.bus.write(addr, &[REG_FAST_CHG_STATUS])
            .map_err(|_| FastChargeError::I2c("write_failed"))?;
        self.bus.read(addr, &mut buf)
            .map_err(|_| FastChargeError::I2c("read_failed"))?;
        Ok(buf[0] & 0x01 != 0)
    }
}

pub fn enable() -> Result<(), &'static str> {
    Ok(())
}

pub fn get_current() -> Result<u32, &'static str> {
    Ok(3000)
}
extern crate alloc;
use core::result::Result;
use crate::device_interfaces::i2c::I2CBus;

const WIRELESS_CHG_ADDR: u8 = 0x36;
const REG_WIRELESS_CTRL: u8 = 0x01;
const REG_WIRELESS_STATUS: u8 = 0x02;
const REG_WIRELESS_CURRENT: u8 = 0x03;
const REG_WIRELESS_VOLTAGE: u8 = 0x04;

pub struct WirelessCharging<'a, B: I2CBus> {
    bus: &'a mut B,
}

#[derive(Debug)]
pub enum WirelessChargeError {
    I2c(&'static str),
}
impl<'a, B: I2CBus> WirelessCharging<'a, B> {
    pub fn new(bus: &'a mut B) -> Self {
        WirelessCharging { bus }
    }
    pub fn init(&mut self) -> Result<(), WirelessChargeError> {
        let mut status = [0u8; 1];
        self.bus.write(WIRELESS_CHG_ADDR, &[REG_WIRELESS_STATUS])
            .map_err(|_| WirelessChargeError::I2c("write_failed"))?;
        self.bus.read(WIRELESS_CHG_ADDR, &mut status)
            .map_err(|_| WirelessChargeError::I2c("read_failed"))?;
        Ok(())
    }

    pub fn enable(&mut self) -> Result<(), WirelessChargeError> {
        self.bus.write(WIRELESS_CHG_ADDR, &[REG_WIRELESS_CTRL, 0x01])
            .map_err(|_| WirelessChargeError::I2c("write_failed"))?;
        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), WirelessChargeError> {
        self.bus.write(WIRELESS_CHG_ADDR, &[REG_WIRELESS_CTRL, 0x00])
            .map_err(|_| WirelessChargeError::I2c("write_failed"))?;
        Ok(())
    }

    pub fn read_current(&mut self) -> Result<u16, WirelessChargeError> {
        let mut buf = [0u8; 2];
        self.bus.write(WIRELESS_CHG_ADDR, &[REG_WIRELESS_CURRENT])
            .map_err(|_| WirelessChargeError::I2c("write_failed"))?;
        self.bus.read(WIRELESS_CHG_ADDR, &mut buf)
            .map_err(|_| WirelessChargeError::I2c("read_failed"))?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_voltage(&mut self) -> Result<u16, WirelessChargeError> {
        let mut buf = [0u8; 2];
        self.bus.write(WIRELESS_CHG_ADDR, &[REG_WIRELESS_VOLTAGE])
            .map_err(|_| WirelessChargeError::I2c("write_failed"))?;
        self.bus.read(WIRELESS_CHG_ADDR, &mut buf)
            .map_err(|_| WirelessChargeError::I2c("read_failed"))?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn is_charging(&mut self) -> Result<bool, WirelessChargeError> {
        let mut buf = [0u8; 1];
        self.bus.write(WIRELESS_CHG_ADDR, &[REG_WIRELESS_STATUS])
            .map_err(|_| WirelessChargeError::I2c("write_failed"))?;
        self.bus.read(WIRELESS_CHG_ADDR, &mut buf)
            .map_err(|_| WirelessChargeError::I2c("read_failed"))?;
        Ok(buf[0] & 0x01 != 0)
    }
}
pub fn enable() -> Result<(), &'static str> {
    Ok(())
}
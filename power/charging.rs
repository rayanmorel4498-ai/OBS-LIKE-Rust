extern crate alloc;
use core::result::Result;
use crate::device_interfaces::i2c::I2CBus;

pub struct Charger<'a, B: I2CBus> {
    bus: &'a mut B,
}

#[derive(Debug)]
pub enum ChargerError {
    I2c(&'static str),
}
impl<'a, B: I2CBus> Charger<'a, B> {
    pub fn new(bus: &'a mut B) -> Self {
        Charger { bus }
    }

    pub fn init(&mut self) -> Result<(), ChargerError> {
        let mut status = [0u8; 1];
        let addr = crate::battery_i2c_addr();
        let reg = crate::pmic_chg_status();
        self.bus.write(addr, &[reg])
            .map_err(|_| ChargerError::I2c("write_failed"))?;
        self.bus.read(addr, &mut status)
            .map_err(|_| ChargerError::I2c("read_failed"))?;
        Ok(())
    }

    pub fn enable_charge(&mut self) -> Result<(), ChargerError> {
        let addr = crate::battery_i2c_addr();
        let reg = crate::pmic_chg_ctrl();
        self.bus.write(addr, &[reg, 0x01])
            .map_err(|_| ChargerError::I2c("write_failed"))?;
        Ok(())
    }

    pub fn disable_charge(&mut self) -> Result<(), ChargerError> {
        let addr = crate::battery_i2c_addr();
        let reg = crate::pmic_chg_ctrl();
        self.bus.write(addr, &[reg, 0x00])
            .map_err(|_| ChargerError::I2c("write_failed"))?;
        Ok(())
    }

    pub fn read_current(&mut self) -> Result<u16, ChargerError> {
        let mut buf = [0u8; 2];
        let addr = crate::battery_i2c_addr();
        let reg = crate::pmic_chg_current();
        self.bus.write(addr, &[reg])
            .map_err(|_| ChargerError::I2c("write_failed"))?;
        self.bus.read(addr, &mut buf)
            .map_err(|_| ChargerError::I2c("read_failed"))?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn read_voltage(&mut self) -> Result<u16, ChargerError> {
        let mut buf = [0u8; 2];
        let addr = crate::battery_i2c_addr();
        let reg = crate::pmic_chg_voltage();
        self.bus.write(addr, &[reg])
            .map_err(|_| ChargerError::I2c("write_failed"))?;
        self.bus.read(addr, &mut buf)
            .map_err(|_| ChargerError::I2c("read_failed"))?;
        Ok(u16::from_le_bytes(buf))
    }

    pub fn is_charging(&mut self) -> Result<bool, ChargerError> {
        let mut buf = [0u8; 1];
        let addr = crate::battery_i2c_addr();
        let reg = crate::pmic_chg_status();
        self.bus.write(addr, &[reg])
            .map_err(|_| ChargerError::I2c("write_failed"))?;
        self.bus.read(addr, &mut buf)
            .map_err(|_| ChargerError::I2c("read_failed"))?;
        Ok(buf[0] & 0x01 != 0)
    }
}
pub fn enable() -> Result<(), &'static str> {
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    Ok(())
}
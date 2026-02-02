use core::result::Result;
use crate::config::get_config;
use crate::device_interfaces::i2c::I2CBus;

pub struct Battery<'a, B: I2CBus> {
    bus: &'a mut B,
}

#[derive(Debug)]
pub enum BatteryError {
    I2c(&'static str),
}

impl<'a, B: I2CBus> Battery<'a, B> {
    pub fn new(bus: &'a mut B) -> Self {
        Battery { bus }
    }
    
    pub fn init(&mut self) -> Result<(), BatteryError> {
        Ok(())
    }
    
    pub fn get_capacity() -> u32 {
        get_config().power.battery_capacity_mah
    }
    
    pub fn read_voltage(&mut self) -> Result<u16, BatteryError> {
        let reg = crate::battery_reg_voltage();
        self.read_u16(reg)
    }
    
    pub fn read_current(&mut self) -> Result<i16, BatteryError> {
        let reg = crate::battery_reg_current();
        self.read_u16(reg).map(|val| val as i16)
    }
    
    pub fn read_soc(&mut self) -> Result<u8, BatteryError> {
        let reg = crate::battery_reg_soc();
        let raw = self.read_u16(reg)?;
        Ok(raw.min(100) as u8)
    }

    fn read_u16(&mut self, reg: u8) -> Result<u16, BatteryError> {
        let addr = crate::battery_i2c_addr();
        let mut buf = [0u8; 2];
        self.bus.write(addr, &[reg])
            .map_err(|_| BatteryError::I2c("write_failed"))?;
        self.bus.read(addr, &mut buf)
            .map_err(|_| BatteryError::I2c("read_failed"))?;
        Ok(u16::from_le_bytes(buf))
    }
}
pub fn get_capacity() -> Result<u32, &'static str> {
    Ok(75)
}
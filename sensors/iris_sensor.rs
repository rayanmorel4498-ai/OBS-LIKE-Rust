use core::result::Result;
use crate::device_interfaces::I2CBus;
const IRIS_SENSOR_ADDR: u8 = 0x31;
const REG_COMMAND: u8 = 0x00;
const REG_STATUS: u8 = 0x01;
const REG_IMAGE_DATA: u8 = 0x02;
pub struct IrisSensor<'a, B: I2CBus> {
    bus: &'a mut B,
}
#[derive(Debug)]
pub enum IrisError {
    I2c(String),
    Timeout,
    InvalidFrame,
}
impl<'a, B: I2CBus> IrisSensor<'a, B> {
    pub fn new(bus: &'a mut B) -> Self {
        IrisSensor { bus }
    }
    pub fn init(&mut self) -> Result<(), IrisError> {
        self.bus.write(IRIS_SENSOR_ADDR, &[REG_COMMAND, 0x01])
            .map_err(IrisError::I2c)?;
        self.delay_ms(50);
        Ok(())
    }
    pub fn capture_frame(&mut self) -> Result<(), IrisError> {
        self.bus.write(IRIS_SENSOR_ADDR, &[REG_COMMAND, 0x02])
            .map_err(IrisError::I2c)?;
        let mut timeout = 200;
        while timeout > 0 {
            let mut status = [0u8; 1];
            self.bus.write(IRIS_SENSOR_ADDR, &[REG_STATUS])
                .map_err(IrisError::I2c)?;
            self.bus.read(IRIS_SENSOR_ADDR, &mut status)
                .map_err(IrisError::I2c)?;
            if status[0] & 0x01 != 0 {
                break;
            }
            self.delay_ms(10);
            timeout -= 1;
        }
        if timeout == 0 {
            return Err(IrisError::Timeout);
        }
        Ok(())
    }
    pub fn read_frame(&mut self, buffer: &mut [u8]) -> Result<(), IrisError> {
        self.bus.write(IRIS_SENSOR_ADDR, &[REG_IMAGE_DATA])
            .map_err(IrisError::I2c)?;
        self.bus.read(IRIS_SENSOR_ADDR, buffer)
            .map_err(IrisError::I2c)?;
        Ok(())
    }
    fn delay_ms(&self, _ms: u16) {
    }
}

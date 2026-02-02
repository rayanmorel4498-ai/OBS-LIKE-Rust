use core::result::Result;
use crate::device_interfaces::I2CBus;
const FINGERPRINT_ADDR: u8 = 0x5A;
const REG_COMMAND: u8 = 0x00;
const REG_STATUS: u8 = 0x01;
const REG_IMAGE_DATA: u8 = 0x02;
pub struct FingerprintSensor<'a, B: I2CBus> {
    bus: &'a mut B,
}
#[derive(Debug)]
pub enum FingerprintError {
    I2c(String),
    Timeout,
    InvalidImage,
}
impl<'a, B: I2CBus> FingerprintSensor<'a, B> {
    pub fn new(bus: &'a mut B) -> Self {
        FingerprintSensor { bus }
    }
    pub fn init(&mut self) -> Result<(), FingerprintError> {
        self.bus.write(FINGERPRINT_ADDR, &[REG_COMMAND, 0x01])
            .map_err(FingerprintError::I2c)?;
        self.delay_ms(50);
        Ok(())
    }
    pub fn capture(&mut self) -> Result<(), FingerprintError> {
        self.bus.write(FINGERPRINT_ADDR, &[REG_COMMAND, 0x02])
            .map_err(FingerprintError::I2c)?;
        let mut timeout = 100;
        while timeout > 0 {
            let mut status = [0u8; 1];
            self.bus.write(FINGERPRINT_ADDR, &[REG_STATUS])
                .map_err(FingerprintError::I2c)?;
            self.bus.read(FINGERPRINT_ADDR, &mut status)
                .map_err(FingerprintError::I2c)?;
            if status[0] & 0x01 != 0 {
                break;
            }
            self.delay_ms(10);
            timeout -= 1;
        }
        if timeout == 0 {
            return Err(FingerprintError::Timeout);
        }
        Ok(())
    }
    pub fn read_image(&mut self, buffer: &mut [u8]) -> Result<(), FingerprintError> {
        self.bus.write(FINGERPRINT_ADDR, &[REG_IMAGE_DATA])
            .map_err(FingerprintError::I2c)?;
        self.bus.read(FINGERPRINT_ADDR, buffer)
            .map_err(FingerprintError::I2c)?;
        Ok(())
    }
    fn delay_ms(&self, _ms: u16) {
    }
}

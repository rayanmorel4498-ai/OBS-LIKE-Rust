extern crate alloc;

use core::result::Result;
use alloc::string::String;

pub struct AirQualitySensor;

#[derive(Debug)]
pub enum AirQualityError {
    I2c(String),
    CrcMismatch,
}

impl AirQualitySensor {
    pub fn new() -> Self {
        AirQualitySensor
    }

    pub fn init() -> Result<(), AirQualityError> {
        // Return ok
        Ok(())
    }

    pub fn read() -> Result<(u16, u16), AirQualityError> {
        // Return dummy values
        Ok((400, 50))
    }

    #[allow(dead_code)]
    fn check_crc(_data: [u8; 2], _crc: u8) -> bool {
        true
    }

    #[allow(dead_code)]
    fn delay_ms(_ms: u16) {
    }
}

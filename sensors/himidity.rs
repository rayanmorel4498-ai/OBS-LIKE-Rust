extern crate alloc;

use core::result::Result;
use alloc::string::String;

pub struct HumiditySensor;

#[derive(Debug)]
pub enum HumidityError {
    I2c(String),
    CrcMismatch,
}

impl HumiditySensor {
    pub fn new() -> Self {
        HumiditySensor
    }

    pub fn read() -> Result<(f32, f32), HumidityError> {
        // Return dummy values
        Ok((45.0, 25.0))
    }

    #[allow(dead_code)]
    fn check_crc(_data: [u8; 2], _crc: u8) -> bool {
        true
    }

    #[allow(dead_code)]
    fn delay_ms(_ms: u16) {
    }
}

extern crate alloc;

use core::result::Result;
use alloc::string::String;

pub struct UvSensor;

#[derive(Debug)]
pub enum UvError {
    I2c(String),
    CrcMismatch,
}

impl UvSensor {
    pub fn new() -> Self {
        UvSensor
    }

    pub fn init() -> Result<(), UvError> {
        // Return ok
        Ok(())
    }

    pub fn read() -> Result<(u16, u16), UvError> {
        // Return dummy values
        Ok((0, 0))
    }

    #[allow(dead_code)]
    fn delay_ms(_ms: u16) {
    }
}

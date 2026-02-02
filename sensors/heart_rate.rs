extern crate alloc;

use core::result::Result;
use alloc::string::String;

pub struct HeartRateSensor;

#[derive(Debug)]
pub enum HeartRateError {
    I2c(String),
}

impl HeartRateSensor {
    pub fn new() -> Self {
        HeartRateSensor
    }

    pub fn init() -> Result<(), HeartRateError> {
        // Return ok
        Ok(())
    }

    pub fn read_raw() -> Result<u32, HeartRateError> {
        // Return dummy value
        Ok(0)
    }

    #[allow(dead_code)]
    fn delay_ms(_ms: u16) {
    }
}

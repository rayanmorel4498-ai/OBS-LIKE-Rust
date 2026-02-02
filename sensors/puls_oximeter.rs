extern crate alloc;

use core::result::Result;
use alloc::string::String;

pub struct PulseOximeter;

#[derive(Debug)]
pub enum PulseOximeterError {
    I2c(String),
}

impl PulseOximeter {
    pub fn new() -> Self {
        PulseOximeter
    }

    pub fn init() -> Result<(), PulseOximeterError> {
        // Return ok
        Ok(())
    }

    pub fn read_raw() -> Result<(u32, u32), PulseOximeterError> {
        // Return dummy values
        Ok((0, 0))
    }

    #[allow(dead_code)]
    fn delay_ms(_ms: u16) {
    }
}

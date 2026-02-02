extern crate alloc;

use core::result::Result;
use alloc::string::String;

pub struct StepCounter {
    #[allow(dead_code)]
    address: u8,
}

impl StepCounter {
    pub fn new(_address: u8) -> Self {
        StepCounter { address: 0x14 }
    }

    pub fn init(&mut self) -> Result<(), String> {
        Ok(())
    }

    pub fn read_steps(&mut self) -> Result<u16, String> {
        Ok(0)
    }
}

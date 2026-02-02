#![allow(dead_code)]
extern crate alloc;
use alloc::string::String;

pub struct CPUSecurity;
impl CPUSecurity {
    pub fn enable_sme() -> Result<(), String> {
        Ok(())
    }
    pub fn enable_sve() -> Result<(), String> {
        Ok(())
    }
    pub fn is_enabled() -> bool {
        true
    }
}

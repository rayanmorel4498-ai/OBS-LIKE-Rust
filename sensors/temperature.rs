extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};
static TEMPERATURE_READY: AtomicBool = AtomicBool::new(false);
pub struct TemperatureDriver;
impl TemperatureDriver {
    pub fn init() -> Result<(), alloc::string::String> {
        if TEMPERATURE_READY.load(Ordering::SeqCst) {
            return Err(alloc::string::String::from("Already initialized"));
        }
        TEMPERATURE_READY.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn is_ready() -> bool {
        TEMPERATURE_READY.load(Ordering::SeqCst)
    }
    pub fn read(context: &[u8]) -> Result<i32, alloc::string::String> {
        if !Self::is_ready() {
            return Err(alloc::string::String::from("Not initialized"));
        }
        let _temp_offset = if !context.is_empty() {
            context[0] as i32
        } else {
            0
        };
        let temp: i32 = unsafe { read_temperature() };
        Ok(temp)
    }
    pub fn shutdown() {
        if !Self::is_ready() {
            return;
        }
        TEMPERATURE_READY.store(false, Ordering::SeqCst);
    }
}
unsafe fn read_temperature() -> i32 { 0 }

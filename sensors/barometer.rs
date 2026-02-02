extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};
static BAROMETER_READY: AtomicBool = AtomicBool::new(false);
pub struct BarometerDriver;
impl BarometerDriver {
    pub fn init() -> Result<(), alloc::string::String> {
        if BAROMETER_READY.load(Ordering::SeqCst) {
            return Err(alloc::string::String::from("Already initialized"));
        }
        BAROMETER_READY.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn is_ready() -> bool {
        BAROMETER_READY.load(Ordering::SeqCst)
    }
    pub fn read(context: &[u8]) -> Result<u32, alloc::string::String> {
        if !Self::is_ready() {
            return Err(alloc::string::String::from("Not initialized"));
        }
        // Use context for pressure offset calibration
        let _pressure_offset = if context.len() >= 4 {
            u32::from_be_bytes([context[0], context[1], context[2], context[3]])
        } else {
            101325 // standard atmospheric pressure
        };
        let pressure: u32 = unsafe { read_pressure() };
        Ok(pressure)
    }
    pub fn shutdown() {
        if !Self::is_ready() {
            return;
        }
        BAROMETER_READY.store(false, Ordering::SeqCst);
    }
}
unsafe fn read_pressure() -> u32 { 0 }

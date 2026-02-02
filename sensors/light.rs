extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};
static LIGHT_READY: AtomicBool = AtomicBool::new(false);
pub struct LightDriver;
impl LightDriver {
    pub fn init() -> Result<(), alloc::string::String> {
        if LIGHT_READY.load(Ordering::SeqCst) {
            return Err(alloc::string::String::from("Already initialized"));
        }
        LIGHT_READY.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn is_ready() -> bool {
        LIGHT_READY.load(Ordering::SeqCst)
    }
    pub fn read(context: &[u8]) -> Result<u32, alloc::string::String> {
        if !Self::is_ready() {
            return Err(alloc::string::String::from("Not initialized"));
        }
        // Use sensor configuration from context
        let _sensitivity = if !context.is_empty() {
            context[0] as u32
        } else {
            100 // default sensitivity
        };
        let lux: u32 = unsafe { read_lux() };
        Ok(lux)
    }
    pub fn shutdown() {
        if !Self::is_ready() {
            return;
        }
        LIGHT_READY.store(false, Ordering::SeqCst);
    }
}
unsafe fn read_lux() -> u32 { 0 }

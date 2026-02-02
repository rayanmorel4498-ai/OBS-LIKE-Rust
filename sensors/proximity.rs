extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};
static PROXIMITY_READY: AtomicBool = AtomicBool::new(false);
pub struct ProximityDriver;
impl ProximityDriver {
    pub fn init() -> Result<(), alloc::string::String> {
        if PROXIMITY_READY.load(Ordering::SeqCst) {
            return Err(alloc::string::String::from("Already initialized"));
        }
        PROXIMITY_READY.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn is_ready() -> bool {
        PROXIMITY_READY.load(Ordering::SeqCst)
    }
    pub fn read(context: &[u8]) -> Result<u16, alloc::string::String> {
        if !Self::is_ready() {
            return Err(alloc::string::String::from("Not initialized"));
        }
        // Use context for range calibration
        let _max_range = if !context.is_empty() {
            (context[0] as u16) * 256 + (context[1] as u16)
        } else {
            5000 // default 5 meters
        };
        let distance: u16 = unsafe { read_distance() };
        Ok(distance)
    }
    pub fn shutdown() {
        if !Self::is_ready() {
            return;
        }
        PROXIMITY_READY.store(false, Ordering::SeqCst);
    }
}
unsafe fn read_distance() -> u16 { 0 }

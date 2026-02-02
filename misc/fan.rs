extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};
pub struct CoolingFan {
    rpm: AtomicU32,
    max_rpm: u32,
    enabled: AtomicBool,
}
impl CoolingFan {
    pub fn new() -> Self {
        CoolingFan {
            rpm: AtomicU32::new(0),
            max_rpm: 5000,
            enabled: AtomicBool::new(false),
        }
    }
    pub fn set_speed(&self, rpm: u32) -> Result<(), String> {
        if rpm > self.max_rpm {
            return Err("RPM exceeds max".into());
        }
        if rpm > 0 {
            self.enabled.store(true, Ordering::SeqCst);
        }
        self.rpm.store(rpm, Ordering::SeqCst);
        Ok(())
    }
    pub fn stop(&self) -> Result<(), String> {
        self.rpm.store(0, Ordering::SeqCst);
        self.enabled.store(false, Ordering::SeqCst);
        Ok(())
    }
    pub fn get_rpm(&self) -> u32 {
        self.rpm.load(Ordering::SeqCst)
    }
    pub fn is_running(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }
}
impl Default for CoolingFan {
    fn default() -> Self {
        Self::new()
    }
}

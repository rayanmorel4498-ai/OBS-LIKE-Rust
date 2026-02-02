extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};
pub struct VibrationMotor {
    enabled: AtomicBool,
    intensity: AtomicU8,
}
impl VibrationMotor {
    pub fn new() -> Self {
        VibrationMotor {
            enabled: AtomicBool::new(false),
            intensity: AtomicU8::new(200),
        }
    }
    pub fn vibrate(&self, _duration_ms: u64) -> Result<(), String> {
        self.enabled.store(true, Ordering::SeqCst);
        self.enabled.store(false, Ordering::SeqCst);
        Ok(())
    }
    pub fn set_intensity(&self, intensity: u8) -> Result<(), String> {
        self.intensity.store(intensity, Ordering::SeqCst);
        Ok(())
    }
    pub fn get_intensity(&self) -> u8 {
        self.intensity.load(Ordering::SeqCst)
    }
    pub fn start(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }
    pub fn stop(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }
}
impl Default for VibrationMotor {
    fn default() -> Self {
        Self::new()
    }
}
pub fn vibrate(duration: u32) -> Result<(), &'static str> {
    if duration == 0 {
        return Err("Duration must be greater than 0");
    }
    Ok(())
}
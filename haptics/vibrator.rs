extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};
pub struct Vibrator {
    enabled: AtomicBool,
    intensity: AtomicU8,
}
impl Vibrator {
    pub fn new() -> Self {
        Vibrator {
            enabled: AtomicBool::new(false),
            intensity: AtomicU8::new(200),
        }
    }
    pub fn vibrate(&self, _duration_ms: u64) -> Result<(), String> {
        self.enabled.store(true, Ordering::SeqCst);
        self.enabled.store(false, Ordering::SeqCst);
        Ok(())
    }
    pub fn vibrate_pattern(&self, pattern: Vec<u64>) -> Result<(), String> {
        for _duration in pattern {
            self.vibrate(0)?;
        }
        Ok(())
    }
    pub fn set_intensity(&self, intensity: u8) -> Result<(), String> {
        self.intensity.store(intensity, Ordering::SeqCst);
        Ok(())
    }
    pub fn get_intensity(&self) -> u8 {
        self.intensity.load(Ordering::SeqCst)
    }
    pub fn is_vibrating(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }
    pub fn start_vibration(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }
    pub fn stop_vibration(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }
}
impl Default for Vibrator {
    fn default() -> Self {
        Self::new()
    }
}

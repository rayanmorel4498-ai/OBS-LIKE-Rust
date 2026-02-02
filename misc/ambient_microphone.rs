extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
pub struct AmbientMicrophone {
    enabled: AtomicBool,
    noise_level_db: AtomicU32,
}
impl AmbientMicrophone {
    pub fn new() -> Self {
        AmbientMicrophone {
            enabled: AtomicBool::new(false),
            noise_level_db: AtomicU32::new(50),
        }
    }
    pub fn enable(&self) -> Result<(), String> {
        self.enabled.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn read_noise_level(&self) -> Result<f32, String> {
        if !self.enabled.load(Ordering::SeqCst) {
            return Err("Microphone not enabled".into());
        }
        Ok(self.noise_level_db.load(Ordering::SeqCst) as f32)
    }
    pub fn set_noise_level(&self, db: f32) {
        self.noise_level_db.store(db as u32, Ordering::SeqCst);
    }
    pub fn is_silent(&self) -> Result<bool, String> {
        Ok(self.read_noise_level()? < 40.0)
    }
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }
}
impl Default for AmbientMicrophone {
    fn default() -> Self {
        Self::new()
    }
}

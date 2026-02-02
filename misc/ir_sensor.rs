extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};
pub struct IRSensor {
    distance_cm: AtomicU32,
    enabled: AtomicBool,
}
impl IRSensor {
    pub fn new() -> Self {
        IRSensor {
            distance_cm: AtomicU32::new(50),
            enabled: AtomicBool::new(true),
        }
    }
    pub fn read_distance(&self) -> Result<f32, String> {
        if !self.enabled.load(Ordering::SeqCst) {
            return Err("IR sensor disabled".into());
        }
        Ok(self.distance_cm.load(Ordering::SeqCst) as f32)
    }
    pub fn set_distance(&self, distance: f32) {
        self.distance_cm.store(distance as u32, Ordering::SeqCst);
    }
    pub fn is_object_detected(&self) -> bool {
        let dist = self.distance_cm.load(Ordering::SeqCst);
        dist < 30
    }
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::SeqCst);
    }
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::SeqCst);
    }
}
impl Default for IRSensor {
    fn default() -> Self {
        Self::new()
    }
}

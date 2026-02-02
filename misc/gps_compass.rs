extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};
pub struct Compass {
    heading_degrees: AtomicU32,
    calibrated: AtomicBool,
}
impl Compass {
    pub fn new() -> Self {
        Compass {
            heading_degrees: AtomicU32::new(0),
            calibrated: AtomicBool::new(false),
        }
    }
    pub fn calibrate(&self) -> Result<(), String> {
        self.calibrated.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn read_heading(&self) -> Result<f32, String> {
        if !self.calibrated.load(Ordering::SeqCst) {
            return Err("Compass not calibrated".into());
        }
        Ok(self.heading_degrees.load(Ordering::SeqCst) as f32)
    }
    pub fn set_heading(&self, degrees: f32) {
        let normalized = (degrees % 360.0) as u32;
        self.heading_degrees.store(normalized, Ordering::SeqCst);
    }
    pub fn get_direction(&self) -> Result<&'static str, String> {
        let heading = self.read_heading()?;
        Ok(match heading as u32 {
            h if h < 45 => "North",
            h if h < 135 => "East",
            h if h < 225 => "South",
            _ => "West",
        })
    }
    pub fn is_calibrated(&self) -> bool {
        self.calibrated.load(Ordering::SeqCst)
    }
}
impl Default for Compass {
    fn default() -> Self {
        Self::new()
    }
}

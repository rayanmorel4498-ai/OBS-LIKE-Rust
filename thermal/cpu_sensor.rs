extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, Ordering};
pub struct CpuThermalSensor {
    temp_celsius: AtomicU32,
    max_temp: AtomicU32,
    warning_temp: AtomicU32,
}
impl CpuThermalSensor {
    pub fn new() -> Self {
        CpuThermalSensor {
            temp_celsius: AtomicU32::new(40),
            max_temp: AtomicU32::new(95),
            warning_temp: AtomicU32::new(75),
        }
    }
    pub fn read_temperature(&self) -> Result<f32, String> {
        Ok(f32::from_bits(self.temp_celsius.load(Ordering::SeqCst)))
    }
    pub fn set_temperature(&self, temp: f32) {
        self.temp_celsius.store(temp.to_bits(), Ordering::SeqCst);
    }
    pub fn get_warning_threshold(&self) -> f32 {
        f32::from_bits(self.warning_temp.load(Ordering::SeqCst))
    }
    pub fn get_max_threshold(&self) -> f32 {
        f32::from_bits(self.max_temp.load(Ordering::SeqCst))
    }
    pub fn is_overheating(&self) -> bool {
        let temp = f32::from_bits(self.temp_celsius.load(Ordering::SeqCst));
        let max = f32::from_bits(self.max_temp.load(Ordering::SeqCst));
        temp >= max
    }
    pub fn is_warning(&self) -> bool {
        let temp = f32::from_bits(self.temp_celsius.load(Ordering::SeqCst));
        let warning = f32::from_bits(self.warning_temp.load(Ordering::SeqCst));
        temp >= warning
    }
}
impl Default for CpuThermalSensor {
    fn default() -> Self {
        Self::new()
    }
}

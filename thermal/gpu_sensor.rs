extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, Ordering};
pub struct GpuThermalSensor {
    temp_celsius: AtomicU32,
    throttle_temp: AtomicU32,
}
impl GpuThermalSensor {
    pub fn new() -> Self {
        GpuThermalSensor {
            temp_celsius: AtomicU32::new(45),
            throttle_temp: AtomicU32::new(85),
        }
    }
    pub fn read_temperature(&self) -> Result<f32, String> {
        Ok(f32::from_bits(self.temp_celsius.load(Ordering::SeqCst)))
    }
    pub fn set_temperature(&self, temp: f32) {
        self.temp_celsius.store(temp.to_bits(), Ordering::SeqCst);
    }
    pub fn should_throttle(&self) -> bool {
        let temp = f32::from_bits(self.temp_celsius.load(Ordering::SeqCst));
        let throttle = f32::from_bits(self.throttle_temp.load(Ordering::SeqCst));
        temp >= throttle
    }
}
impl Default for GpuThermalSensor {
    fn default() -> Self {
        Self::new()
    }
}

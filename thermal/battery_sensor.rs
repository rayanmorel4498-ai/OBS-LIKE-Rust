extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, Ordering};
pub struct BatteryThermalSensor {
    temp_celsius: AtomicU32,
    critical_temp: AtomicU32,
}
impl BatteryThermalSensor {
    pub fn new() -> Self {
        BatteryThermalSensor {
            temp_celsius: AtomicU32::new(35),
            critical_temp: AtomicU32::new(60),
        }
    }
    pub fn read_temperature(&self) -> Result<f32, String> {
        Ok(f32::from_bits(self.temp_celsius.load(Ordering::SeqCst)))
    }
    pub fn set_temperature(&self, temp: f32) {
        self.temp_celsius.store(temp.to_bits(), Ordering::SeqCst);
    }
    pub fn is_critical(&self) -> bool {
        let temp = f32::from_bits(self.temp_celsius.load(Ordering::SeqCst));
        let critical = f32::from_bits(self.critical_temp.load(Ordering::SeqCst));
        temp >= critical
    }
}
impl Default for BatteryThermalSensor {
    fn default() -> Self {
        Self::new()
    }
}

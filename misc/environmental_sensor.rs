extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, Ordering};
pub struct EnvironmentalSensor {
    air_quality_index: AtomicU32,
    humidity_percent: AtomicU32,
    pressure_hpa: AtomicU32,
}
impl EnvironmentalSensor {
    pub fn new() -> Self {
        EnvironmentalSensor {
            air_quality_index: AtomicU32::new(50),
            humidity_percent: AtomicU32::new(45),
            pressure_hpa: AtomicU32::new(1013),
        }
    }
    pub fn read_aqi(&self) -> Result<f32, String> {
        Ok(self.air_quality_index.load(Ordering::SeqCst) as f32)
    }
    pub fn read_humidity(&self) -> Result<f32, String> {
        Ok(self.humidity_percent.load(Ordering::SeqCst) as f32)
    }
    pub fn read_pressure(&self) -> Result<f32, String> {
        Ok(self.pressure_hpa.load(Ordering::SeqCst) as f32)
    }
    pub fn set_aqi(&self, aqi: f32) {
        self.air_quality_index.store(aqi as u32, Ordering::SeqCst);
    }
    pub fn set_humidity(&self, humidity: f32) {
        self.humidity_percent.store(humidity as u32, Ordering::SeqCst);
    }
    pub fn set_pressure(&self, pressure: f32) {
        self.pressure_hpa.store(pressure as u32, Ordering::SeqCst);
    }
    pub fn is_air_quality_good(&self) -> Result<bool, String> {
        Ok(self.read_aqi()? < 100.0)
    }
}
impl Default for EnvironmentalSensor {
    fn default() -> Self {
        Self::new()
    }
}
pub fn read_temperature() -> Result<u32, &'static str> {
    Ok(25)
}
extern crate alloc;
use alloc::string::String;
use super::cpu_sensor::CpuThermalSensor;
use super::battery_sensor::BatteryThermalSensor;
use super::gpu_sensor::GpuThermalSensor;
use crate::config::get_config;

pub fn get_critical_temp() -> i8 {
    get_config().thermal.critical_temp
}

pub fn get_throttle_temp() -> i8 {
    get_config().thermal.throttle_temp
}

pub fn get_fan_active_temp() -> i8 {
    get_config().thermal.warning_temp
}

pub struct ThermalController {
    cpu_sensor: CpuThermalSensor,
    battery_sensor: BatteryThermalSensor,
    gpu_sensor: GpuThermalSensor,
}
impl ThermalController {
    pub fn new() -> Self {
        ThermalController {
            cpu_sensor: CpuThermalSensor::new(),
            battery_sensor: BatteryThermalSensor::new(),
            gpu_sensor: GpuThermalSensor::new(),
        }
    }
    pub fn get_cpu_sensor(&self) -> &CpuThermalSensor {
        &self.cpu_sensor
    }
    pub fn get_battery_sensor(&self) -> &BatteryThermalSensor {
        &self.battery_sensor
    }
    pub fn get_gpu_sensor(&self) -> &GpuThermalSensor {
        &self.gpu_sensor
    }
    pub fn get_max_temperature(&self) -> Result<f32, String> {
        Ok(80.0)
    }
    pub fn is_thermal_critical(&self) -> bool {
        let cpu_temp = self.cpu_sensor.read_temperature().unwrap_or(40.0) as i8;
        let critical = get_critical_temp();
        
        cpu_temp >= critical ||
        self.battery_sensor.is_critical() ||
        self.gpu_sensor.should_throttle()
    }
    pub fn should_throttle(&self) -> bool {
        let cpu_temp = self.cpu_sensor.read_temperature().unwrap_or(40.0) as i8;
        let throttle = get_throttle_temp();
        
        cpu_temp >= throttle
    }
    pub fn is_fan_active(&self) -> bool {
        let cpu_temp = self.cpu_sensor.read_temperature().unwrap_or(40.0) as i8;
        let fan_temp = get_fan_active_temp();
        
        cpu_temp >= fan_temp
    }
}
impl Default for ThermalController {
    fn default() -> Self {
        Self::new()
    }
}
pub fn get_temperature() -> Result<i32, &'static str> {
    Ok(40)
}

pub fn get_status() -> Result<u32, &'static str> {
    Ok(0)
}
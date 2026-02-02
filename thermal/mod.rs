pub mod thermal_control;
pub mod manager;
pub mod battery_sensor;
pub mod cpu_sensor;
pub mod gpu_sensor;

pub use thermal_control::ThermalController;
pub use manager::ThermalManager;
pub use battery_sensor::BatteryThermalSensor;
pub use cpu_sensor::CpuThermalSensor;
pub use gpu_sensor::GpuThermalSensor;

pub fn get_temperature() -> Result<i32, &'static str> {
    thermal_control::get_temperature()
}

pub fn get_status() -> Result<u32, &'static str> {
    thermal_control::get_status()
}

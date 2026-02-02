extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
pub use crate::{ComponentState, ErrorTelemetry, HardwareManager, InitError, SystemHealth};

pub fn new() -> HardwareManager {
	HardwareManager::new()
}

pub fn with_default_config() -> HardwareManager {
	HardwareManager::with_default_config()
}

pub fn init_all(manager: &mut HardwareManager) -> Result<(), String> {
	manager.init_all()
}

pub fn low_power_mode(manager: &mut HardwareManager) {
	manager.low_power_mode()
}

pub fn exit_low_power_mode(manager: &mut HardwareManager) {
	manager.exit_low_power_mode()
}

pub fn hard_reset(manager: &mut HardwareManager) {
	manager.hard_reset()
}

pub fn get_state(manager: &HardwareManager, component: &str) -> ComponentState {
	manager.get_state(component)
}

pub fn all_critical_ready(manager: &HardwareManager) -> bool {
	manager.all_critical_ready()
}

pub fn errors(manager: &HardwareManager) -> &[InitError] {
	manager.errors()
}

pub fn take_errors(manager: &mut HardwareManager) -> Vec<InitError> {
	manager.take_errors()
}

pub fn system_health(manager: &HardwareManager) -> SystemHealth {
	manager.system_health()
}

pub fn telemetry(manager: &HardwareManager) -> &ErrorTelemetry {
	manager.telemetry()
}

use crate::thermal::manager::ThermalManager;

pub fn init() -> Result<(), &'static str> {
    let manager = ThermalManager::new();
    manager.initialize()
}

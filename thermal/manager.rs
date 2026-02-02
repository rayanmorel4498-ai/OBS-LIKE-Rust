use core::sync::atomic::{AtomicU32, Ordering};
use crate::config::get_config;

#[derive(Debug, Clone, Copy)]
pub struct ThermalStatus {
    pub temp_celsius: u32,
    pub max_threshold: u32,
    pub critical: bool,
    pub cooling_active: bool,
}

pub struct ThermalManager {
    current_temp: AtomicU32,
    max_threshold: AtomicU32,
    critical_temp: AtomicU32,
    cooling_level: AtomicU32,
}

impl ThermalManager {
    pub fn new() -> Self {
        let cfg = get_config();
        ThermalManager {
            current_temp: AtomicU32::new(35),
            max_threshold: AtomicU32::new(cfg.thermal.throttle_temp as u32),
            critical_temp: AtomicU32::new(cfg.thermal.critical_temp as u32),
            cooling_level: AtomicU32::new(0),
        }
    }

    pub fn initialize(&self) -> Result<(), &'static str> {
        let cfg = get_config();
        self.current_temp.store(35, Ordering::Release);
        self.max_threshold.store(cfg.thermal.throttle_temp as u32, Ordering::Release);
        self.critical_temp.store(cfg.thermal.critical_temp as u32, Ordering::Release);
        Ok(())
    }

    pub fn update_temperature(&self, temp: u32) {
        self.current_temp.store(temp, Ordering::Release);
    }

    pub fn get_current_temperature(&self) -> u32 {
        self.current_temp.load(Ordering::Acquire)
    }

    pub fn set_threshold(&self, threshold: u32) -> Result<(), &'static str> {
        self.max_threshold.store(threshold, Ordering::Release);
        Ok(())
    }

    pub fn get_threshold(&self) -> u32 {
        self.max_threshold.load(Ordering::Acquire)
    }

    pub fn get_status(&self) -> ThermalStatus {
        let temp = self.current_temp.load(Ordering::Acquire);
        let thresh = self.max_threshold.load(Ordering::Acquire);
        let critical = self.critical_temp.load(Ordering::Acquire);

        ThermalStatus {
            temp_celsius: temp,
            max_threshold: thresh,
            critical: temp > critical,
            cooling_active: temp > thresh,
        }
    }

    pub fn trigger_cooling(&self, level: u32) -> Result<(), &'static str> {
        if level > 100 {
            return Err("Invalid cooling level");
        }
        self.cooling_level.store(level, Ordering::Release);
        Ok(())
    }

    pub fn get_cooling_level(&self) -> u32 {
        self.cooling_level.load(Ordering::Acquire)
    }
}

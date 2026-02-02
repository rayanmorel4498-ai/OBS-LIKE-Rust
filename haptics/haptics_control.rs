extern crate alloc;
use alloc::string::String;

pub struct Vibrator {
    #[allow(dead_code)]
    intensity: u8,
}

impl Vibrator {
    pub fn new() -> Self {
        Vibrator { intensity: 100 }
    }
}

pub struct LinearActuator {
    #[allow(dead_code)]
    frequency: u32,
}

impl LinearActuator {
    pub fn new() -> Self {
        LinearActuator { frequency: 200 }
    }
}

pub struct HapticsController {
    vibrator: Vibrator,
    linear: LinearActuator,
    #[allow(dead_code)]
    enabled: bool,
}
impl HapticsController {
    pub fn new() -> Self {
        HapticsController {
            vibrator: Vibrator::new(),
            linear: LinearActuator::new(),
            enabled: false,
        }
    }
    pub fn click_feedback(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn double_tap_feedback(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn long_press_feedback(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn get_vibrator(&self) -> &Vibrator {
        &self.vibrator
    }
    pub fn get_linear(&self) -> &LinearActuator {
        &self.linear
    }
}
impl Default for HapticsController {
    fn default() -> Self {
        Self::new()
    }
}
pub fn vibrate(duration: u32) -> Result<(), &'static str> {
    if duration == 0 {
        return Err("Duration must be greater than 0");
    }
    Ok(())
}
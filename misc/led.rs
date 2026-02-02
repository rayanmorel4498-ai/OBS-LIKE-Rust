extern crate alloc;
use alloc::string::String;
use core::sync::atomic::{AtomicU8, AtomicBool, Ordering};
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEDColor {
    Off = 0,
    Red = 1,
    Green = 2,
    Blue = 3,
    Yellow = 4,
    Cyan = 5,
    Magenta = 6,
    White = 7,
}
pub struct LED {
    color: AtomicU8,
    brightness: AtomicU8,
    blinking: AtomicBool,
}
impl LED {
    pub fn new() -> Self {
        LED {
            color: AtomicU8::new(LEDColor::Off as u8),
            brightness: AtomicU8::new(255),
            blinking: AtomicBool::new(false),
        }
    }
    pub fn set_color(&self, color: LEDColor) -> Result<(), String> {
        self.color.store(color as u8, Ordering::SeqCst);
        Ok(())
    }
    pub fn set_brightness(&self, brightness: u8) -> Result<(), String> {
        self.brightness.store(brightness, Ordering::SeqCst);
        Ok(())
    }
    pub fn set_blinking(&self, enabled: bool) -> Result<(), String> {
        self.blinking.store(enabled, Ordering::SeqCst);
        Ok(())
    }
    pub fn get_color(&self) -> LEDColor {
        match self.color.load(Ordering::SeqCst) {
            1 => LEDColor::Red,
            2 => LEDColor::Green,
            3 => LEDColor::Blue,
            4 => LEDColor::Yellow,
            5 => LEDColor::Cyan,
            6 => LEDColor::Magenta,
            7 => LEDColor::White,
            _ => LEDColor::Off,
        }
    }
    pub fn get_brightness(&self) -> u8 {
        self.brightness.load(Ordering::SeqCst)
    }
    pub fn is_blinking(&self) -> bool {
        self.blinking.load(Ordering::SeqCst)
    }
}
impl Default for LED {
    fn default() -> Self {
        Self::new()
    }
}
pub fn enable() -> Result<(), &'static str> {
    Ok(())
}
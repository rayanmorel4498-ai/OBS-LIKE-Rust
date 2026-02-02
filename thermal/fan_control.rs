const FAN_CTRL_BASE: u64 = 0xFD10_0000;
const FAN_CTRL: u64 = FAN_CTRL_BASE + 0x0000;
const FAN_STATUS: u64 = FAN_CTRL_BASE + 0x0004;
const FAN_SPEED: u64 = FAN_CTRL_BASE + 0x0008;
const FAN_PWM: u64 = FAN_CTRL_BASE + 0x000C;
const FAN_CONFIG: u64 = FAN_CTRL_BASE + 0x0010;
const FAN_TEMP: u64 = FAN_CTRL_BASE + 0x0014;
const FAN_MODE: u64 = FAN_CTRL_BASE + 0x0018;
const FAN_DATA: u64 = FAN_CTRL_BASE + 0x001C;

pub fn set_speed(speed: u8) -> Result<(), &'static str> {
    unsafe {
        core::ptr::write_volatile(FAN_CTRL as *mut u32, 0x1);
        core::ptr::write_volatile(FAN_STATUS as *mut u32, 0x0);
        core::ptr::write_volatile(FAN_SPEED as *mut u32, speed as u32);
        core::ptr::write_volatile(FAN_PWM as *mut u32, speed as u32);
        core::ptr::write_volatile(FAN_CONFIG as *mut u32, 0x1);
        core::ptr::write_volatile(FAN_TEMP as *mut u32, 0x0);
        core::ptr::write_volatile(FAN_MODE as *mut u32, 0x1);
        core::ptr::write_volatile(FAN_DATA as *mut u32, speed as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

const THERMAL_THROTTLE_BASE: u64 = 0xFD11_0000;
const THERMAL_CTRL: u64 = THERMAL_THROTTLE_BASE + 0x0000;
const THERMAL_STATUS: u64 = THERMAL_THROTTLE_BASE + 0x0004;
const THERMAL_LIMIT: u64 = THERMAL_THROTTLE_BASE + 0x0008;
const THERMAL_CURRENT: u64 = THERMAL_THROTTLE_BASE + 0x000C;
const THERMAL_CONFIG: u64 = THERMAL_THROTTLE_BASE + 0x0010;
const THERMAL_MODE: u64 = THERMAL_THROTTLE_BASE + 0x0014;
const THERMAL_FREQ: u64 = THERMAL_THROTTLE_BASE + 0x0018;
const THERMAL_DATA: u64 = THERMAL_THROTTLE_BASE + 0x001C;

pub fn set_limit(temp: i16) -> Result<(), &'static str> {
    unsafe {
        core::ptr::write_volatile(THERMAL_CTRL as *mut u32, 0x1);
        core::ptr::write_volatile(THERMAL_STATUS as *mut u32, 0x0);
        core::ptr::write_volatile(THERMAL_LIMIT as *mut u32, temp as u32);
        core::ptr::write_volatile(THERMAL_CURRENT as *mut u32, 0x0);
        core::ptr::write_volatile(THERMAL_CONFIG as *mut u32, 0x1);
        core::ptr::write_volatile(THERMAL_MODE as *mut u32, 0x1);
        core::ptr::write_volatile(THERMAL_FREQ as *mut u32, temp as u32);
        core::ptr::write_volatile(THERMAL_DATA as *mut u32, temp as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

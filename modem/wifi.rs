use core::ptr::{read_volatile, write_volatile};

const WIFI_CTRL_OFFSET: u64 = 0x0000;
const WIFI_STATUS_OFFSET: u64 = 0x0004;
const WIFI_FREQ_OFFSET: u64 = 0x0008;
const WIFI_BAND_OFFSET: u64 = 0x000C;
const WIFI_POWER_OFFSET: u64 = 0x0010;
const WIFI_SIGNAL_OFFSET: u64 = 0x0014;
const WIFI_STANDARD_OFFSET: u64 = 0x0018;
const WIFI_CONFIG_OFFSET: u64 = 0x001C;

fn wifi_reg(offset: u64) -> u64 {
    crate::wifi_base() + offset
}

pub fn init() -> Result<(), &'static str> {
    unsafe {
        write_volatile(wifi_reg(WIFI_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        let status = read_volatile(wifi_reg(WIFI_STATUS_OFFSET) as *const u32);
        if status & 0x1 == 0 {
            return Err("WiFi initialization failed");
        }
    }
    Ok(())
}

pub fn enable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(wifi_reg(WIFI_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(wifi_reg(WIFI_CTRL_OFFSET) as *mut u32, 0x0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_status() -> u32 {
    unsafe { read_volatile(wifi_reg(WIFI_STATUS_OFFSET) as *const u32) }
}

pub fn set_frequency(freq: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(wifi_reg(WIFI_FREQ_OFFSET) as *mut u32, freq);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_frequency() -> u32 {
    unsafe { read_volatile(wifi_reg(WIFI_FREQ_OFFSET) as *const u32) }
}

pub fn set_band(band: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(wifi_reg(WIFI_BAND_OFFSET) as *mut u32, band);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_band() -> u32 {
    unsafe { read_volatile(wifi_reg(WIFI_BAND_OFFSET) as *const u32) }
}

pub fn set_power(power: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(wifi_reg(WIFI_POWER_OFFSET) as *mut u32, power);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_signal() -> u32 {
    unsafe { read_volatile(wifi_reg(WIFI_SIGNAL_OFFSET) as *const u32) }
}

pub fn set_standard(standard: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(wifi_reg(WIFI_STANDARD_OFFSET) as *mut u32, standard);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_standard() -> u32 {
    unsafe { read_volatile(wifi_reg(WIFI_STANDARD_OFFSET) as *const u32) }
}

pub fn set_config(config: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(wifi_reg(WIFI_CONFIG_OFFSET) as *mut u32, config);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_config() -> u32 {
    unsafe { read_volatile(wifi_reg(WIFI_CONFIG_OFFSET) as *const u32) }
}

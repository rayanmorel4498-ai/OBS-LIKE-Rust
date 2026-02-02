use core::ptr::{read_volatile, write_volatile};

const SAT_CTRL_OFFSET: u64 = 0x0000;
const SAT_STATUS_OFFSET: u64 = 0x0004;
const SAT_FREQ_OFFSET: u64 = 0x0008;
const SAT_BAND_OFFSET: u64 = 0x000C;
const SAT_POWER_OFFSET: u64 = 0x0010;
const SAT_SIGNAL_OFFSET: u64 = 0x0014;
const SAT_LINK_OFFSET: u64 = 0x0018;
const SAT_CONFIG_OFFSET: u64 = 0x001C;

fn sat_reg(offset: u64) -> u64 {
    crate::satellite_base() + offset
}

pub fn init() -> Result<(), &'static str> {
    unsafe {
        write_volatile(sat_reg(SAT_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        let status = read_volatile(sat_reg(SAT_STATUS_OFFSET) as *const u32);
        if status & 0x1 == 0 {
            return Err("Satellite modem initialization failed");
        }
    }
    Ok(())
}

pub fn enable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(sat_reg(SAT_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(sat_reg(SAT_CTRL_OFFSET) as *mut u32, 0x0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_status() -> u32 {
    unsafe { read_volatile(sat_reg(SAT_STATUS_OFFSET) as *const u32) }
}

pub fn set_frequency(freq: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sat_reg(SAT_FREQ_OFFSET) as *mut u32, freq);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_frequency() -> u32 {
    unsafe { read_volatile(sat_reg(SAT_FREQ_OFFSET) as *const u32) }
}

pub fn set_band(band: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sat_reg(SAT_BAND_OFFSET) as *mut u32, band);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_band() -> u32 {
    unsafe { read_volatile(sat_reg(SAT_BAND_OFFSET) as *const u32) }
}

pub fn set_power(power: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sat_reg(SAT_POWER_OFFSET) as *mut u32, power);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_signal() -> u32 {
    unsafe { read_volatile(sat_reg(SAT_SIGNAL_OFFSET) as *const u32) }
}

pub fn set_link(link: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sat_reg(SAT_LINK_OFFSET) as *mut u32, link);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_link() -> u32 {
    unsafe { read_volatile(sat_reg(SAT_LINK_OFFSET) as *const u32) }
}

pub fn set_config(config: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sat_reg(SAT_CONFIG_OFFSET) as *mut u32, config);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_config() -> u32 {
    unsafe { read_volatile(sat_reg(SAT_CONFIG_OFFSET) as *const u32) }
}

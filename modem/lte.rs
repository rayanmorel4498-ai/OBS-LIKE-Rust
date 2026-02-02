use core::ptr::{read_volatile, write_volatile};

const LTE_CTRL_OFFSET: u64 = 0x0000;
const LTE_STATUS_OFFSET: u64 = 0x0004;
const LTE_FREQ_OFFSET: u64 = 0x0008;
const LTE_BAND_OFFSET: u64 = 0x000C;
const LTE_POWER_OFFSET: u64 = 0x0010;
const LTE_SIGNAL_OFFSET: u64 = 0x0014;
const LTE_RSRP_OFFSET: u64 = 0x0018;
const LTE_RSRQ_OFFSET: u64 = 0x001C;

fn lte_reg(offset: u64) -> u64 {
    crate::lte_base() + offset
}

pub fn init() -> Result<(), &'static str> {
    unsafe {
        write_volatile(lte_reg(LTE_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn enable() -> Result<(), &'static str> {
    unsafe {
        let ctrl = read_volatile(lte_reg(LTE_CTRL_OFFSET) as *const u32);
        write_volatile(lte_reg(LTE_CTRL_OFFSET) as *mut u32, ctrl | 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    unsafe {
        let ctrl = read_volatile(lte_reg(LTE_CTRL_OFFSET) as *const u32);
        write_volatile(lte_reg(LTE_CTRL_OFFSET) as *mut u32, ctrl & !0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_status() -> u32 {
    unsafe { read_volatile(lte_reg(LTE_STATUS_OFFSET) as *const u32) }
}

pub fn set_frequency(freq_mhz: u32) -> Result<(), &'static str> {
    if freq_mhz < 600 || freq_mhz > 3800 {
        return Err("frequency_out_of_range");
    }
    unsafe {
        write_volatile(lte_reg(LTE_FREQ_OFFSET) as *mut u32, freq_mhz);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_frequency() -> u32 {
    unsafe { read_volatile(lte_reg(LTE_FREQ_OFFSET) as *const u32) }
}

pub fn set_band(band: u8) -> Result<(), &'static str> {
    unsafe {
        write_volatile(lte_reg(LTE_BAND_OFFSET) as *mut u32, band as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_band() -> u32 {
    unsafe { read_volatile(lte_reg(LTE_BAND_OFFSET) as *const u32) }
}

pub fn set_power(dbm: u16) -> Result<(), &'static str> {
    if dbm > 33 {
        return Err("power_exceeds_max");
    }
    unsafe {
        write_volatile(lte_reg(LTE_POWER_OFFSET) as *mut u32, dbm as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_signal() -> u32 {
    unsafe { read_volatile(lte_reg(LTE_SIGNAL_OFFSET) as *const u32) }
}

pub fn get_rsrp() -> u32 {
    unsafe { read_volatile(lte_reg(LTE_RSRP_OFFSET) as *const u32) }
}

pub fn get_rsrq() -> u32 {
    unsafe { read_volatile(lte_reg(LTE_RSRQ_OFFSET) as *const u32) }
}

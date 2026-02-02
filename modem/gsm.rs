use core::ptr::{read_volatile, write_volatile};

const GSM_CTRL_OFFSET: u64 = 0x0000;
const GSM_STATUS_OFFSET: u64 = 0x0004;
const GSM_FREQ_OFFSET: u64 = 0x0008;
const GSM_BAND_OFFSET: u64 = 0x000C;
const GSM_POWER_OFFSET: u64 = 0x0010;
const GSM_SIGNAL_OFFSET: u64 = 0x0014;
const GSM_CHANNEL_OFFSET: u64 = 0x0018;
const GSM_TIMESLOT_OFFSET: u64 = 0x001C;

fn gsm_reg(offset: u64) -> u64 {
    crate::gsm_base() + offset
}

pub fn init() -> Result<(), &'static str> {
    unsafe {
        write_volatile(gsm_reg(GSM_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn enable() -> Result<(), &'static str> {
    unsafe {
        let ctrl = read_volatile(gsm_reg(GSM_CTRL_OFFSET) as *const u32);
        write_volatile(gsm_reg(GSM_CTRL_OFFSET) as *mut u32, ctrl | 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    unsafe {
        let ctrl = read_volatile(gsm_reg(GSM_CTRL_OFFSET) as *const u32);
        write_volatile(gsm_reg(GSM_CTRL_OFFSET) as *mut u32, ctrl & !0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_status() -> u32 {
    unsafe { read_volatile(gsm_reg(GSM_STATUS_OFFSET) as *const u32) }
}

pub fn set_frequency(freq_mhz: u32) -> Result<(), &'static str> {
    if freq_mhz < 800 || freq_mhz > 1900 {
        return Err("frequency_out_of_range");
    }
    unsafe {
        write_volatile(gsm_reg(GSM_FREQ_OFFSET) as *mut u32, freq_mhz);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_frequency() -> u32 {
    unsafe { read_volatile(gsm_reg(GSM_FREQ_OFFSET) as *const u32) }
}

pub fn set_band(band: u8) -> Result<(), &'static str> {
    unsafe {
        write_volatile(gsm_reg(GSM_BAND_OFFSET) as *mut u32, band as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_band() -> u32 {
    unsafe { read_volatile(gsm_reg(GSM_BAND_OFFSET) as *const u32) }
}

pub fn set_power(dbm: u16) -> Result<(), &'static str> {
    if dbm > 33 {
        return Err("power_exceeds_max");
    }
    unsafe {
        write_volatile(gsm_reg(GSM_POWER_OFFSET) as *mut u32, dbm as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_signal() -> u32 {
    unsafe { read_volatile(gsm_reg(GSM_SIGNAL_OFFSET) as *const u32) }
}

pub fn set_channel(channel: u16) -> Result<(), &'static str> {
    unsafe {
        write_volatile(gsm_reg(GSM_CHANNEL_OFFSET) as *mut u32, channel as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_channel() -> u32 {
    unsafe { read_volatile(gsm_reg(GSM_CHANNEL_OFFSET) as *const u32) }
}

pub fn get_timeslot() -> u32 {
    unsafe { read_volatile(gsm_reg(GSM_TIMESLOT_OFFSET) as *const u32) }
}

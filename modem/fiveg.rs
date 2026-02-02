use core::ptr::{read_volatile, write_volatile};

const NR_CTRL_OFFSET: u64 = 0x0000;
const NR_STATUS_OFFSET: u64 = 0x0004;
const NR_FREQ_OFFSET: u64 = 0x0008;
const NR_BAND_OFFSET: u64 = 0x000C;
const NR_POWER_OFFSET: u64 = 0x0010;
const NR_SIGNAL_OFFSET: u64 = 0x0014;
const NR_LATENCY_OFFSET: u64 = 0x0018;
const NR_THROUGHPUT_OFFSET: u64 = 0x001C;

fn nr_reg(offset: u64) -> u64 {
    crate::fiveg_base() + offset
}

pub fn init() -> Result<(), &'static str> {
    unsafe {
        write_volatile(nr_reg(NR_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn enable() -> Result<(), &'static str> {
    unsafe {
        let ctrl = read_volatile(nr_reg(NR_CTRL_OFFSET) as *const u32);
        write_volatile(nr_reg(NR_CTRL_OFFSET) as *mut u32, ctrl | 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    unsafe {
        let ctrl = read_volatile(nr_reg(NR_CTRL_OFFSET) as *const u32);
        write_volatile(nr_reg(NR_CTRL_OFFSET) as *mut u32, ctrl & !0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_status() -> u32 {
    unsafe { read_volatile(nr_reg(NR_STATUS_OFFSET) as *const u32) }
}

pub fn set_frequency(freq_mhz: u32) -> Result<(), &'static str> {
    if freq_mhz < 600 || freq_mhz > 6000 {
        return Err("frequency_out_of_range");
    }
    unsafe {
        write_volatile(nr_reg(NR_FREQ_OFFSET) as *mut u32, freq_mhz);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_frequency() -> u32 {
    unsafe { read_volatile(nr_reg(NR_FREQ_OFFSET) as *const u32) }
}

pub fn set_band(band: u8) -> Result<(), &'static str> {
    unsafe {
        write_volatile(nr_reg(NR_BAND_OFFSET) as *mut u32, band as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_band() -> u32 {
    unsafe { read_volatile(nr_reg(NR_BAND_OFFSET) as *const u32) }
}

pub fn set_power(dbm: u16) -> Result<(), &'static str> {
    if dbm > 33 {
        return Err("power_exceeds_max");
    }
    unsafe {
        write_volatile(nr_reg(NR_POWER_OFFSET) as *mut u32, dbm as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_signal() -> u32 {
    unsafe { read_volatile(nr_reg(NR_SIGNAL_OFFSET) as *const u32) }
}

pub fn get_latency() -> u32 {
    unsafe { read_volatile(nr_reg(NR_LATENCY_OFFSET) as *const u32) }
}

pub fn get_throughput() -> u32 {
    unsafe { read_volatile(nr_reg(NR_THROUGHPUT_OFFSET) as *const u32) }
}

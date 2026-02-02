use core::ptr::{read_volatile, write_volatile};

const BT_CTRL_OFFSET: u64 = 0x0000;
const BT_STATUS_OFFSET: u64 = 0x0004;
const BT_FREQ_OFFSET: u64 = 0x0008;
const BT_BAND_OFFSET: u64 = 0x000C;
const BT_POWER_OFFSET: u64 = 0x0010;
const BT_SIGNAL_OFFSET: u64 = 0x0014;
const BT_MODE_OFFSET: u64 = 0x0018;
const BT_CONFIG_OFFSET: u64 = 0x001C;

fn bt_reg(offset: u64) -> u64 {
    crate::bt_base() + offset
}

pub fn init() -> Result<(), &'static str> {
    unsafe {
        write_volatile(bt_reg(BT_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        let status = read_volatile(bt_reg(BT_STATUS_OFFSET) as *const u32);
        if status & 0x1 == 0 {
            return Err("BT initialization failed");
        }
    }
    Ok(())
}

pub fn enable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(bt_reg(BT_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(bt_reg(BT_CTRL_OFFSET) as *mut u32, 0x0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_status() -> u32 {
    unsafe { read_volatile(bt_reg(BT_STATUS_OFFSET) as *const u32) }
}

pub fn set_frequency(freq: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(bt_reg(BT_FREQ_OFFSET) as *mut u32, freq);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_frequency() -> u32 {
    unsafe { read_volatile(bt_reg(BT_FREQ_OFFSET) as *const u32) }
}

pub fn set_band(band: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(bt_reg(BT_BAND_OFFSET) as *mut u32, band);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_band() -> u32 {
    unsafe { read_volatile(bt_reg(BT_BAND_OFFSET) as *const u32) }
}

pub fn set_power(power: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(bt_reg(BT_POWER_OFFSET) as *mut u32, power);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_signal() -> u32 {
    unsafe { read_volatile(bt_reg(BT_SIGNAL_OFFSET) as *const u32) }
}

pub fn set_mode(mode: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(bt_reg(BT_MODE_OFFSET) as *mut u32, mode);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_mode() -> u32 {
    unsafe { read_volatile(bt_reg(BT_MODE_OFFSET) as *const u32) }
}

pub fn set_config(config: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(bt_reg(BT_CONFIG_OFFSET) as *mut u32, config);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_config() -> u32 {
    unsafe { read_volatile(bt_reg(BT_CONFIG_OFFSET) as *const u32) }
}

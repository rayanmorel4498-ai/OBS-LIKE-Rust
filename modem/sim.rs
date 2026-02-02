use core::ptr::{read_volatile, write_volatile};

const SIM_CTRL_OFFSET: u64 = 0x0000;
const SIM_STATUS_OFFSET: u64 = 0x0004;
const SIM_ICCID_OFFSET: u64 = 0x0008;
const SIM_IMSI_OFFSET: u64 = 0x000C;
const SIM_PIN_OFFSET: u64 = 0x0010;
const SIM_AUTH_OFFSET: u64 = 0x0014;
const SIM_DATA_OFFSET: u64 = 0x0018;
const SIM_CONFIG_OFFSET: u64 = 0x001C;

fn sim_reg(offset: u64) -> u64 {
    crate::sim_base() + offset
}

pub fn init() -> Result<(), &'static str> {
    unsafe {
        write_volatile(sim_reg(SIM_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        let status = read_volatile(sim_reg(SIM_STATUS_OFFSET) as *const u32);
        if status & 0x1 == 0 {
            return Err("SIM initialization failed");
        }
    }
    Ok(())
}

pub fn enable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(sim_reg(SIM_CTRL_OFFSET) as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(sim_reg(SIM_CTRL_OFFSET) as *mut u32, 0x0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_status() -> u32 {
    unsafe { read_volatile(sim_reg(SIM_STATUS_OFFSET) as *const u32) }
}

pub fn get_iccid() -> u32 {
    unsafe { read_volatile(sim_reg(SIM_ICCID_OFFSET) as *const u32) }
}

pub fn get_imsi() -> u32 {
    unsafe { read_volatile(sim_reg(SIM_IMSI_OFFSET) as *const u32) }
}

pub fn verify_pin(pin: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sim_reg(SIM_PIN_OFFSET) as *mut u32, pin);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_pin() -> u32 {
    unsafe { read_volatile(sim_reg(SIM_PIN_OFFSET) as *const u32) }
}

pub fn set_auth(auth: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sim_reg(SIM_AUTH_OFFSET) as *mut u32, auth);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_auth() -> u32 {
    unsafe { read_volatile(sim_reg(SIM_AUTH_OFFSET) as *const u32) }
}

pub fn write_data(data: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sim_reg(SIM_DATA_OFFSET) as *mut u32, data);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn read_data() -> u32 {
    unsafe { read_volatile(sim_reg(SIM_DATA_OFFSET) as *const u32) }
}

pub fn set_config(config: u32) -> Result<(), &'static str> {
    unsafe {
        write_volatile(sim_reg(SIM_CONFIG_OFFSET) as *mut u32, config);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_config() -> u32 {
    unsafe { read_volatile(sim_reg(SIM_CONFIG_OFFSET) as *const u32) }
}

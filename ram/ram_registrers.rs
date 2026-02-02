use core::ptr::{read_volatile, write_volatile};

pub type RegAddr = u64;

const REG_STRIDE: u64 = 0x10;

#[inline(always)]
pub unsafe fn read_reg(addr: RegAddr) -> u32 {
    read_volatile(addr as *const u32)
}

#[inline(always)]
pub unsafe fn write_reg(addr: RegAddr, value: u32) {
    write_volatile(addr as *mut u32, value);
}

#[inline(always)]
pub unsafe fn write_reg_fenced(addr: RegAddr, value: u32) {
    write_volatile(addr as *mut u32, value);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn set_bits(addr: RegAddr, mask: u32) {
    let val = read_reg(addr);
    write_reg(addr, val | mask);
}

#[inline(always)]
pub unsafe fn clear_bits(addr: RegAddr, mask: u32) {
    let val = read_reg(addr);
    write_reg(addr, val & !mask);
}

#[inline(always)]
pub unsafe fn write_field(addr: RegAddr, mask: u32, shift: u8, value: u32) {
    let mut val = read_reg(addr);
    val &= !mask;
    val |= (value << shift) & mask;
    write_reg(addr, val);
}

#[inline(always)]
pub unsafe fn read_field(addr: RegAddr, mask: u32, shift: u8) -> u32 {
    (read_reg(addr) & mask) >> shift
}

pub fn get_phy_freq() -> u32 {
    unsafe { read_reg(crate::phy_freq_reg()) }
}

pub fn set_phy_freq(freq: u32) {
    unsafe { write_reg(crate::phy_freq_reg(), freq); }
}

pub fn get_memc_status() -> u32 {
    unsafe { read_reg(crate::memc_status_reg()) }
}

pub fn get_phy_status() -> u32 {
    unsafe { read_reg(crate::phy_status_reg()) }
}

pub fn get_refresh_timer() -> u32 {
    unsafe { read_reg(crate::refresh_timer()) }
}

pub fn set_memc_refresh_interval(interval: u32) {
    unsafe { write_reg(crate::memc_refresh_reg(), interval); }
}

pub fn get_axi_config() -> u32 {
    unsafe { read_reg(crate::axi_config_reg()) }
}

pub fn set_axi_config(config: u32) {
    unsafe { write_reg(crate::axi_config_reg(), config); }
}

pub fn get_axi_status() -> u32 {
    unsafe { read_reg(crate::axi_status_reg()) }
}

pub fn get_phy_mode() -> u32 {
    unsafe { read_reg(crate::phy_mode_reg()) }
}

pub fn get_phy_timing() -> u32 {
    unsafe { read_reg(crate::phy_timing_reg()) }
}

pub fn get_phy_voltage() -> u32 {
    unsafe { read_reg(crate::phy_voltage_reg()) }
}

pub fn get_phy_power() -> u32 {
    unsafe { read_reg(crate::phy_power_reg()) }
}

pub fn get_phy_security_ctrl() -> u32 {
    unsafe { read_reg(crate::phy_security_ctrl()) }
}

pub fn get_phy_security_status() -> u32 {
    unsafe { read_reg(crate::phy_security_status()) }
}

pub fn get_memc_ctrl() -> u32 {
    unsafe { read_reg(crate::memc_ctrl_reg()) }
}

pub fn get_memc_freq() -> u32 {
    unsafe { read_reg(crate::memc_freq_reg()) }
}

pub fn get_memc_timing() -> u32 {
    unsafe { read_reg(crate::memc_timing_reg()) }
}

pub fn get_refresh_status() -> u32 {
    unsafe { read_reg(crate::refresh_status()) }
}

pub fn get_memc_lock_ctrl() -> u32 {
    unsafe { read_reg(crate::memc_lock_ctrl()) }
}

pub fn get_memc_erase_ctrl() -> u32 {
    unsafe { read_reg(crate::memc_erase_ctrl()) }
}

pub fn get_memc_debug_ctrl() -> u32 {
    unsafe { read_reg(crate::memc_debug_ctrl()) }
}

pub fn get_reg_stride() -> u64 {
    REG_STRIDE
}

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use core::ptr::{read_volatile, write_volatile};
use crate::config::get_config;

static RAM_INITIALIZED: AtomicBool = AtomicBool::new(false);
static RAM_FREQ_MHZ: AtomicU32 = AtomicU32::new(0);
static RAM_LOCK: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);
const MIN_FREQ_MHZ: u32 = 533;

struct RamLockGuard;

impl Drop for RamLockGuard {
    fn drop(&mut self) {
        RAM_LOCK.store(false, Ordering::Release);
    }
}

fn lock_ram() -> RamLockGuard {
    while RAM_LOCK
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_err()
    {
        core::hint::spin_loop();
    }
    RamLockGuard
}

fn ram_size() -> u64 {
    (get_config().ram.size_mb as u64) * 1024 * 1024
}

fn ram_frequency() -> u32 {
    get_config().ram.frequency
}

pub struct RamSpec;
impl RamSpec {
    pub const MEMORY_TYPE: &'static str = "LPDDR4x";
    pub const MAX_FREQUENCY_MHZ: u32 = 2133;
    pub const STANDARD_FREQUENCY_MHZ: u32 = 1800;
    pub const BUS_WIDTH_BITS: u32 = 128;
    pub const TOTAL_SIZE_BYTES: u64 = 8 * 1024 * 1024 * 1024;
    pub const REFRESH_RATE_US: u32 = 7_800;
}
pub fn init() -> Result<(), &'static str> {
    if RAM_INITIALIZED.load(Ordering::SeqCst) {
        return Ok(());
    }
    
    let freq = ram_frequency();
    if freq < MIN_FREQ_MHZ || freq > RamSpec::MAX_FREQUENCY_MHZ {
        return Err("invalid_ram_frequency");
    }
    
    unsafe {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        
        let phy_pwr = read_volatile(crate::phy_power_reg() as *const u32);
        write_volatile(crate::phy_power_reg() as *mut u32, phy_pwr | 0x1);
        
        let memc_ctrl = read_volatile(crate::memc_ctrl_reg() as *const u32);
        write_volatile(crate::memc_ctrl_reg() as *mut u32, memc_ctrl | 0x1);
        
        write_volatile(crate::phy_freq_reg() as *mut u32, freq);
        write_volatile(crate::memc_freq_reg() as *mut u32, freq);
        
        write_volatile(crate::memc_refresh_reg() as *mut u32, RamSpec::REFRESH_RATE_US);
        
        let timeout = 1000;
        let mut count = 0;
        while count < timeout {
            let status = read_volatile(crate::memc_status_reg() as *const u32);
            if (status & 0x1) != 0 {
                break;
            }
            count += 1;
        }
        
        if count >= timeout {
            return Err("ram_init_timeout");
        }
        
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    RAM_INITIALIZED.store(true, Ordering::SeqCst);
    RAM_FREQ_MHZ.store(freq, Ordering::SeqCst);
    Ok(())
}
pub fn is_ready() -> bool {
    RAM_INITIALIZED.load(Ordering::SeqCst)
}
pub fn map_physical(addr: u64, size: u64) -> Result<u64, &'static str> {
    let ram_base = crate::ram_base();
    let ram_sz = ram_size();
    if !is_ready() || addr < ram_base || addr.checked_add(size).map_or(true, |end| end > ram_base + ram_sz) {
        return Err("invalid_address_range");
    }
    Ok(addr)
}
pub fn clear_all() {
    if !is_ready() {
        return;
    }
    let _guard = lock_ram();
    unsafe {
        let ram_base = crate::ram_base();
        let ram_sz = ram_size();
        let ptr = ram_base as *mut u32;
        let total_words = ram_sz / 4;
        for i in 0..total_words {
            write_volatile(ptr.add(i as usize), 0);
        }
        let remainder = ram_sz % 4;
        if remainder != 0 {
            let byte_ptr = ram_base as *mut u8;
            let start = total_words * 4;
            for i in 0..remainder {
                write_volatile(byte_ptr.add((start + i) as usize), 0);
            }
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
pub fn set_frequency(mhz: u32) -> Result<(), &'static str> {
    if !is_ready() {
        return Err("ram_not_initialized");
    }
    
    if mhz < MIN_FREQ_MHZ || mhz > RamSpec::MAX_FREQUENCY_MHZ {
        return Err("frequency_out_of_range");
    }
    
    unsafe {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        write_volatile(crate::phy_freq_reg() as *mut u32, mhz);
        write_volatile(crate::memc_freq_reg() as *mut u32, mhz);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    RAM_FREQ_MHZ.store(mhz, Ordering::SeqCst);
    Ok(())
}

pub fn get_frequency() -> u32 {
    RAM_FREQ_MHZ.load(Ordering::SeqCst)
}

pub fn read_status() -> u32 {
    if !is_ready() {
        return 0;
    }
    unsafe {
        read_volatile(crate::memc_status_reg() as *const u32)
    }
}

pub fn get_phy_status() -> u32 {
    if !is_ready() {
        return 0;
    }
    unsafe {
        read_volatile(crate::phy_status_reg() as *const u32)
    }
}

pub fn set_voltage(mv: u16) -> Result<(), &'static str> {
    if !is_ready() {
        return Err("ram_not_initialized");
    }
    
    if mv < 1200 || mv > 1500 {
        return Err("voltage_out_of_range");
    }
    
    unsafe {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        write_volatile(crate::phy_voltage_reg() as *mut u32, mv as u32);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}
pub fn enable_refresh() -> Result<(), &'static str> {
    if !is_ready() {
        return Err("ram_not_initialized");
    }
    unsafe {
        write_volatile(crate::memc_refresh_reg() as *mut u32, 1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable_refresh() -> Result<(), &'static str> {
    if !is_ready() {
        return Err("ram_not_initialized");
    }
    unsafe {
        write_volatile(crate::memc_refresh_reg() as *mut u32, 0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_axi_status() -> u32 {
    unsafe {
        read_volatile(crate::axi_status_reg() as *const u32)
    }
}

pub fn set_phy_mode(mode: u32) -> Result<(), &'static str> {
    if !is_ready() {
        return Err("ram_not_initialized");
    }
    unsafe {
        write_volatile(crate::phy_mode_reg() as *mut u32, mode);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn set_phy_timing(timing: u32) -> Result<(), &'static str> {
    if !is_ready() {
        return Err("ram_not_initialized");
    }
    unsafe {
        write_volatile(crate::phy_timing_reg() as *mut u32, timing);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn set_memc_timing(timing: u32) -> Result<(), &'static str> {
    if !is_ready() {
        return Err("ram_not_initialized");
    }
    unsafe {
        write_volatile(crate::memc_timing_reg() as *mut u32, timing);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn set_axi_config(config: u32) -> Result<(), &'static str> {
    if !is_ready() {
        return Err("ram_not_initialized");
    }
    unsafe {
        write_volatile(crate::axi_config_reg() as *mut u32, config);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}
pub fn enter_low_power() -> Result<(), &'static str> {
    set_frequency(MIN_FREQ_MHZ)
}

pub fn exit_low_power() -> Result<(), &'static str> {
    set_frequency(RamSpec::STANDARD_FREQUENCY_MHZ)
}
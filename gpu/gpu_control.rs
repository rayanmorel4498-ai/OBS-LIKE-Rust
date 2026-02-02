use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, Ordering};

static GPU_ENABLED: AtomicBool = AtomicBool::new(false);

pub struct GpuSpec;
impl GpuSpec {
    pub const GPU_TYPE: &'static str = "Mali-G57 MC2";
    pub const CLUSTERS: u32 = 2;
    pub const CORES_PER_CLUSTER: u32 = 4;
    pub const MAX_FREQ_MHZ: u32 = 1000;
    pub const MIN_FREQ_MHZ: u32 = 300;
}

#[inline(always)]
unsafe fn gpu_read32(addr: u64) -> u32 {
    read_volatile(addr as *const u32)
}

#[inline(always)]
unsafe fn gpu_write32(addr: u64, value: u32) {
    write_volatile(addr as *mut u32, value);
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

pub fn enable() {
    if GPU_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    
    unsafe {
        gpu_write32(crate::gpu_power_control(), 1);
        
        gpu_write32(crate::gpu_clock_control(), 1);
        
        gpu_write32(crate::gpu_reset_control(), 0);
        
        gpu_write32(crate::gpu_shader_cores_enable(), 0xFF);
        
        gpu_write32(crate::gpu_power_domain_0(), 1);
        gpu_write32(crate::gpu_power_domain_1(), 1);
        gpu_write32(crate::gpu_power_domain_2(), 1);
        gpu_write32(crate::gpu_power_domain_3(), 1);
    }
    
    GPU_ENABLED.store(true, Ordering::SeqCst);
}

pub fn init() -> Result<(), &'static str> {
    enable();
    Ok(())
}

pub fn disable() {
    if !GPU_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    
    unsafe {
        gpu_write32(crate::gpu_reset_control(), 1);
        
        gpu_write32(crate::gpu_clock_control(), 0);
        
        gpu_write32(crate::gpu_power_control(), 0);
        
        gpu_write32(crate::gpu_shader_cores_enable(), 0);
        
        gpu_write32(crate::gpu_power_domain_0(), 0);
        gpu_write32(crate::gpu_power_domain_1(), 0);
        gpu_write32(crate::gpu_power_domain_2(), 0);
        gpu_write32(crate::gpu_power_domain_3(), 0);
    }
    
    GPU_ENABLED.store(false, Ordering::SeqCst);
}

pub fn is_enabled() -> bool {
    GPU_ENABLED.load(Ordering::SeqCst)
}

pub fn hard_reset() {
    unsafe {
        gpu_write32(crate::gpu_reset_control(), 1);
        
        for _ in 0..1000 {
            core::hint::spin_loop();
        }
        
        gpu_write32(crate::gpu_reset_control(), 0);
    }
}

pub fn send_command(cmd: u32) {
    if !GPU_ENABLED.load(Ordering::SeqCst) {
        return;
    }
    
    unsafe {
        gpu_write32(crate::gpu_command_reg(), cmd);
    }
}

pub fn read_status() -> u32 {
    unsafe { gpu_read32(crate::gpu_status_reg()) }
}

pub fn secure_lock() {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

pub fn emergency_halt() {
    unsafe {
        gpu_write32(crate::gpu_command_reg(), 0);
        gpu_write32(crate::gpu_shader_cores_enable(), 0);
    }
}

pub fn set_frequency(mhz: u32) -> Result<(), &'static str> {
    if !GPU_ENABLED.load(Ordering::SeqCst) {
        return Err("gpu_disabled");
    }
    
    let clamped = if mhz > GpuSpec::MAX_FREQ_MHZ {
        GpuSpec::MAX_FREQ_MHZ
    } else if mhz < GpuSpec::MIN_FREQ_MHZ {
        GpuSpec::MIN_FREQ_MHZ
    } else {
        mhz
    };
    
    unsafe {
        gpu_write32(crate::gpu_frequency_reg(), clamped);
    }
    
    Ok(())
}

pub fn get_frequency() -> u32 {
    if !GPU_ENABLED.load(Ordering::SeqCst) {
        return 0;
    }
    
    unsafe { gpu_read32(crate::gpu_frequency_reg()) }
}

pub fn get_interrupt_status() -> u32 {
    unsafe { gpu_read32(crate::gpu_interrupt_status()) }
}

pub fn mask_interrupts(mask: u32) {
    unsafe {
        gpu_write32(crate::gpu_interrupt_mask(), mask);
    }
}

pub fn get_cores_status() -> u32 {
    unsafe { gpu_read32(crate::gpu_cores_status()) }
}

pub fn is_power_domain_enabled(domain: u32) -> bool {
    let status = unsafe {
        match domain {
            0 => gpu_read32(crate::gpu_power_domain_0()),
            1 => gpu_read32(crate::gpu_power_domain_1()),
            2 => gpu_read32(crate::gpu_power_domain_2()),
            3 => gpu_read32(crate::gpu_power_domain_3()),
            _ => 0,
        }
    };
    status != 0
}

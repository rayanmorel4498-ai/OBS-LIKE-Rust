use core::sync::atomic::{AtomicU32, Ordering};
static GPU_POWER_STATE: AtomicU32 = AtomicU32::new(0);
pub enum PowerState {
    Off = 0,
    Idle = 1,
    Active = 2,
    Turbo = 3,
}
pub struct GPUPower;
impl GPUPower {
    pub fn enable() {
        unsafe {
            core::ptr::write_volatile(crate::gpu_power_ctrl() as *mut u32, 0x01);
        }
        GPU_POWER_STATE.store(PowerState::Active as u32, Ordering::SeqCst);
    }
    pub fn disable() {
        unsafe {
            core::ptr::write_volatile(crate::gpu_power_ctrl() as *mut u32, 0x00);
        }
        GPU_POWER_STATE.store(PowerState::Off as u32, Ordering::SeqCst);
    }
    pub fn set_turbo_mode() {
        unsafe {
            core::ptr::write_volatile(crate::gpu_power_ctrl() as *mut u32, 0x03);
        }
        GPU_POWER_STATE.store(PowerState::Turbo as u32, Ordering::SeqCst);
    }
    pub fn set_idle_mode() {
        unsafe {
            core::ptr::write_volatile(crate::gpu_power_ctrl() as *mut u32, 0x01);
        }
        GPU_POWER_STATE.store(PowerState::Idle as u32, Ordering::SeqCst);
    }
    pub fn get_power_state() -> u32 {
        GPU_POWER_STATE.load(Ordering::SeqCst)
    }
    pub fn read_status() -> u32 {
        unsafe {
            core::ptr::read_volatile(crate::gpu_power_status() as *const u32)
        }
    }
    pub fn measure_consumption() -> u32 {
        unsafe {
            core::ptr::read_volatile((crate::gpu_power_status() + 0x08) as *const u32)
        }
    }
}

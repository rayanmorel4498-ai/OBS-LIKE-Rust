use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, Ordering};
static CMD_READY: AtomicBool = AtomicBool::new(true);
pub fn send_command(cmd: u32) -> bool {
    if !CMD_READY.load(Ordering::SeqCst) {
        return false;
    }
    unsafe {
        write_volatile(crate::gpu_cmd_base() as *mut u32, cmd);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    true
}
pub fn status() -> u32 {
    unsafe { read_volatile(crate::gpu_cmd_status() as *const u32) }
}
pub fn fence() -> u32 {
    let f = unsafe { read_volatile(crate::gpu_cmd_fence() as *const u32) };
    unsafe {
        write_volatile(crate::gpu_cmd_fence() as *mut u32, f + 1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    f
}
pub fn wait_fence(target: u32) {
    while unsafe { read_volatile(crate::gpu_cmd_fence() as *const u32) } < target {
        #[cfg(target_arch = "aarch64")]
        unsafe { asm!("wfi") };
        #[cfg(not(target_arch = "aarch64"))]
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
pub fn reset_queue() {
    unsafe {
        write_volatile(crate::gpu_cmd_base() as *mut u32, 0);
        write_volatile(crate::gpu_cmd_fence() as *mut u32, 0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    CMD_READY.store(true, Ordering::SeqCst);
}
pub fn lock_queue() {
    CMD_READY.store(false, Ordering::SeqCst);
}
pub fn unlock_queue() {
    CMD_READY.store(true, Ordering::SeqCst);
}
pub fn submit_command(cmd: u64) -> Result<(), &'static str> {
    if send_command(cmd as u32) {
        Ok(())
    } else {
        Err("command_queue_busy")
    }
}
use core::ptr::{read_volatile, write_volatile};

pub fn enable_auto_refresh() -> Result<(), &'static str> {
    unsafe {
        let val = read_volatile(crate::refresh_ctrl() as *const u32);
        write_volatile(crate::refresh_ctrl() as *mut u32, val | 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable_auto_refresh() -> Result<(), &'static str> {
    unsafe {
        let val = read_volatile(crate::refresh_ctrl() as *const u32);
        write_volatile(crate::refresh_ctrl() as *mut u32, val & !0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn trigger_refresh() -> Result<(), &'static str> {
    unsafe {
        let val = read_volatile(crate::refresh_ctrl() as *const u32);
        write_volatile(crate::refresh_ctrl() as *mut u32, val | 0x2);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        write_volatile(crate::refresh_ctrl() as *mut u32, val & !0x2);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn status() -> u32 {
    unsafe { 
        read_volatile(crate::refresh_status() as *const u32)
    }
}

pub fn get_timer() -> u32 {
    unsafe {
        read_volatile(crate::refresh_timer() as *const u32)
    }
}

pub fn set_interval(cycles: u32) -> Result<(), &'static str> {
    if cycles == 0 {
        return Err("interval_zero");
    }
    if cycles > 0xFFFF {
        return Err("interval_out_of_range");
    }
    unsafe {
        let val = read_volatile(crate::refresh_ctrl() as *const u32);
        write_volatile(crate::refresh_ctrl() as *mut u32, (val & 0xFF) | (cycles << 8));
        write_volatile(crate::refresh_interval() as *mut u32, cycles);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_interval() -> u32 {
    unsafe {
        (read_volatile(crate::refresh_ctrl() as *const u32) >> 8) & 0xFFFF
    }
}

pub fn is_auto_refresh_enabled() -> bool {
    unsafe {
        (read_volatile(crate::refresh_ctrl() as *const u32) & 0x1) != 0
    }
}
pub fn start_refresh() -> Result<(), &'static str> {
    enable_auto_refresh()
}

pub fn stop_refresh() -> Result<(), &'static str> {
    disable_auto_refresh()
}
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicBool, Ordering};

static RAM_LOCKED: AtomicBool = AtomicBool::new(false);

pub fn lock_down() -> Result<(), &'static str> {
    if RAM_LOCKED.load(Ordering::SeqCst) {
        return Ok(());
    }
    unsafe {
        let val = read_volatile(crate::memc_lock_ctrl() as *const u32);
        write_volatile(crate::memc_lock_ctrl() as *mut u32, val | 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    RAM_LOCKED.store(true, Ordering::SeqCst);
    Ok(())
}

pub fn is_locked() -> bool {
    RAM_LOCKED.load(Ordering::SeqCst)
}

pub fn disable_external_debug() -> Result<(), &'static str> {
    unsafe {
        let val = read_volatile(crate::memc_debug_ctrl() as *const u32);
        write_volatile(crate::memc_debug_ctrl() as *mut u32, val | 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn secure_erase() -> Result<(), &'static str> {
    if RAM_LOCKED.load(Ordering::SeqCst) {
        return Err("ram_locked");
    }
    unsafe {
        let val = read_volatile(crate::memc_erase_ctrl() as *const u32);
        write_volatile(crate::memc_erase_ctrl() as *mut u32, val | 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        
        let timeout = 10000;
        let mut count = 0;
        while count < timeout {
            let status = read_volatile(crate::memc_erase_ctrl() as *const u32);
            if (status & 0x2) != 0 {
                break;
            }
            count += 1;
        }
        
        if count >= timeout {
            return Err("erase_timeout");
        }
        
        write_volatile(crate::memc_erase_ctrl() as *mut u32, val & !0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn permanent_lock() -> Result<(), &'static str> {
    if RAM_LOCKED.load(Ordering::SeqCst) {
        return Ok(());
    }
    unsafe {
        let val = read_volatile(crate::memc_lock_ctrl() as *const u32);
        write_volatile(crate::memc_lock_ctrl() as *mut u32, val | 0x4);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    RAM_LOCKED.store(true, Ordering::SeqCst);
    Ok(())
}

pub fn runtime_integrity_check() -> Result<u32, &'static str> {
    let status = unsafe {
        read_volatile(crate::phy_security_status() as *const u32)
    };
    
    if (status & 0x1) != 0 {
        Ok(status)
    } else {
        Err("integrity_check_failed")
    }
}

pub fn get_security_status() -> u32 {
    unsafe {
        read_volatile(crate::phy_security_status() as *const u32)
    }
}

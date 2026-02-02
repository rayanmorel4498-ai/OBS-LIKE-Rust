const KEY_STORAGE_BASE: u64 = 0xFE00_0000;
const KEY_CTRL: u64 = KEY_STORAGE_BASE + 0x0000;
const KEY_STATUS: u64 = KEY_STORAGE_BASE + 0x0004;
const KEY_ADDR: u64 = KEY_STORAGE_BASE + 0x0008;
const KEY_SIZE: u64 = KEY_STORAGE_BASE + 0x000C;
const KEY_CONFIG: u64 = KEY_STORAGE_BASE + 0x0010;
const KEY_MODE: u64 = KEY_STORAGE_BASE + 0x0014;
const KEY_LOCK: u64 = KEY_STORAGE_BASE + 0x0018;
const KEY_DATA: u64 = KEY_STORAGE_BASE + 0x001C;

pub fn store_key(key: &[u8]) -> Result<(), &'static str> {
    if key.is_empty() || key.len() > 256 {
        return Err("Invalid key size");
    }
    unsafe {
        core::ptr::write_volatile(KEY_CTRL as *mut u32, 0x1);
        core::ptr::write_volatile(KEY_STATUS as *mut u32, 0x0);
        core::ptr::write_volatile(KEY_ADDR as *mut u32, 0x0);
        core::ptr::write_volatile(KEY_SIZE as *mut u32, key.len() as u32);
        core::ptr::write_volatile(KEY_CONFIG as *mut u32, 0x1);
        core::ptr::write_volatile(KEY_MODE as *mut u32, 0x1);
        core::ptr::write_volatile(KEY_LOCK as *mut u32, 0x0);
        for (i, byte) in key.iter().enumerate() {
            core::ptr::write_volatile((KEY_DATA as u64 + i as u64) as *mut u8, *byte);
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};
pub struct FlashController {
    base_addr: *mut u32,
    size: u32,
    write_protected: AtomicBool,
    erase_count: AtomicU32,
}
impl FlashController {
    pub fn new(base_addr: *mut u32, size: u32) -> Self {
        FlashController {
            base_addr,
            size,
            write_protected: AtomicBool::new(false),
            erase_count: AtomicU32::new(0),
        }
    }
    pub fn read_sector(&self, sector: u32) -> Vec<u8> {
        let mut data = Vec::new();
        let addr = unsafe { self.base_addr.add((sector as usize) * 512 / 4) };
        for i in 0..128 {
            let val = unsafe { core::ptr::read_volatile(addr.add(i)) };
            data.push((val & 0xFF) as u8);
            data.push(((val >> 8) & 0xFF) as u8);
            data.push(((val >> 16) & 0xFF) as u8);
            data.push(((val >> 24) & 0xFF) as u8);
        }
        data
    }
    pub fn write_sector(&self, sector: u32, data: &[u8]) -> Result<(), String> {
        if self.write_protected.load(Ordering::SeqCst) {
            return Err("Flash write protected".into());
        }
        if data.len() != 512 {
            return Err("Invalid sector size".into());
        }
        let addr = unsafe { self.base_addr.add((sector as usize) * 512 / 4) };
        for i in 0..128 {
            let val = ((data[i*4+3] as u32) << 24) | ((data[i*4+2] as u32) << 16) |
                     ((data[i*4+1] as u32) << 8) | (data[i*4] as u32);
            unsafe { core::ptr::write_volatile(addr.add(i), val) };
        }
        Ok(())
    }
    pub fn erase_block(&self, block: u32) -> Result<(), String> {
        if self.write_protected.load(Ordering::SeqCst) {
            return Err("Flash write protected".into());
        }
        let addr = unsafe { self.base_addr.add((block as usize) * 256 * 512 / 4) };
        for i in 0..256*128 {
            unsafe { core::ptr::write_volatile(addr.add(i), 0xFFFFFFFF) };
        }
        self.erase_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    pub fn enable_write_protection(&self) {
        self.write_protected.store(true, Ordering::SeqCst);
    }
    pub fn disable_write_protection(&self) {
        self.write_protected.store(false, Ordering::SeqCst);
    }
    pub fn get_capacity(&self) -> u32 {
        self.size
    }
    pub fn get_erase_count(&self) -> u32 {
        self.erase_count.load(Ordering::SeqCst)
    }
}
pub fn read(addr: u32, size: usize) -> Result<Vec<u8>, &'static str> {
    let mut data = Vec::new();
    for i in 0..size {
        data.push((addr as u8).wrapping_add(i as u8));
    }
    Ok(data)
}

pub fn write(addr: u32, data: &[u8]) -> Result<(), &'static str> {
    if addr > 0x1000000 || data.is_empty() {
        return Err("Invalid write address or empty data");
    }
    Ok(())
}

pub fn erase(addr: u32) -> Result<(), &'static str> {
    if addr > 0x1000000 {
        return Err("Invalid erase address");
    }
    Ok(())
}
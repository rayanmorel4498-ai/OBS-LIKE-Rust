use core::ptr::{read_volatile, write_volatile};

fn nfc_command_reg() -> u64 { crate::nfc_command_reg() }
fn nfc_response_reg() -> u64 { crate::nfc_response_reg() }
fn nfc_fifo_reg() -> u64 { crate::nfc_fifo_reg() }
fn nfc_status_reg() -> u64 { crate::nfc_status_reg() }
fn reader_config_reg() -> u64 { crate::reader_config_reg() }
fn reader_detect_reg() -> u64 { crate::reader_detect_reg() }
fn uid_reg() -> u64 { crate::uid_reg() }
fn whitelist_reg() -> u64 { crate::whitelist_reg() }

pub struct NFCReader;

impl NFCReader {
    pub fn new() -> Self {
        NFCReader
    }

    pub fn init() -> Result<(), &'static str> {
        unsafe {
            write_volatile(reader_config_reg() as *mut u32, 0x1);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn is_tag_present() -> Result<bool, &'static str> {
        let status = unsafe {
            read_volatile(nfc_status_reg() as *const u32)
        };
        Ok((status & 0x1) != 0)
    }

    pub fn read_tag_uid() -> Result<u64, &'static str> {
        if !Self::is_tag_present()? {
            return Err("no_tag_present");
        }
        let uid = unsafe {
            read_volatile(uid_reg() as *const u64)
        };
        Ok(uid)
    }

    pub fn send_command(cmd: u32) -> Result<(), &'static str> {
        unsafe {
            write_volatile(nfc_command_reg() as *mut u32, cmd);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn get_response() -> u32 {
        unsafe {
            read_volatile(nfc_response_reg() as *const u32)
        }
    }

    pub fn read_fifo() -> u32 {
        unsafe {
            read_volatile(nfc_fifo_reg() as *const u32)
        }
    }

    pub fn set_whitelist(addr_offset: u64, value: u64) -> Result<(), &'static str> {
        if addr_offset > 32 {
            return Err("whitelist_offset_too_large");
        }
        unsafe {
            write_volatile((whitelist_reg() + addr_offset) as *mut u64, value);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn get_detect_status() -> u32 {
        unsafe {
            read_volatile(reader_detect_reg() as *const u32)
        }
    }

    pub fn read_ndef(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let status = unsafe {
            read_volatile(nfc_status_reg() as *const u32)
        };
        if (status & 0x2) == 0 {
            return Ok(0);
        }
        Ok(buffer.len())
    }
}
pub fn enable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(reader_config_reg() as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn disable() -> Result<(), &'static str> {
    unsafe {
        write_volatile(reader_config_reg() as *mut u32, 0x0);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn scan_card() -> Result<(), &'static str> {
    unsafe {
        write_volatile(nfc_command_reg() as *mut u32, 0x1);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    Ok(())
}

pub fn get_uid() -> Result<u32, &'static str> {
    unsafe {
        Ok(read_volatile(uid_reg() as *const u32))
    }
}
use core::ptr::{read_volatile, write_volatile};

fn nfc_command_reg() -> u64 { crate::nfc_command_reg() }
fn nfc_fifo_reg() -> u64 { crate::nfc_fifo_reg() }
fn nfc_status_reg() -> u64 { crate::nfc_status_reg() }
fn writer_config_reg() -> u64 { crate::writer_config_reg() }
fn writer_erase_reg() -> u64 { crate::writer_erase_reg() }
fn write_data_reg() -> u64 { crate::write_data_reg() }
fn write_addr_reg() -> u64 { crate::write_addr_reg() }

pub struct NFCWriter;

impl NFCWriter {
    pub fn new() -> Self {
        NFCWriter
    }

    pub fn init() -> Result<(), &'static str> {
        unsafe {
            write_volatile(writer_config_reg() as *mut u32, 0x1);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn write_page(page: u32, data: u32) -> Result<(), &'static str> {
        if page > 0xFFFF {
            return Err("page_out_of_range");
        }
        unsafe {
            write_volatile(write_addr_reg() as *mut u32, page);
            write_volatile(write_data_reg() as *mut u32, data);
            write_volatile(nfc_command_reg() as *mut u32, 0x2);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            
            let mut timeout = 1000;
            while timeout > 0 {
                let status = read_volatile(nfc_status_reg() as *const u32);
                if (status & 0x2) != 0 {
                    break;
                }
                timeout -= 1;
            }
            
            if timeout == 0 {
                return Err("write_timeout");
            }
        }
        Ok(())
    }

    pub fn erase_all() -> Result<(), &'static str> {
        unsafe {
            write_volatile(writer_erase_reg() as *mut u32, 0x1);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            
            let mut timeout = 5000;
            while timeout > 0 {
                let status = read_volatile(nfc_status_reg() as *const u32);
                if (status & 0x4) != 0 {
                    break;
                }
                timeout -= 1;
            }
            
            if timeout == 0 {
                return Err("erase_timeout");
            }
        }
        Ok(())
    }

    pub fn write_ndef(data: &[u8]) -> Result<(), &'static str> {
        if data.len() > 4096 {
            return Err("data_too_large");
        }
        for (i, chunk) in data.chunks(4).enumerate() {
            let mut word: u32 = 0;
            for (j, &byte) in chunk.iter().enumerate() {
                word |= (byte as u32) << (j * 8);
            }
            Self::write_page(i as u32, word)?;
        }
        Ok(())
    }

    pub fn is_write_ready() -> bool {
        let status = unsafe {
            read_volatile(nfc_status_reg() as *const u32)
        };
        (status & 0x8) != 0
    }

    pub fn get_erase_status() -> u32 {
        unsafe {
            read_volatile(writer_erase_reg() as *const u32)
        }
    }

    pub fn write_fifo(data: u32) -> Result<(), &'static str> {
        unsafe {
            write_volatile(nfc_fifo_reg() as *mut u32, data);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn read_fifo() -> u32 {
        unsafe {
            read_volatile(nfc_fifo_reg() as *const u32)
        }
    }
}

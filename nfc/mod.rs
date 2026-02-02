pub mod reader;
pub mod writer;
pub mod payment;

pub use reader::NFCReader;
pub use writer::NFCWriter;
pub use payment::NFCPayment;

fn nfc_ctrl_reg() -> u64 { crate::nfc_ctrl_reg() }
fn nfc_status_reg() -> u64 { crate::nfc_status_reg() }
fn nfc_interrupt_reg() -> u64 { crate::nfc_interrupt_reg() }
fn nfc_error_reg() -> u64 { crate::nfc_error_reg() }
fn nfc_command_reg() -> u64 { crate::nfc_command_reg() }
fn nfc_response_reg() -> u64 { crate::nfc_response_reg() }
fn nfc_fifo_reg() -> u64 { crate::nfc_fifo_reg() }
fn nfc_timeout_reg() -> u64 { crate::nfc_timeout_reg() }
fn nfc_config_reg() -> u64 { crate::nfc_config_reg() }
fn nfc_mode_reg() -> u64 { crate::nfc_mode_reg() }

use core::ptr::{read_volatile, write_volatile};

pub struct NFCController {
    enabled: bool,
}

impl NFCController {
    pub fn new() -> Self {
        NFCController { enabled: true }
    }

    pub fn enable(&mut self) -> Result<(), &'static str> {
        unsafe {
            let ctrl = read_volatile(nfc_ctrl_reg() as *const u32);
            write_volatile(nfc_ctrl_reg() as *mut u32, ctrl | 0x1);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        self.enabled = true;
        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), &'static str> {
        unsafe {
            let ctrl = read_volatile(nfc_ctrl_reg() as *const u32);
            write_volatile(nfc_ctrl_reg() as *mut u32, ctrl & !0x1);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        self.enabled = false;
        Ok(())
    }

    pub fn get_status() -> u32 {
        unsafe {
            read_volatile(nfc_status_reg() as *const u32)
        }
    }

    pub fn reset() -> Result<(), &'static str> {
        unsafe {
            write_volatile(nfc_ctrl_reg() as *mut u32, 0x2);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
            let mut timeout = 1000;
            while timeout > 0 {
                let status = read_volatile(nfc_status_reg() as *const u32);
                if (status & 0x4) != 0 {
                    break;
                }
                timeout -= 1;
            }
            if timeout == 0 {
                return Err("nfc_reset_timeout");
            }
        }
        Ok(())
    }

    pub fn get_interrupt_status() -> u32 {
        unsafe {
            read_volatile(nfc_interrupt_reg() as *const u32)
        }
    }

    pub fn clear_interrupt() -> Result<(), &'static str> {
        unsafe {
            write_volatile(nfc_interrupt_reg() as *mut u32, 0xFFFFFFFF);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn get_error() -> u32 {
        unsafe {
            read_volatile(nfc_error_reg() as *const u32)
        }
    }

    pub fn set_timeout(ms: u32) -> Result<(), &'static str> {
        if ms > 5000 {
            return Err("timeout_too_long");
        }
        unsafe {
            write_volatile(nfc_timeout_reg() as *mut u32, ms);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn set_mode(mode: u32) -> Result<(), &'static str> {
        if mode > 3 {
            return Err("invalid_nfc_mode");
        }
        unsafe {
            write_volatile(nfc_mode_reg() as *mut u32, mode);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn get_config() -> u32 {
        unsafe {
            read_volatile(nfc_config_reg() as *const u32)
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nfc_controller_init() {
        let nfc = NFCController::new();
        assert!(nfc.is_enabled());
    }
    #[test]
    fn test_nfc_enable_disable() {
        let mut nfc = NFCController::new();
        nfc.disable();
        assert!(!nfc.is_enabled());
        nfc.enable();
        assert!(nfc.is_enabled());
    }
}

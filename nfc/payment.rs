use core::ptr::{read_volatile, write_volatile};

fn payment_ctrl_reg() -> u64 { crate::payment_ctrl_reg() }
fn payment_status_reg() -> u64 { crate::payment_status_reg() }
fn payment_amount_reg() -> u64 { crate::payment_amount_reg() }
fn payment_currency_reg() -> u64 { crate::payment_currency_reg() }
fn payment_security_reg() -> u64 { crate::payment_security_reg() }
fn payment_log_reg() -> u64 { crate::payment_log_reg() }
fn payment_config_reg() -> u64 { crate::payment_config_reg() }

pub struct NFCPayment;

impl NFCPayment {
    pub fn new() -> Self {
        NFCPayment
    }

    pub fn init() -> Result<(), &'static str> {
        unsafe {
            write_volatile(payment_ctrl_reg() as *mut u32, 0x1);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn enable() -> Result<(), &'static str> {
        unsafe {
            let ctrl = read_volatile(payment_ctrl_reg() as *const u32);
            write_volatile(payment_ctrl_reg() as *mut u32, ctrl | 0x1);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn disable() -> Result<(), &'static str> {
        unsafe {
            let ctrl = read_volatile(payment_ctrl_reg() as *const u32);
            write_volatile(payment_ctrl_reg() as *mut u32, ctrl & !0x1);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn send_transaction(amount: u32, currency: u8) -> Result<(), &'static str> {
        if amount == 0 {
            return Err("amount_cannot_be_zero");
        }
        if amount > 999999 {
            return Err("amount_exceeds_limit");
        }
        
        unsafe {
            write_volatile(payment_amount_reg() as *mut u32, amount);
            write_volatile(payment_currency_reg() as *mut u32, currency as u32);
            let ctrl = read_volatile(payment_ctrl_reg() as *const u32);
            write_volatile(payment_ctrl_reg() as *mut u32, ctrl | 0x2);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn check_status() -> u32 {
        unsafe {
            read_volatile(payment_status_reg() as *const u32)
        }
    }

    pub fn verify_security() -> Result<bool, &'static str> {
        let sec = unsafe {
            read_volatile(payment_security_reg() as *const u32)
        };
        Ok((sec & 0x1) != 0)
    }

    pub fn get_transaction_log() -> u32 {
        unsafe {
            read_volatile(payment_log_reg() as *const u32)
        }
    }

    pub fn set_config(config: u32) -> Result<(), &'static str> {
        unsafe {
            write_volatile(payment_config_reg() as *mut u32, config);
            core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        }
        Ok(())
    }

    pub fn get_config() -> u32 {
        unsafe {
            read_volatile(payment_config_reg() as *const u32)
        }
    }
}

pub fn enable() -> Result<(), &'static str> {
    NFCPayment::enable()
}

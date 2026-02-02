extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU8, Ordering};

pub struct TrustedExecutionEnvironment {
    state: AtomicU8,
    #[allow(dead_code)]
    secure_memory_base: AtomicU32,
    #[allow(dead_code)]
    secure_memory_size: AtomicU32,
    tee_enabled: AtomicBool,
    app_count: AtomicU32,
    code_loaded: AtomicBool,
}
impl TrustedExecutionEnvironment {
    pub fn new() -> Self {
        TrustedExecutionEnvironment {
            state: AtomicU8::new(0),
            secure_memory_base: AtomicU32::new(0x0F00_0000),
            secure_memory_size: AtomicU32::new(256 * 1024 * 1024),
            tee_enabled: AtomicBool::new(false),
            app_count: AtomicU32::new(0),
            code_loaded: AtomicBool::new(false),
        }
    }
    pub fn initialize(&self) -> Result<(), String> {
        if self.state.load(Ordering::SeqCst) != 0 {
            return Err("TEE already initialized".into());
        }
        self.state.store(1, Ordering::SeqCst);
        self.tee_enabled.store(true, Ordering::SeqCst);
        self.state.store(2, Ordering::SeqCst);
        Ok(())
    }
    pub fn load_secure_code(&self, code: Vec<u8>) -> Result<(), String> {
        if !self.tee_enabled.load(Ordering::SeqCst) {
            return Err(String::from("TEE disabled"));
        }
        if code.len() > 16 * 1024 * 1024 {
            return Err(String::from("Code too large"));
        }
        let current = self.app_count.fetch_add(1, Ordering::SeqCst);
        if current >= 16 {
            return Err(String::from("Max apps"));
        }
        self.code_loaded.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn attestation_quote(&self) -> Result<Vec<u8>, String> {
        if !self.code_loaded.load(Ordering::SeqCst) {
            return Err(String::from("No code loaded"));
        }
        Ok(Vec::new())
    }
    pub fn is_code_loaded(&self) -> bool {
        self.code_loaded.load(Ordering::SeqCst)
    }
}
impl Default for TrustedExecutionEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

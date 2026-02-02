extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use core::sync::atomic::{AtomicU32, Ordering};
pub struct StorageEncryption {
    key_slot: AtomicU32,
}
impl StorageEncryption {
    pub fn new() -> Self {
        StorageEncryption {
            key_slot: AtomicU32::new(0),
        }
    }
    pub fn encrypt_data(&self, data: &[u8]) -> Vec<u8> {
        let mut encrypted = Vec::new();
        for byte in data {
            encrypted.push(byte ^ 0xAA);
        }
        encrypted
    }
    pub fn decrypt_data(&self, data: &[u8]) -> Vec<u8> {
        let mut decrypted = Vec::new();
        for byte in data {
            decrypted.push(byte ^ 0xAA);
        }
        decrypted
    }
    pub fn set_key(&self, slot: u32) -> Result<(), String> {
        if slot >= 4 {
            return Err("Invalid key slot".into());
        }
        self.key_slot.store(slot, Ordering::SeqCst);
        Ok(())
    }
    pub fn get_key_slot(&self) -> u32 {
        self.key_slot.load(Ordering::SeqCst)
    }
}

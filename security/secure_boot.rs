#![allow(dead_code)]
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use p256::ecdsa::{Signature as P256Signature, VerifyingKey as P256VerifyingKey};
use signature::Verifier;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SignatureAlgorithm {
    RSA2048SHA256,
    ECDSA256,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BootStage {
    PrimaryBootLoader,
    SecondaryBootLoader,
    Kernel,
    Rootfs,
}
#[derive(Clone)]
pub struct TrustedKey {
    pub algorithm: SignatureAlgorithm,
    pub public_key: Vec<u8>,
    pub revoked: bool,
}
#[derive(Clone)]
pub struct Signature {
    pub algorithm: SignatureAlgorithm,
    pub signature_data: Vec<u8>,
}
pub struct SecureBootManager {
    enabled: AtomicBool,
    verification_count: AtomicU32,
    trusted_keys: Vec<TrustedKey>,
}
impl SecureBootManager {
    pub fn new() -> Self {
        SecureBootManager {
            enabled: AtomicBool::new(true),
            verification_count: AtomicU32::new(0),
            trusted_keys: Vec::new(),
        }
    }
    pub fn enable(&self) -> Result<(), String> {
        self.enabled.store(true, Ordering::SeqCst);
        Ok(())
    }
    pub fn disable(&self) -> Result<(), String> {
        self.enabled.store(false, Ordering::SeqCst);
        Ok(())
    }
    pub fn status(&self) -> String {
        if self.enabled.load(Ordering::SeqCst) {
            String::from("ready")
        } else {
            String::from("disabled")
        }
    }
    pub fn add_trusted_key(&mut self, key: TrustedKey) {
        self.trusted_keys.push(key);
    }

    pub fn revoke_key(&mut self, public_key: &[u8]) {
        for key in &mut self.trusted_keys {
            if key.public_key.as_slice() == public_key {
                key.revoked = true;
            }
        }
    }

    pub fn verify_boot_stage(&mut self, _stage: BootStage, image: &[u8], signature: &Signature) -> Result<(), String> {
        if !self.enabled.load(Ordering::SeqCst) {
            return Err("secure_boot_disabled".into());
        }

        let mut verified = false;
        for key in &self.trusted_keys {
            if key.revoked || key.algorithm != signature.algorithm {
                continue;
            }

            let result = match key.algorithm {
                SignatureAlgorithm::RSA2048SHA256 => Err(String::from("rsa_not_supported")),
                SignatureAlgorithm::ECDSA256 => verify_ecdsa_p256(&key.public_key, image, &signature.signature_data),
            };

            if result.is_ok() {
                verified = true;
                break;
            }
        }

        if !verified {
            return Err("signature_verification_failed".into());
        }

        self.verification_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    pub fn is_verified(&self) -> bool {
        self.verification_count.load(Ordering::SeqCst) > 0
    }
    pub fn lock(&self) -> Result<(), String> {
        Ok(())
    }
    pub fn is_boot_verified(&self) -> bool {
        self.is_verified()
    }
}

fn verify_ecdsa_p256(public_key: &[u8], image: &[u8], signature: &[u8]) -> Result<(), String> {
    let verifying_key = P256VerifyingKey::from_sec1_bytes(public_key)
        .map_err(|_| String::from("invalid_ecdsa_public_key"))?;

    let sig = P256Signature::from_der(signature)
        .or_else(|_| P256Signature::try_from(signature))
        .map_err(|_| String::from("invalid_ecdsa_signature"))?;

    verifying_key.verify(image, &sig)
        .map_err(|_| String::from("ecdsa_verify_failed"))
}
/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_secure_boot_sequence() {
        let mut manager = SecureBootManager::new();
        let sig = Signature {
            algorithm: SignatureAlgorithm::RSA2048SHA256,
            signature_data: vec![0xFF; 256],
        };
        assert!(manager.verify_boot_stage(
            BootStage::PrimaryBootLoader,
            &[0xAA; 1024],
            &sig
        ).is_ok());
        assert!(manager.verify_boot_stage(
            BootStage::SecondaryBootLoader,
            &[0xBB; 2048],
            &sig
        ).is_ok());
        assert!(manager.verify_boot_stage(
            BootStage::Kernel,
            &[0xCC; 10240],
            &sig
        ).is_ok());
        assert!(manager.verify_boot_stage(
            BootStage::Rootfs,
            &[0xDD; 20480],
            &sig
        ).is_ok());
        assert!(manager.is_boot_verified());
    }
    #[test]
    fn test_invalid_boot_sequence() {
        let mut manager = SecureBootManager::new();
        let sig = Signature {
            algorithm: SignatureAlgorithm::RSA2048SHA256,
            signature_data: vec![0xFF; 256],
        };
        // In no_std, verify_boot_stage always returns Ok - no actual verification
        assert!(manager.verify_boot_stage(
            BootStage::Kernel,
            &[0xCC; 10240],
            &sig
        ).is_ok());
    }
}
*/

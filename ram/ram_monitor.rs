extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};
pub struct RAMMonitor {
    total_memory: u32,
    used_memory: AtomicU32,
    peak_usage: AtomicU32,
    refresh_rate: AtomicU32,
}
impl RAMMonitor {
    pub fn new(total: u32) -> Self {
        RAMMonitor {
            total_memory: total,
            used_memory: AtomicU32::new(0),
            peak_usage: AtomicU32::new(0),
            refresh_rate: AtomicU32::new(1000),
        }
    }
    pub fn get_memory_snapshot(&self) -> Vec<u32> {
        // Use alloc::vec::Vec to collect memory statistics
        let mut snapshot = Vec::new();
        snapshot.push(self.total_memory);
        snapshot.push(self.used_memory.load(Ordering::SeqCst));
        snapshot.push(self.get_available_memory());
        snapshot.push(self.peak_usage.load(Ordering::SeqCst));
        snapshot
    }
    pub fn get_total_memory(&self) -> u32 {
        self.total_memory
    }
    pub fn get_used_memory(&self) -> u32 {
        self.used_memory.load(Ordering::SeqCst)
    }
    pub fn set_used_memory(&self, used: u32) {
        let clamped = used.min(self.total_memory);
        self.used_memory.store(clamped, Ordering::SeqCst);
        let current_peak = self.peak_usage.load(Ordering::SeqCst);
        if clamped > current_peak {
            self.peak_usage.store(clamped, Ordering::SeqCst);
        }
    }
    pub fn get_peak_usage(&self) -> u32 {
        self.peak_usage.load(Ordering::SeqCst)
    }
    pub fn get_available_memory(&self) -> u32 {
        let used = self.used_memory.load(Ordering::SeqCst);
        self.total_memory.saturating_sub(used)
    }
    pub fn get_usage_percent(&self) -> u32 {
        let used = self.used_memory.load(Ordering::SeqCst);
        if self.total_memory == 0 {
            return 0;
        }
        (used as u64 * 100 / self.total_memory as u64) as u32
    }
    pub fn reset_peak(&self) {
        self.peak_usage.store(self.used_memory.load(Ordering::SeqCst), Ordering::SeqCst);
    }
    pub fn set_refresh_rate(&self, rate_ms: u32) -> Result<(), String> {
        if rate_ms == 0 {
            return Err("refresh_rate_zero".into());
        }
        self.refresh_rate.store(rate_ms, Ordering::SeqCst);
        Ok(())
    }
    pub fn get_refresh_rate(&self) -> u32 {
        self.refresh_rate.load(Ordering::SeqCst)
    }
    pub fn get_memory_stats(&self) -> RAMStats {
        RAMStats {
            total: self.total_memory,
            used: self.used_memory.load(Ordering::SeqCst),
            available: self.get_available_memory(),
            peak: self.peak_usage.load(Ordering::SeqCst),
            usage_percent: self.get_usage_percent(),
        }
    }
    pub fn is_low_memory(&self) -> bool {
        self.get_usage_percent() > 90
    }
    pub fn is_critical_memory(&self) -> bool {
        self.get_usage_percent() > 95
    }

    pub fn try_set_used_memory(&self, used: u32) -> Result<(), &'static str> {
        if used > self.total_memory {
            return Err("used_exceeds_total");
        }
        self.set_used_memory(used);
        Ok(())
    }

    pub fn add_used_memory(&self, delta: u32) -> Result<(), &'static str> {
        loop {
            let current = self.used_memory.load(Ordering::SeqCst);
            let next = current.saturating_add(delta);
            if next > self.total_memory {
                return Err("used_exceeds_total");
            }
            if self
                .used_memory
                .compare_exchange(current, next, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                let current_peak = self.peak_usage.load(Ordering::SeqCst);
                if next > current_peak {
                    self.peak_usage.store(next, Ordering::SeqCst);
                }
                return Ok(());
            }
        }
    }
}
#[derive(Clone, Copy)]
pub struct RAMStats {
    pub total: u32,
    pub used: u32,
    pub available: u32,
    pub peak: u32,
    pub usage_percent: u32,
}
impl RAMStats {
    pub fn to_array(&self) -> [u32; 5] {
        [self.total, self.used, self.available, self.peak, self.usage_percent]
    }
}
impl Default for RAMMonitor {
    fn default() -> Self {
        Self::new(8 * 1024 * 1024)
    }
}

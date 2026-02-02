use core::ptr::{read_volatile, write_volatile};
pub struct RamTiming {
    pub t_ras: u8,
    pub t_rcd: u8,
    pub t_rp: u8,
    pub t_cas: u8,
}
impl RamTiming {
    pub const fn default() -> Self {
        Self {
            t_ras: 35,
            t_rcd: 15,
            t_rp: 15,
            t_cas: 15,
        }
    }
}
pub fn apply(t: &RamTiming) {
    unsafe {
        let value: u32 = ((t.t_ras as u32) << 24)
                       | ((t.t_rcd as u32) << 16)
                       | ((t.t_rp as u32) << 8)
                       | (t.t_cas as u32);
        write_volatile(crate::ram_timing_ctrl() as *mut u32, value);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
pub fn read_current() -> RamTiming {
    unsafe {
        let val = read_volatile(crate::ram_timing_ctrl() as *const u32);
        RamTiming {
            t_ras: ((val >> 24) & 0xFF) as u8,
            t_rcd: ((val >> 16) & 0xFF) as u8,
            t_rp: ((val >> 8) & 0xFF) as u8,
            t_cas: (val & 0xFF) as u8,
        }
    }
}
pub fn low_power_mode() {
    let mut t = read_current();
    t.t_ras = t.t_ras.saturating_add(10);
    t.t_rcd = t.t_rcd.saturating_add(5);
    t.t_rp = t.t_rp.saturating_add(5);
    t.t_cas = t.t_cas.saturating_add(5);
    apply(&t);
}
pub fn high_perf_mode() {
    let mut t = read_current();
    t.t_ras = t.t_ras.saturating_sub(10);
    t.t_rcd = t.t_rcd.saturating_sub(5);
    t.t_rp = t.t_rp.saturating_sub(5);
    t.t_cas = t.t_cas.saturating_sub(5);
    apply(&t);
}

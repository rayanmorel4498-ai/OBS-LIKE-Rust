extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};

static ACCELEROMETER_READY: AtomicBool = AtomicBool::new(false);

pub struct AccelerometerDriver;

impl AccelerometerDriver {
    pub fn init() -> Result<(), alloc::string::String> {
        if ACCELEROMETER_READY.load(Ordering::SeqCst) {
            return Err(alloc::string::String::from("Already initialized"));
        }
        ACCELEROMETER_READY.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn is_ready() -> bool {
        ACCELEROMETER_READY.load(Ordering::SeqCst)
    }

    pub fn read(_context: &[u8]) -> Result<(i16, i16, i16), alloc::string::String> {
        if !Self::is_ready() {
            return Err(alloc::string::String::from("Not initialized"));
        }
        let x: i16 = unsafe { read_accel_x() };
        let y: i16 = unsafe { read_accel_y() };
        let z: i16 = unsafe { read_accel_z() };
        Ok((x, y, z))
    }

    pub fn shutdown() {
        if !Self::is_ready() {
            return;
        }
        ACCELEROMETER_READY.store(false, Ordering::SeqCst);
    }
}

unsafe fn read_accel_x() -> i16 {
    0
}

unsafe fn read_accel_y() -> i16 {
    0
}

unsafe fn read_accel_z() -> i16 {
    0
}

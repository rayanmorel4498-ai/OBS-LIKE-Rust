// visualisation_module/src/utils/timers.rs

use std::time::{Duration, Instant};

pub struct Interval {
    last: Instant,
    period: Duration,
}

impl Interval {
    pub fn new(period: Duration) -> Self {
        Self {
            last: Instant::now(),
            period,
        }
    }

    #[inline]
    pub fn ready(&mut self) -> bool {
        if self.last.elapsed() >= self.period {
            self.last = Instant::now();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        self.last = Instant::now();
    }

    #[inline]
    pub fn set_period(&mut self, period: Duration) {
        self.period = period;
    }
}

/// Mesure précise de durée (sans allocation)
pub fn elapsed_since(t: Instant) -> Duration {
    Instant::now().duration_since(t)
}

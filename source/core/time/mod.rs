//! Shared time state for syscall-facing clocks.

use core::sync::atomic::{AtomicUsize, Ordering};

static MONOTONIC_MILLIS: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeValue {
    seconds: usize,
    subsecond_micros: usize,
}

impl TimeValue {
    pub const fn seconds(self) -> usize {
        self.seconds
    }

    pub const fn subsecond_micros(self) -> usize {
        self.subsecond_micros
    }

    pub const fn subsecond_nanos(self) -> usize {
        self.subsecond_micros * 1000
    }
}

pub fn next_time_value() -> TimeValue {
    let millis = MONOTONIC_MILLIS.fetch_add(1000, Ordering::AcqRel) + 1000;
    TimeValue {
        seconds: millis / 1000,
        subsecond_micros: (millis % 1000) * 1000,
    }
}

//! Cooldown mechanism.

use std::time::{Duration, Instant};

/// A cooldown token.
///
/// The token keeps track of the cooldown state.
#[derive(Clone, Debug)]
pub struct Cooldown {
    duration: Duration,
    instant: Instant,
}

impl Cooldown {
    /// Create a new cooldown token with the given duration.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            instant: Instant::now(),
        }
    }

    /// Resets the cooldown token.
    pub fn reset(&mut self) {
        self.instant = Instant::now();
    }

    /// Returns true if the cooldown is over.
    pub fn is_over(&self) -> bool {
        self.instant.elapsed() >= self.duration
    }
}

use std::time::{Duration, Instant};

use crate::Gate;

/// A fixed-rate real-time loop timer.
///
/// Sleeps until the next scheduled tick, then records the measured delta and
/// accumulated elapsed time. Timing state is advanced only by [`Self::wait`].
#[derive(Debug, Clone)]
pub struct Timer {
    /// Target loop frequency (Hz).
    hz:        f64,
    /// Measured duration between the last two completed waits.
    dt:        Duration,
    /// Instant recorded at the previous completed wait.
    last_tick: Instant,
    /// Scheduled instant for the next tick.
    next_tick: Instant,
    /// Accumulated elapsed time advanced by completed waits.
    elapsed:   Duration,
}

impl Timer {
    /// Creates a timer from a target frequency.
    ///
    /// # Arguments
    /// * `hz` - Target loop frequency (Hz)
    pub fn from_hz(hz: f64) -> Self {
        assert!(
            hz.is_finite() && hz > 0.0,
            "hz must be finite and greater than zero"
        );

        let now = Instant::now();
        let target_dt = Duration::from_secs_f64(1.0 / hz);

        Self {
            hz,
            dt: Duration::ZERO,
            last_tick: now,
            next_tick: now + target_dt,
            elapsed: Duration::ZERO,
        }
    }

    /// Returns the target loop frequency (Hz).
    pub fn hz(&self) -> f64 {
        self.hz
    }

    /// Returns the measured delta from the last completed [`Self::wait`].
    ///
    /// Before the first wait, this is zero.
    pub fn dt(&self) -> Duration {
        self.dt
    }

    /// Returns the accumulated elapsed time.
    ///
    /// This value is advanced only by [`Self::wait`].
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Waits until the next scheduled tick and updates timing state.
    ///
    /// If the caller is already late, the next tick is scheduled relative to
    /// the current time.
    pub fn wait(&mut self) {
        let target_dt = Duration::from_secs_f64(1.0 / self.hz);
        let now = Instant::now();
        if now < self.next_tick {
            spin_sleep::sleep(self.next_tick - now);
            self.next_tick += target_dt;
        } else {
            self.next_tick = now + target_dt;
        }

        let tick = Instant::now();
        self.dt = tick.duration_since(self.last_tick);
        self.elapsed += self.dt;
        self.last_tick = tick;
    }

    /// Creates a gate driven by this timer's source frequency.
    ///
    /// # Arguments
    /// * `target_hz` - Frequency at which the gate should fire (Hz)
    pub fn gate(&self, target_hz: f64) -> Gate {
        Gate::from_hz(self.hz, target_hz)
    }
}

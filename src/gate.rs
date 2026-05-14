/// A rate gate derived from a fixed source frequency.
///
/// Emits `true` at `target_hz` when ticked at `source_hz`. This is useful for
/// running lower-rate work inside a higher-rate loop without manually counting
/// integer divisors.
#[derive(Debug, Clone)]
pub struct Gate {
    /// Frequency of the loop calling [`Self::tick`] (Hz).
    source_hz: f64,
    /// Frequency at which this gate should fire (Hz).
    target_hz: f64,
    /// Fractional rate accumulator.
    acc:       f64,
}

impl Gate {
    /// Creates a gate from source and target frequencies.
    ///
    /// Prefer [`Timer::gate`](crate::Timer::gate) for public construction, so
    /// the gate always uses the timer's source frequency.
    ///
    /// # Arguments
    /// * `source_hz` - Frequency of the source loop (Hz)
    /// * `target_hz` - Frequency at which the gate should fire (Hz)
    pub(crate) fn from_hz(source_hz: f64, target_hz: f64) -> Self {
        assert!(
            source_hz.is_finite() && source_hz > 0.0,
            "source_hz must be finite and greater than zero"
        );
        assert!(
            target_hz.is_finite() && target_hz > 0.0,
            "target_hz must be finite and greater than zero"
        );
        assert!(
            target_hz <= source_hz,
            "target_hz must be less than or equal to source_hz"
        );

        Self {
            source_hz,
            target_hz,
            acc: 0.0,
        }
    }

    /// Returns the source frequency (Hz).
    pub fn source_hz(&self) -> f64 {
        self.source_hz
    }

    /// Returns the target frequency (Hz).
    pub fn target_hz(&self) -> f64 {
        self.target_hz
    }

    /// Advances the gate by one source tick.
    ///
    /// Returns `true` when the target rate should run on this tick.
    #[inline]
    pub fn tick(&mut self) -> bool {
        self.acc += self.target_hz;
        if self.acc >= self.source_hz {
            self.acc -= self.source_hz;
            true
        } else {
            false
        }
    }
}

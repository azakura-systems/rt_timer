//! Real-time loop timing utilities.
//!
//! Provides a fixed-rate [`Timer`] and matching [`Gate`] values for running
//! slower tasks from the same source rate.

mod gate;
mod timer;

pub use gate::Gate;
pub use timer::Timer;

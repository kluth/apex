use std::f64::consts::PI;

/// A Central Pattern Generator (CPG) oscillator.
/// Used to drive rhythmic biological movement by outputting a periodic signal.
#[derive(Debug, Clone)]
pub struct Cpg {
    frequency: f64, // Hz
    phase_offset: f64,
}

impl Cpg {
    /// Creates a new CPG with the given frequency.
    pub fn new(frequency: f64) -> Self {
        Self {
            frequency,
            phase_offset: 0.0,
        }
    }

    /// Advances the CPG state and returns the current signal value.
    /// signal = sin(2 * PI * freq * time + phase_offset)
    pub fn step(&self, time: f64) -> f64 {
        (2.0 * PI * self.frequency * time + self.phase_offset).sin()
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }
}

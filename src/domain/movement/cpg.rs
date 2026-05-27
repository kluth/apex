use std::f64::consts::PI;

/// A Central Pattern Generator (CPG) oscillator.
/// Used to drive rhythmic biological movement by outputting a periodic signal.
#[derive(Debug, Clone)]
pub struct Cpg {
    frequency: f64, // Hz
    phase: f64,     // Accumulated phase
}

impl Cpg {
    /// Creates a new CPG with the given frequency.
    pub fn new(frequency: f64) -> Self {
        Self {
            frequency,
            phase: 0.0,
        }
    }

    /// Advances the CPG state and returns the current signal value.
    /// signal = sin(phase)
    pub fn step(&mut self, dt: f64) -> f64 {
        self.phase += 2.0 * PI * self.frequency * dt;
        // Keep phase within [0, 2PI]
        self.phase %= 2.0 * PI;
        self.phase.sin()
    }

    /// Modulates the phase of the oscillator (Proprioceptive Feedback).
    pub fn modulate_phase(&mut self, delta_phi: f64) {
        self.phase += delta_phi;
    }

    pub fn frequency(&self) -> f64 {
        self.frequency
    }

    pub fn current_phase(&self) -> f64 {
        self.phase
    }
}

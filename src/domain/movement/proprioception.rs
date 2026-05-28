use crate::domain::movement::cpg::Cpg;

/// Domain Service for processing proprioceptive feedback.
pub struct Proprioception;

impl Proprioception {
    /// Modulates a CPG's phase based on muscle length deviation (stretch).
    /// delta_phi = Kp * (current_length - rest_length)
    pub fn modulate_from_stretch(cpg: &mut Cpg, current_len: f64, rest_len: f64, gain: f64) {
        let stretch = current_len - rest_len;
        let delta_phi = gain * stretch;
        cpg.modulate_phase(delta_phi);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::movement::cpg::Cpg;
    use std::f64::consts::PI;

    #[test]
    fn test_proprioceptive_feedback_loop() {
        let mut cpg = Cpg::new("Test_CPG".to_string(), 1.0);
        let initial_phase = cpg.current_phase();

        // Simulate a muscle stretch (20% elongation)
        Proprioception::modulate_from_stretch(&mut cpg, 1.2, 1.0, PI);

        // Expected phase shift = PI * (1.2 - 1.0) = 0.2 * PI
        let expected_phase = initial_phase + 0.2 * PI;
        assert!((cpg.current_phase() - expected_phase).abs() < 1e-6);
    }
}

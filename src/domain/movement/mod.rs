pub mod cpg;
pub mod proprioception;

#[cfg(test)]
mod tests {
    use super::cpg::Cpg;
    use std::f64::consts::PI;

    #[test]
    fn test_cpg_oscillation_signal() {
        // Red Phase: Ensure CPG outputs an oscillatory signal.
        let frequency = 1.0; // 1 Hz
        let mut cpg = Cpg::new(frequency);

        // At t=0.25s (90 degrees / PI/2)
        // Since step takes dt, we call it with 0.25
        let signal_1 = cpg.step(0.25);
        assert!((signal_1 - 1.0).abs() < 1e-6);

        // At t=0.5s (180 degrees / PI)
        // Next 0.25s
        let signal_2 = cpg.step(0.25);
        assert!(signal_2.abs() < 1e-6);
    }

    #[test]
    fn test_cpg_phase_modulation() {
        let mut cpg = Cpg::new(1.0);
        let initial_phase = cpg.current_phase();

        cpg.modulate_phase(PI / 2.0);
        assert!((cpg.current_phase() - initial_phase - PI / 2.0).abs() < 1e-6);
    }
}

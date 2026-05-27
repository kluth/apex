pub mod cpg;

#[cfg(test)]
mod tests {
    use super::cpg::Cpg;

    #[test]
    fn test_cpg_oscillation_signal() {
        // Red Phase: Ensure CPG outputs an oscillatory signal in the range [-1, 1].
        let frequency = 1.0; // 1 Hz
        let cpg = Cpg::new(frequency);

        // Initial signal at t=0
        let signal_0 = cpg.step(0.0);
        assert_eq!(signal_0, 0.0);

        // Signal at t=0.25s (90 degrees / pi/2)
        let signal_1 = cpg.step(0.25);
        assert!((signal_1 - 1.0).abs() < 1e-6);

        // Signal at t=0.5s (180 degrees / pi)
        let signal_2 = cpg.step(0.5);
        assert!(signal_2.abs() < 1e-6);
    }
}

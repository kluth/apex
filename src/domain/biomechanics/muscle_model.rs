/// A 1D Hill-Type Muscle Model calculator.
/// Calculates force as a function of activation, length, and velocity.
pub struct HillCurve;

impl HillCurve {
    /// Calculates the force output of the Contractile Element (CE).
    /// active_force = max_force * activation * f_l(length) * f_v(velocity)
    pub fn active_force(max_force: f64, activation: f64, _length: f64, _velocity: f64) -> f64 {
        // Simplified active force curve (assuming f_l and f_v are 1.0 for now)
        max_force * activation.clamp(0.0, 1.0)
    }

    /// Calculates the force output of the Parallel Elastic Element (PEE).
    /// passive_force = k * (length - rest_length)^2 (non-linear)
    pub fn passive_force(rest_length: f64, current_length: f64, stiffness: f64) -> f64 {
        if current_length > rest_length {
            let dl = current_length - rest_length;
            stiffness * dl * dl
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hill_active_force() {
        let force = HillCurve::active_force(500.0, 0.5, 1.0, 0.0);
        assert_eq!(force, 250.0);
    }

    #[test]
    fn test_hill_passive_force() {
        let rest_len = 1.0;
        let current_len = 1.2;
        let stiffness = 1000.0;
        
        let force = HillCurve::passive_force(rest_len, current_len, stiffness);
        // k * (0.2)^2 = 1000 * 0.04 = 40.0
        assert!((force - 40.0).abs() < 1e-6);
    }
}

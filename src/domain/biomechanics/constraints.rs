use crate::domain::biomechanics::registry::BodyRegistry;

/// The mathematical contract for all XPBD constraints.
pub trait XpbdConstraint: Send + Sync {
    /// Applies the constraint correction to the BodyRegistry.
    /// Returns the new Lagrange multiplier (lambda) for this constraint.
    fn solve(&self, registry: &mut BodyRegistry, dt: f64, lambda: f64) -> f64;

    /// Returns the indices of bodies affected by this constraint.
    fn affected_indices(&self) -> Vec<usize>;
}

/// A stiff or compliant distance constraint between two bodies.
pub struct DistanceConstraint {
    body_a_idx: usize,
    body_b_idx: usize,
    rest_length: f64,
    compliance: f64, // alpha in XPBD
}

impl DistanceConstraint {
    pub fn new(body_a_idx: usize, body_b_idx: usize, rest_length: f64, compliance: f64) -> Self {
        Self {
            body_a_idx,
            body_b_idx,
            rest_length,
            compliance,
        }
    }
}

impl XpbdConstraint for DistanceConstraint {
    fn solve(&self, registry: &mut BodyRegistry, dt: f64, lambda: f64) -> f64 {
        if dt < 1e-9 {
            return lambda;
        }

        let (w1, w2) = (
            registry.inv_mass[self.body_a_idx],
            registry.inv_mass[self.body_b_idx],
        );

        let w_sum = w1 + w2;
        if w_sum < 1e-9 && self.compliance < 1e-9 {
            return lambda;
        }

        let dx = registry.pos_x[self.body_a_idx] - registry.pos_x[self.body_b_idx];
        let dy = registry.pos_y[self.body_a_idx] - registry.pos_y[self.body_b_idx];
        let dz = registry.pos_z[self.body_a_idx] - registry.pos_z[self.body_b_idx];

        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        if dist < 1e-9 {
            return lambda;
        }

        let nx = dx / dist;
        let ny = dy / dist;
        let nz = dz / dist;

        let c = dist - self.rest_length;
        let alpha_tilde = self.compliance / (dt * dt);

        let denom = w_sum + alpha_tilde;
        if denom < 1e-9 {
            return lambda;
        }

        let delta_lambda = (-c - alpha_tilde * lambda) / denom;

        registry.apply_correction(
            self.body_a_idx,
            nx * delta_lambda * w1,
            ny * delta_lambda * w1,
            nz * delta_lambda * w1,
        );

        registry.apply_correction(
            self.body_b_idx,
            -nx * delta_lambda * w2,
            -ny * delta_lambda * w2,
            -nz * delta_lambda * w2,
        );

        lambda + delta_lambda
    }

    fn affected_indices(&self) -> Vec<usize> {
        vec![self.body_a_idx, self.body_b_idx]
    }
}

/// An active muscle actuator constraint.
/// Applies force based on a dynamic rest_length or direct force injection.
pub struct MuscleActuator {
    body_a_idx: usize,
    body_b_idx: usize,
    max_force: f64,
    activation: f64, // 0.0 to 1.0
}

impl MuscleActuator {
    pub fn new(body_a_idx: usize, body_b_idx: usize, max_force: f64) -> Self {
        Self {
            body_a_idx,
            body_b_idx,
            max_force,
            activation: 0.0,
        }
    }

    pub fn set_activation(&mut self, activation: f64) {
        self.activation = activation.clamp(0.0, 1.0);
    }
}

impl XpbdConstraint for MuscleActuator {
    fn solve(&self, registry: &mut BodyRegistry, dt: f64, _lambda: f64) -> f64 {
        if self.activation < 1e-3 { return 0.0; }

        let dx = registry.pos_x[self.body_a_idx] - registry.pos_x[self.body_b_idx];
        let dy = registry.pos_y[self.body_a_idx] - registry.pos_y[self.body_b_idx];
        let dz = registry.pos_z[self.body_a_idx] - registry.pos_z[self.body_b_idx];

        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        if dist < 1e-9 { return 0.0; }

        let nx = dx / dist;
        let ny = dy / dist;
        let nz = dz / dist;

        // Apply active force as position correction (compliance-based contraction)
        // force = max_force * activation
        // This is a simplification: in XPBD, we treat it as a contraction toward zero length
        // but with high compliance to simulate muscle pulling force.
        let force = self.max_force * self.activation;
        let impulse = force * dt * dt; // Position-based force equivalent

        let w1 = registry.inv_mass[self.body_a_idx];
        let w2 = registry.inv_mass[self.body_b_idx];

        registry.apply_correction(self.body_a_idx, -nx * impulse * w1, -ny * impulse * w1, -nz * impulse * w1);
        registry.apply_correction(self.body_b_idx, nx * impulse * w2, ny * impulse * w2, nz * impulse * w2);

        0.0
    }

    fn affected_indices(&self) -> Vec<usize> {
        vec![self.body_a_idx, self.body_b_idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::biomechanics::registry::BodyRegistry;

    #[test]
    fn test_distance_constraint_solving_soa() {
        let mut registry = BodyRegistry::new();
        registry.add_body(0.0, 0.0, 0.0, 1.0);
        registry.add_body(2.0, 0.0, 0.0, 1.0);

        let constraint = DistanceConstraint::new(0, 1, 1.0, 0.0);
        let _new_lambda = constraint.solve(&mut registry, 1.0, 0.0);

        assert!((registry.pos_x[0] - 0.5).abs() < 1e-6);
        assert!((registry.pos_x[1] - 1.5).abs() < 1e-6);
    }
}

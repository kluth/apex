use crate::domain::biomechanics::registry::BodyRegistry;

/// The mathematical contract for all XPBD constraints.
pub trait XpbdConstraint: Send + Sync {
    /// Applies the constraint correction to the BodyRegistry.
    /// Returns the updated accumulated Lagrange multiplier.
    fn solve(&self, registry: &mut BodyRegistry, dt: f64, accumulated_lambda: f64) -> f64;

    /// Returns the indices of the bodies affected by this constraint.
    fn affected_indices(&self) -> Vec<usize>;
}

/// An XPBD constraint enforcing a specific distance between two points.
/// Used for Spherical Joints (d=0) and rigid bone segments.
#[derive(Debug, Clone)]
pub struct DistanceConstraint {
    pub body_a_idx: usize,
    pub body_b_idx: usize,
    pub rest_length: f64,
    pub compliance: f64, // alpha
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
        let (w1, w2) = (registry.inv_mass[self.body_a_idx], registry.inv_mass[self.body_b_idx]);
        let dx = registry.pos_x[self.body_a_idx] - registry.pos_x[self.body_b_idx];
        let dy = registry.pos_y[self.body_a_idx] - registry.pos_y[self.body_b_idx];
        let dz = registry.pos_z[self.body_a_idx] - registry.pos_z[self.body_b_idx];

        let dist = (dx * dx + dy * dy + dz * dz).sqrt();
        if dist < 1e-9 { return lambda; }

        let nx = dx / dist;
        let ny = dy / dist;
        let nz = dz / dist;

        let c = dist - self.rest_length;
        let alpha_tilde = self.compliance / (dt * dt);

        let delta_lambda = (-c - alpha_tilde * lambda) / (w1 + w2 + alpha_tilde);
        
        registry.apply_correction(self.body_a_idx, nx * delta_lambda * w1, ny * delta_lambda * w1, nz * delta_lambda * w1);
        registry.apply_correction(self.body_b_idx, -nx * delta_lambda * w2, -ny * delta_lambda * w2, -nz * delta_lambda * w2);

        lambda + delta_lambda
    }

    fn affected_indices(&self) -> Vec<usize> {
        vec![self.body_a_idx, self.body_b_idx]
    }
}

/// An XPBD constraint restricting rotation around one or more axes.
/// Used for Revolute (Hinge) Joints.
#[derive(Debug, Clone)]
pub struct AngularConstraint {
    pub body_a_idx: usize,
    pub body_b_idx: usize,
    pub compliance: f64,
}

impl AngularConstraint {
    pub fn new(body_a_idx: usize, body_b_idx: usize, compliance: f64) -> Self {
        Self {
            body_a_idx,
            body_b_idx,
            compliance,
        }
    }
}

impl XpbdConstraint for AngularConstraint {
    fn solve(&self, _registry: &mut BodyRegistry, _dt: f64, lambda: f64) -> f64 {
        lambda
    }

    fn affected_indices(&self) -> Vec<usize> {
        vec![self.body_a_idx, self.body_b_idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

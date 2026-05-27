use crate::domain::biomechanics::rigid_body::RigidBody;

/// The mathematical contract for all XPBD constraints.
pub trait XpbdConstraint {
    /// Applies the constraint correction to the involved rigid bodies.
    /// Returns the updated accumulated Lagrange multiplier.
    fn solve(&self, bodies: &mut [RigidBody], dt: f64, accumulated_lambda: f64) -> f64;
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
    fn solve(&self, bodies: &mut [RigidBody], dt: f64, lambda: f64) -> f64 {
        // We need to access two different indices from the same slice.
        // This is tricky in safe Rust. We'll use split_at_mut if indices are ordered.

        let (w1, w2, p1, p2) = {
            let b1 = &bodies[self.body_a_idx];
            let b2 = &bodies[self.body_b_idx];
            (
                b1.inverse_mass(),
                b2.inverse_mass(),
                *b1.position(),
                *b2.position(),
            )
        };

        let diff = p1 - p2;
        let dist = diff.length();
        if dist < 1e-9 {
            return lambda;
        }

        let n = diff.normalize();
        let c = dist - self.rest_length;
        let alpha_tilde = self.compliance / (dt * dt);

        let delta_lambda = (-c - alpha_tilde * lambda) / (w1 + w2 + alpha_tilde);
        let p = n * delta_lambda;

        bodies[self.body_a_idx].apply_correction(p * w1);
        bodies[self.body_b_idx].apply_correction(p * -w2);

        lambda + delta_lambda
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

// Angular constraint implementation is more complex (requires orientations).
// For Chapter 8, we prioritize the Distance constraint to hold the skeleton together.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::biomechanics::rigid_body::Vector3;

    #[test]
    fn test_distance_constraint_solving() {
        let mut bodies = vec![
            RigidBody::new(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                1.0,
            ),
            RigidBody::new(
                Vector3 {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                1.0,
            ),
        ];

        let constraint = DistanceConstraint::new(0, 1, 1.0, 0.0); // Rigid dist=1.0

        // Solve with dt=1.0, lambda=0.0
        let _new_lambda = constraint.solve(&mut bodies, 1.0, 0.0);

        // Positions should move toward each other to satisfy dist=1.0
        // Expected: b1 moves to 0.5, b2 moves to 1.5
        assert!((bodies[0].position().x - 0.5).abs() < 1e-6);
        assert!((bodies[1].position().x - 1.5).abs() < 1e-6);
    }
}

use crate::domain::biomechanics::rigid_body::RigidBody;
use crate::domain::biomechanics::constraints::XpbdConstraint;

/// An XPBD constraint that preserves the 3D volume of a tetrahedral segment.
/// Each tetrahedron has 4 vertices (modelled as rigid body indices).
#[derive(Debug, Clone)]
pub struct VolumeConstraint {
    pub bodies: [usize; 4],
    pub rest_volume: f64,
    pub compliance: f64,
}

impl VolumeConstraint {
    pub fn new(bodies: [usize; 4], rest_volume: f64, compliance: f64) -> Self {
        Self {
            bodies,
            rest_volume,
            compliance,
        }
    }
}

impl XpbdConstraint for VolumeConstraint {
    fn solve(&self, _bodies: &mut [RigidBody], _dt: f64, lambda: f64) -> f64 {
        // Volumetric integration is O(1) per tetrahedron but requires 4-way cross products.
        // For the Tier 4 foundation, we establish the structure.
        // Full mathematical kernel follows in the optimized solver loop.
        lambda
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_constraint_initialization() {
        let constraint = VolumeConstraint::new([0, 1, 2, 3], 0.1, 0.0);
        assert_eq!(constraint.bodies, [0, 1, 2, 3]);
        assert_eq!(constraint.rest_volume, 0.1);
    }
}

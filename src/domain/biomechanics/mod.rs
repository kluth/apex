pub mod constraints;
pub mod muscle_model;
pub mod rigid_body;
pub mod soft_body;
pub mod world;

#[cfg(test)]
mod tests {
    use super::rigid_body::{RigidBody, Vector3};
    

    #[test]
    fn test_xpbd_rigid_body_initialization_and_inverse_mass() {
        // Red Phase: Ensure rigid body calculates inverse mass correctly.
        // If mass is infinite (0.0 inverse mass), it acts as a static kinematic body.

        let position = Vector3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        };
        let mass = 2.0;

        let body = RigidBody::new(position, mass);

        assert_eq!(body.position().y, 10.0);
        assert_eq!(body.velocity().y, 0.0);
        // Inverse mass should be 1.0 / 2.0
        assert_eq!(body.inverse_mass(), 0.5);

        // Static body test (mass = 0.0 signals infinite mass in XPBD)
        let static_body = RigidBody::new(position, 0.0);
        assert_eq!(static_body.inverse_mass(), 0.0);
    }
}

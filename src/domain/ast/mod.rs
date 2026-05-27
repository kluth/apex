#[cfg(test)]
mod tests {
    use super::bone::{Bone, Mass};
    use crate::domain::biomechanics::rigid_body::Vector3;

    #[test]
    fn test_bone_instantiation_invariant() {
        // Red Phase: We expect instantiation to succeed only with valid >0 mass.
        // The Result pattern must be used.
        let valid_mass = Mass::new(2.5).expect("Should create valid mass");
        let femur = Bone::new("Femur".to_string(), valid_mass, Vector3::default());

        assert_eq!(femur.id(), "Femur");
        assert_eq!(femur.mass().value(), 2.5);

        // This must return an error (illegal state)
        let invalid_mass = Mass::new(-1.0);
        assert!(invalid_mass.is_err(), "Mass cannot be negative");
    }
}

pub mod bone;
pub mod joint;
pub mod muscle;
pub mod skin;
pub mod synapse;

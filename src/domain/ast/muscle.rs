use crate::domain::ast::bone::Bone;

/// Specifies the bone ID and local offset where a muscle is anchored.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MuscleAttachment {
    pub bone_id_hash: u64, // Placeholder for hashed ID or reference
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
}

impl MuscleAttachment {
    pub fn new(_bone: &Bone, x: f64, y: f64, z: f64) -> Self {
        // In a real implementation, we'd store a reference or a stable ID.
        // For AST, we use the string ID.
        Self {
            bone_id_hash: 0, // Placeholder
            offset_x: x,
            offset_y: y,
            offset_z: z,
        }
    }
}

/// Aggregate Root representing a biological actuator (Muscle).
#[derive(Debug, Clone, PartialEq)]
pub struct Muscle {
    id: String,
    source_bone_id: String,
    target_bone_id: String,
    origin: MuscleAttachment,
    insertion: MuscleAttachment,
    max_force: f64, // Newton
}

impl Muscle {
    pub fn new(
        id: String,
        source_bone: &Bone,
        target_bone: &Bone,
        origin_offset: (f64, f64, f64),
        insertion_offset: (f64, f64, f64),
        max_force: f64,
    ) -> Self {
        Self {
            id,
            source_bone_id: source_bone.id().to_string(),
            target_bone_id: target_bone.id().to_string(),
            origin: MuscleAttachment {
                bone_id_hash: 0,
                offset_x: origin_offset.0,
                offset_y: origin_offset.1,
                offset_z: origin_offset.2,
            },
            insertion: MuscleAttachment {
                bone_id_hash: 0,
                offset_x: insertion_offset.0,
                offset_y: insertion_offset.1,
                offset_z: insertion_offset.2,
            },
            max_force,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn source_bone_id(&self) -> &str {
        &self.source_bone_id
    }

    pub fn target_bone_id(&self) -> &str {
        &self.target_bone_id
    }

    pub fn max_force(&self) -> f64 {
        self.max_force
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ast::bone::{Bone, Mass};
    use crate::domain::biomechanics::rigid_body::Vector3;

    #[test]
    fn test_muscle_instantiation() {
        let mass = Mass::new(1.0).unwrap();
        let femur = Bone::new("Femur".to_string(), mass.clone(), Vector3::default());
        let tibia = Bone::new("Tibia".to_string(), mass, Vector3::default());

        let biceps_femoris = Muscle::new(
            "BicepsFemoris".to_string(),
            &femur,
            &tibia,
            (0.0, 0.0, 0.0),
            (0.0, 0.5, 0.0),
            500.0,
        );

        assert_eq!(biceps_femoris.id(), "BicepsFemoris");
        assert_eq!(biceps_femoris.source_bone_id(), "Femur");
        assert_eq!(biceps_femoris.target_bone_id(), "Tibia");
        assert_eq!(biceps_femoris.max_force(), 500.0);
    }
}

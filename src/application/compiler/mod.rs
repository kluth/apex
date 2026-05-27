pub mod pipeline;
pub mod validator;

#[cfg(test)]
mod tests {
    use super::pipeline::CompilerPipeline;
    use super::validator::{BiologicalValidator, ValidationError};
    use crate::domain::ast::bone::{Bone, Mass};
    use crate::domain::ast::joint::{Joint, JointAttachment, JointType};

    #[test]
    fn test_compiler_lowering_bone_to_topology() {
        let mass = Mass::new(1.0).unwrap();
        let bone_ast = Bone::new("Humerus".to_string(), mass);

        let pipeline = CompilerPipeline::new();
        let topology = pipeline
            .lower(vec![bone_ast], vec![])
            .expect("Validation should pass");

        assert_eq!(topology.node_count(), 1);
    }

    #[test]
    fn test_compiler_lowering_joint_to_topology() {
        let mass = Mass::new(1.0).unwrap();
        let femur = Bone::new("Femur".to_string(), mass.clone());
        let tibia = Bone::new("Tibia".to_string(), mass);

        let knee = Joint::new(
            "Knee".to_string(),
            JointType::Revolute,
            &femur,
            &tibia,
            JointAttachment::default(),
            JointAttachment::default(),
        )
        .unwrap();

        let pipeline = CompilerPipeline::new();
        let topology = pipeline
            .lower(vec![femur, tibia], vec![knee])
            .expect("Lowering should pass");

        assert_eq!(topology.node_count(), 2);
        // Topology currently doesn't expose a way to check edges by name, but we can verify node count.
    }

    #[test]
    fn test_biological_validation_duplicate_id() {
        let mass = Mass::new(1.0).unwrap();
        let bone1 = Bone::new("Femur".to_string(), mass.clone());
        let bone2 = Bone::new("Femur".to_string(), mass);

        let bones = vec![bone1, bone2];
        let result = BiologicalValidator::validate_bones(&bones);

        assert_eq!(
            result,
            Err(ValidationError::DuplicateIdentifier("Femur".to_string()))
        );
    }
}

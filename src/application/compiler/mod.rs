pub mod pipeline;
pub mod validator;

#[cfg(test)]
mod tests {
    use crate::domain::ast::bone::{Bone, Mass};
    use crate::domain::ast::joint::{Joint, JointAttachment, JointType};
    use crate::domain::ast::muscle::Muscle;
    use crate::domain::ast::synapse::Synapse;
    use crate::domain::movement::cpg::Cpg;
    use super::pipeline::CompilerPipeline;
    use super::validator::{BiologicalValidator, ValidationError};

    #[test]
    fn test_compiler_lowering_bone_to_topology() {
        let mass = Mass::new(1.0).unwrap();
        let bone_ast = Bone::new("Humerus".to_string(), mass);
        
        let pipeline = CompilerPipeline::new();
        let topology = pipeline.lower(vec![bone_ast], vec![], vec![], vec![]).expect("Validation should pass");
        
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
        ).unwrap();

        let pipeline = CompilerPipeline::new();
        let topology = pipeline.lower(vec![femur, tibia], vec![knee], vec![], vec![]).expect("Lowering should pass");

        assert_eq!(topology.node_count(), 2);
    }

    #[test]
    fn test_compiler_lowering_muscle_to_topology() {
        let mass = Mass::new(1.0).unwrap();
        let femur = Bone::new("Femur".to_string(), mass.clone());
        let tibia = Bone::new("Tibia".to_string(), mass);
        
        let biceps = Muscle::new(
            "Biceps".to_string(),
            &femur,
            &tibia,
            (0.0, 0.0, 0.0),
            (0.0, 0.5, 0.0),
            500.0,
        );

        let pipeline = CompilerPipeline::new();
        let topology = pipeline.lower(vec![femur, tibia], vec![], vec![biceps], vec![]).expect("Lowering should pass");

        assert_eq!(topology.node_count(), 2);
    }

    #[test]
    fn test_compiler_lowering_synapse() {
        let mass = Mass::new(1.0).unwrap();
        let femur = Bone::new("Femur".to_string(), mass.clone());
        let tibia = Bone::new("Tibia".to_string(), mass);
        let muscle = Muscle::new(
            "Biceps".to_string(),
            &femur,
            &tibia,
            (0.0, 0.0, 0.0),
            (0.0, 0.5, 0.0),
            500.0,
        );
        let cpg = Cpg::new(1.0);
        let synapse = Synapse::new("Syn_1".to_string(), &cpg, &muscle, 1.0);

        let pipeline = CompilerPipeline::new();
        let result = pipeline.lower(vec![femur, tibia], vec![], vec![muscle], vec![synapse]);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_biological_validation_duplicate_id() {
        let mass = Mass::new(1.0).unwrap();
        let bone1 = Bone::new("Femur".to_string(), mass.clone());
        let bone2 = Bone::new("Femur".to_string(), mass);
        
        let bones = vec![bone1, bone2];
        let result = BiologicalValidator::validate_bones(&bones);
        
        assert_eq!(result, Err(ValidationError::DuplicateIdentifier("Femur".to_string())));
    }
}

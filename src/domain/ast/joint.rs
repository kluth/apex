use crate::domain::ast::bone::Bone;

#[derive(Debug, Clone, PartialEq)]
pub enum JointError {
    SelfConnection(String),
}

/// Specifies the mechanical behavior of a joint.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JointType {
    /// 3 rotational DOF (Ball-and-socket)
    Spherical,
    /// 1 rotational DOF (Hinge)
    Revolute,
}

/// The local offset from a bone's center where a joint is anchored.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JointAttachment {
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
}

impl Default for JointAttachment {
    fn default() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
        }
    }
}

/// Aggregate Root representing a Kinematic Connection between two bones.
#[derive(Debug, Clone, PartialEq)]
pub struct Joint {
    id: String,
    joint_type: JointType,
    source_bone_id: String,
    target_bone_id: String,
    source_attachment: JointAttachment,
    target_attachment: JointAttachment,
}

impl Joint {
    pub fn new(
        id: String,
        joint_type: JointType,
        source_bone: &Bone,
        target_bone: &Bone,
        source_attachment: JointAttachment,
        target_attachment: JointAttachment,
    ) -> Result<Self, JointError> {
        if source_bone.id() == target_bone.id() {
            return Err(JointError::SelfConnection(source_bone.id().to_string()));
        }

        Ok(Self {
            id,
            joint_type,
            source_bone_id: source_bone.id().to_string(),
            target_bone_id: target_bone.id().to_string(),
            source_attachment,
            target_attachment,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn joint_type(&self) -> JointType {
        self.joint_type
    }

    pub fn source_bone_id(&self) -> &str {
        &self.source_bone_id
    }

    pub fn target_bone_id(&self) -> &str {
        &self.target_bone_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ast::bone::{Bone, Mass};

    #[test]
    fn test_joint_instantiation_invariant() {
        let mass = Mass::new(1.0).unwrap();
        let femur = Bone::new("Femur".to_string(), mass.clone());
        let tibia = Bone::new("Tibia".to_string(), mass);

        // Valid joint
        let knee = Joint::new(
            "Knee".to_string(),
            JointType::Revolute,
            &femur,
            &tibia,
            JointAttachment::default(),
            JointAttachment::default(),
        );
        assert!(knee.is_ok());
        let knee = knee.unwrap();
        assert_eq!(knee.source_bone_id(), "Femur");
        assert_eq!(knee.target_bone_id(), "Tibia");

        // Self-connection violation
        let invalid_joint = Joint::new(
            "Broken".to_string(),
            JointType::Spherical,
            &femur,
            &femur,
            JointAttachment::default(),
            JointAttachment::default(),
        );
        assert_eq!(
            invalid_joint,
            Err(JointError::SelfConnection("Femur".to_string()))
        );
    }
}

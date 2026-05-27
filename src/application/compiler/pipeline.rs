use super::validator::{BiologicalValidator, ValidationError};
use crate::domain::air::topology::{NodeId, Topology};
use crate::domain::ast::bone::Bone;
use crate::domain::ast::joint::Joint;
use std::collections::HashMap;

/// Application Service responsible for lowering AST to AIR.
pub struct CompilerPipeline;

impl Default for CompilerPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl CompilerPipeline {
    pub fn new() -> Self {
        Self
    }

    /// Lowers a collection of AST Bones and Joints into an AIR Topology.
    /// This is the primary 'compilation' pass for anatomical structure.
    /// Returns an error if biological validation fails.
    pub fn lower(&self, bones: Vec<Bone>, joints: Vec<Joint>) -> Result<Topology, ValidationError> {
        // 1. Validation Pass
        BiologicalValidator::validate_bones(&bones)?;

        // 2. Lowering Pass
        let mut topology = Topology::new();
        let mut bone_map: HashMap<String, NodeId> = HashMap::new();

        for bone in bones {
            let id = topology.add_node(bone.id().to_string());
            bone_map.insert(bone.id().to_string(), id);
        }

        for joint in joints {
            let source_id = bone_map
                .get(joint.source_bone_id())
                .expect("Source bone must exist");
            let target_id = bone_map
                .get(joint.target_bone_id())
                .expect("Target bone must exist");

            topology.add_edge(*source_id, *target_id, joint.id().to_string());
        }

        Ok(topology)
    }
}

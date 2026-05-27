use crate::application::compiler::validator::{BiologicalValidator, ValidationError};
use crate::domain::air::topology::{NodeId, Topology};
use crate::domain::ast::bone::Bone;
use crate::domain::ast::joint::Joint;
use crate::domain::ast::muscle::Muscle;
use crate::domain::ast::skin::Skin;
use crate::domain::ast::synapse::Synapse;
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

    /// Lowers a collection of AST components into an AIR Topology.
    /// This is the primary 'compilation' pass for anatomical and neural structure.
    pub fn lower(
        &self,
        bones: Vec<Bone>,
        joints: Vec<Joint>,
        muscles: Vec<Muscle>,
        synapses: Vec<Synapse>,
        skins: Vec<Skin>,
    ) -> Result<Topology, ValidationError> {
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

        for muscle in muscles {
            let source_id = bone_map
                .get(muscle.source_bone_id())
                .expect("Source bone must exist");
            let target_id = bone_map
                .get(muscle.target_bone_id())
                .expect("Target bone must exist");

            topology.add_edge(*source_id, *target_id, muscle.id().to_string());
        }

        for synapse in synapses {
            tracing::debug!("Mapping Synapse: {}", synapse.id());
        }

        for skin in skins {
            let _target_id = bone_map
                .get(skin.target_bone_id())
                .expect("Target bone for skin must exist");
            
            // Skin segments are lowered to volumetric hulls attached to the node.
            tracing::debug!("Lowering Skin shell: {}", skin.id());
        }

        Ok(topology)
    }
}

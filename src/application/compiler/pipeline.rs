use crate::domain::ast::bone::Bone;
use crate::domain::air::topology::Topology;
use super::validator::{BiologicalValidator, ValidationError};

/// Application Service responsible for lowering AST to AIR.
pub struct CompilerPipeline;

impl CompilerPipeline {
    pub fn new() -> Self {
        Self
    }

    /// Lowers a collection of AST Bones into an AIR Topology.
    /// This is the primary 'compilation' pass for anatomical structure.
    /// Returns an error if biological validation fails.
    pub fn lower(&self, bones: Vec<Bone>) -> Result<Topology, ValidationError> {
        // 1. Validation Pass
        BiologicalValidator::validate_bones(&bones)?;

        // 2. Lowering Pass
        let mut topology = Topology::new();
        for bone in bones {
            topology.add_node(bone.id().to_string());
        }
        
        Ok(topology)
    }
}

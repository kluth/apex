use crate::application::compiler::parser::{ParseError, Parser};
use crate::application::compiler::validator::{BiologicalValidator, ValidationError};
use crate::domain::air::topology::{NodeId, Topology};
use crate::domain::ast::bone::Bone;
use crate::domain::ast::joint::Joint;
use crate::domain::ast::muscle::Muscle;
use crate::domain::ast::skin::Skin;
use crate::domain::ast::synapse::Synapse;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum CompileError {
    Parse(String),
    Validation(ValidationError),
}

impl From<ParseError> for CompileError {
    fn from(err: ParseError) -> Self {
        CompileError::Parse(format!("{:?}", err))
    }
}

impl From<ValidationError> for CompileError {
    fn from(err: ValidationError) -> Self {
        CompileError::Validation(err)
    }
}

/// Application Service responsible for orchestrating the compilation pass.
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

    /// Compiles raw APEX source text into an AIR Topology.
    /// Full Pipeline: Text -> Lex -> Parse -> Validate -> Lower
    pub fn compile(&self, source: &str) -> Result<Topology, CompileError> {
        // 1. Lex & Parse
        let mut parser = Parser::new(source);
        let ast = parser.parse_organism()?;

        // 2. Validate & Lower
        self.lower(ast.bones, ast.joints, ast.muscles, vec![], vec![])
            .map_err(Into::into)
    }

    /// Lowers a collection of AST components into an AIR Topology.
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
            let id = topology.add_node(bone.id().to_string(), bone.position());
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
            tracing::debug!("Lowering Skin shell: {}", skin.id());
        }

        Ok(topology)
    }
}

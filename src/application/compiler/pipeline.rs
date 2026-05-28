use crate::application::compiler::parser::{ParseError, Parser, ReceptorAst};
use crate::application::compiler::validator::{BiologicalValidator, ValidationError};
use crate::domain::air::topology::{EdgeType, NodeId, Topology};
use crate::domain::ast::bone::Bone;
use crate::domain::ast::joint::Joint;
use crate::domain::ast::muscle::Muscle;
use crate::domain::ast::skin::Skin;
use crate::domain::ast::synapse::Synapse;
use crate::domain::movement::cpg::Cpg;
use crate::domain::biomechanics::rigid_body::Vector3;
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
        self.lower(ast.bones, ast.joints, ast.muscles, ast.cpgs, ast.synapses, ast.receptors, vec![])
            .map_err(Into::into)
    }

    /// Lowers a collection of AST components into an AIR Topology.
    pub fn lower(
        &self,
        bones: Vec<Bone>,
        joints: Vec<Joint>,
        muscles: Vec<Muscle>,
        cpgs: Vec<Cpg>,
        synapses: Vec<Synapse>,
        receptors: Vec<ReceptorAst>,
        _skins: Vec<Skin>,
    ) -> Result<Topology, ValidationError> {
        // 1. Validation Pass
        BiologicalValidator::validate_bones(&bones)?;

        // 2. Lowering Pass
        let mut topology = Topology::new();
        let mut bone_map: HashMap<String, NodeId> = HashMap::new();
        let mut muscle_map: HashMap<String, NodeId> = HashMap::new();
        let mut cpg_map: HashMap<String, NodeId> = HashMap::new();

        // Bones become Nodes
        for bone in bones {
            let id = topology.add_node(
                bone.id().to_string(),
                bone.position(),
                bone.mesh_reference().cloned(),
            );
            bone_map.insert(bone.id().to_string(), id);
        }

        // CPGs become Neural Nodes (Positioned in a virtual "brain" area or near head)
        for (i, cpg) in cpgs.iter().enumerate() {
            // Virtual brain grid at Y=2.0
            let brain_pos = Vector3 { x: (i as f64 * 0.1) - 0.5, y: 2.1, z: 0.0 };
            let id = topology.add_node(format!("CPG_{}", cpg.id()), brain_pos, None);
            cpg_map.insert(cpg.id().to_string(), id);
        }

        // Joints become Structural Edges
        for joint in joints {
            let source_id = bone_map.get(joint.source_bone_id()).ok_or_else(|| {
                ValidationError::MissingIdentifier(joint.source_bone_id().to_string())
            })?;
            let target_id = bone_map.get(joint.target_bone_id()).ok_or_else(|| {
                ValidationError::MissingIdentifier(joint.target_bone_id().to_string())
            })?;

            topology.add_edge(
                *source_id,
                *target_id,
                joint.id().to_string(),
                EdgeType::Structural,
            );
        }

        // Muscles become Actuator Edges + Virtual Attachment Nodes
        for muscle in muscles {
            let source_id = bone_map.get(muscle.source_bone_id()).ok_or_else(|| {
                ValidationError::MissingIdentifier(muscle.source_bone_id().to_string())
            })?;
            let target_id = bone_map.get(muscle.target_bone_id()).ok_or_else(|| {
                ValidationError::MissingIdentifier(muscle.target_bone_id().to_string())
            })?;

            topology.add_edge(
                *source_id,
                *target_id,
                muscle.id().to_string(),
                EdgeType::Actuator,
            );
            
            // Create a virtual node for the muscle center to visualize synapses.
            let s_pos = topology.node_position(*source_id).unwrap_or_default();
            let t_pos = topology.node_position(*target_id).unwrap_or_default();
            let center = (s_pos + t_pos) * 0.5;
            
            let m_node_id = topology.add_node(format!("MUSCLE_NODE_{}", muscle.id()), center, None);
            muscle_map.insert(muscle.id().to_string(), m_node_id);
        }

        // Synapses become Neural Edges
        for synapse in synapses {
            let source_id = cpg_map.get(synapse.source_cpg_id()).ok_or_else(|| {
                ValidationError::MissingIdentifier(synapse.source_cpg_id().to_string())
            })?;
            let target_id = muscle_map.get(synapse.target_muscle_id()).ok_or_else(|| {
                ValidationError::MissingIdentifier(synapse.target_muscle_id().to_string())
            })?;

            topology.add_edge(
                *source_id,
                *target_id,
                synapse.id().to_string(),
                EdgeType::Neural,
            );
        }

        // Receptors become Sensory Edges
        for receptor in receptors {
            let m_node_id = muscle_map.get(&receptor.muscle_id).ok_or_else(|| {
                ValidationError::MissingIdentifier(receptor.muscle_id.clone())
            })?;
            
            // Pick a random CPG as target for feedback (simplified)
            if let Some(target_cpg_id) = cpg_map.values().next() {
                topology.add_edge(
                    *m_node_id,
                    *target_cpg_id,
                    receptor.id.clone(),
                    EdgeType::Sensory,
                );
            }
        }

        Ok(topology)
    }
}

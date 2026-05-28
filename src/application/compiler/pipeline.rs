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
            
            // ANATOMICAL FIX: Muscle node is placed at the Origin (Source) to make synapses look integrated.
            let s_pos = topology.node_position(*source_id).unwrap_or_default();
            let m_node_id = topology.add_node(format!("MUSCLE_NODE_{}", muscle.id()), s_pos, None);
            muscle_map.insert(muscle.id().to_string(), m_node_id);
        }

        // CPGs (Nerves) become Neural Nodes
        for cpg in &cpgs {
            let id_str = cpg.id();
            let mut pos = Vector3 { x: 0.0, y: 2.1, z: 0.0 }; // Fallback "Brain"

            // ANATOMICAL POSITIONING:
            if id_str.starts_with("Nerve_") {
                let parts: Vec<&str> = id_str.split('_').collect();
                if parts.len() >= 2 {
                    let vert_id = parts[1];
                    let side = parts.get(2).cloned().unwrap_or("");
                    if let Some(node_id) = bone_map.get(vert_id) {
                        pos = topology.node_position(*node_id).unwrap_or(pos);
                        if side == "L" { pos.x -= 0.05; }
                        else { pos.x += 0.05; }
                    }
                }
            } else if id_str.starts_with("CN_") {
                if let Some(node_id) = bone_map.get("Occipital") {
                    pos = topology.node_position(*node_id).unwrap_or(pos);
                    if id_str.contains("_L") { pos.x -= 0.05; }
                    else { pos.x += 0.05; }
                    pos.z += 0.05;
                }
            } else if id_str.starts_with("Brain_") {
                 pos = Vector3 { x: 0.0, y: 2.1, z: 0.0 };
            }

            let id = topology.add_node(format!("CPG_{}", id_str), pos, None);
            cpg_map.insert(id_str.to_string(), id);
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
            
            // Receptors provide feedback back to the Nerve.
            // Simplified: Find a nerve with similar name/side
            let side = if receptor.id.contains("_L") { "_L" } else { "_R" };
            let target_cpg_id = cpg_map.keys()
                .find(|k| k.contains(side))
                .and_then(|k| cpg_map.get(k))
                .or_else(|| cpg_map.values().next());

            if let Some(tid) = target_cpg_id {
                topology.add_edge(
                    *m_node_id,
                    *tid,
                    receptor.id.clone(),
                    EdgeType::Sensory,
                );
            }
        }

        Ok(topology)
    }
}

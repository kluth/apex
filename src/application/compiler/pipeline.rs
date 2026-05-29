use crate::application::compiler::parser::{OrganAst, ParseError, Parser, ReceptorAst};
use crate::application::compiler::validator::{BiologicalValidator, ValidationError};
use crate::domain::air::topology::{EdgeType, NodeId, Topology};
use crate::domain::ast::bone::Bone;
use crate::domain::ast::joint::Joint;
use crate::domain::ast::muscle::Muscle;
use crate::domain::ast::skin::Skin;
use crate::domain::ast::synapse::Synapse;
use crate::domain::movement::cpg::Cpg;
use crate::domain::biomechanics::rigid_body::Vector3;
use crate::domain::biomechanics::world::{World, Synapse as PhysicsSynapse};
use crate::domain::biomechanics::constraints::{DistanceConstraint, MuscleActuator};
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
        self.lower(ast.bones, ast.joints, ast.muscles, ast.cpgs, ast.synapses, ast.receptors, ast.skins, ast.organs)
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
        skins: Vec<Skin>,
        organs: Vec<OrganAst>,
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

        // Muscles become Actuator Edges + Origin Nodes
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
            
            // ANATOMICAL ANCHOR: Place muscle node at source for cleaner synaptic wiring
            let s_pos = topology.node_position(*source_id).unwrap_or_default();
            let m_node_id = topology.add_node(format!("MUSCLE_NODE_{}", muscle.id()), s_pos, None);
            muscle_map.insert(muscle.id().to_string(), m_node_id);
        }

        // CPGs (Nerves) become Neural Nodes
        for cpg in &cpgs {
            let id_str = cpg.id();
            let mut pos = Vector3 { x: 0.0, y: 2.1, z: 0.0 };

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

        // Skins become Integument Nodes
        for skin in skins {
            let anchor_id = bone_map.get(skin.target_bone_id()).ok_or_else(|| {
                ValidationError::MissingIdentifier(skin.target_bone_id().to_string())
            })?;
            let anchor_pos = topology.node_position(*anchor_id).unwrap_or_default();

            for (_i, hull) in skin.hulls().iter().enumerate() {
                let local_pos = Vector3 { x: hull.local_offset.0, y: hull.local_offset.1, z: hull.local_offset.2 };
                let world_pos = anchor_pos + local_pos;
                let h_node_id = topology.add_node(format!("SKIN_HULL_{}_{}", skin.id(), hull.id), world_pos, None);
                topology.add_edge(*anchor_id, h_node_id, format!("{}_{}", skin.id(), hull.id), EdgeType::Integument);
            }
        }

        // Organs become Visceral Nodes
        for organ in organs {
            let anchor_id = bone_map.get(&organ.anchor_bone_id).ok_or_else(|| {
                ValidationError::MissingIdentifier(organ.anchor_bone_id.clone())
            })?;
            let anchor_pos = topology.node_position(*anchor_id).unwrap_or_default();
            
            // Place organ slightly offset from anchor
            let world_pos = anchor_pos + Vector3 { x: 0.0, y: -0.05, z: 0.1 }; 
            let o_node_id = topology.add_node(format!("ORGAN_NODE_{}", organ.id), world_pos, None);
            topology.add_edge(*anchor_id, o_node_id, organ.id.clone(), EdgeType::Visceral);
        }

        Ok(topology)
    }

    /// Creates a physics World initialized from source text.
    pub fn compile_world(&self, source: &str) -> Result<(World, Topology), CompileError> {
        let mut parser = Parser::new(source);
        let ast = parser.parse_organism()?;
        
        let topology = self.lower(
            ast.bones.clone(), 
            ast.joints.clone(), 
            ast.muscles.clone(), 
            ast.cpgs.clone(), 
            ast.synapses.clone(), 
            ast.receptors.clone(), 
            ast.skins.clone(),
            ast.organs.clone(),
        )?;

        let mut world = World::new(20);
        let mut bone_map: HashMap<String, usize> = HashMap::new();
        let mut actuator_map: HashMap<String, usize> = HashMap::new();
        let mut cpg_idx_map: HashMap<String, usize> = HashMap::new();

        for bone in &ast.bones {
            let idx = world.add_body(bone.position().x, bone.position().y, bone.position().z, 1.0);
            bone_map.insert(bone.id().to_string(), idx);
        }

        for cpg in &ast.cpgs {
            let idx = world.add_cpg(cpg.clone());
            cpg_idx_map.insert(cpg.id().to_string(), idx);
        }

        for joint in &ast.joints {
            let s_idx = bone_map.get(joint.source_bone_id()).unwrap();
            let t_idx = bone_map.get(joint.target_bone_id()).unwrap();
            let b_s = ast.bones.iter().find(|b| b.id() == joint.source_bone_id()).unwrap();
            let b_t = ast.bones.iter().find(|b| b.id() == joint.target_bone_id()).unwrap();
            let dist = (b_s.position() - b_t.position()).length();
            
            let compliance = if joint.id().contains("Rib") || joint.id().contains("JLR") || joint.id().contains("JRR") { 0.8 } else { 0.0 };
            world.add_constraint(DistanceConstraint::new(*s_idx, *t_idx, dist, compliance));
        }

        for muscle in &ast.muscles {
            let s_idx = bone_map.get(muscle.source_bone_id()).unwrap();
            let t_idx = bone_map.get(muscle.target_bone_id()).unwrap();
            let actuator = MuscleActuator::new(*s_idx, *t_idx, 2000.0);
            let idx = world.add_actuator(actuator);
            actuator_map.insert(muscle.id().to_string(), idx);
        }

        for synapse in &ast.synapses {
            if let (Some(&c_idx), Some(&a_idx)) = (cpg_idx_map.get(synapse.source_cpg_id()), actuator_map.get(synapse.target_muscle_id())) {
                world.add_synapse(PhysicsSynapse {
                    source_cpg_idx: c_idx,
                    target_actuator_idx: a_idx,
                    weight: synapse.weight(),
                });
            }
        }

        Ok((world, topology))
    }
}

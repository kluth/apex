use crate::domain::air::topology::{NodeId, Topology};
use gltf_json as json;
use serde_json::to_vec;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Infrastructure Adapter for exporting AIR Topology to GLTF 2.0.
pub struct GltfExporter;

impl Default for GltfExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl GltfExporter {
    pub fn new() -> Self {
        Self
    }

    /// Exports the provided Topology to a .glb file.
    pub fn export_topology<P: AsRef<Path>>(
        &self,
        topology: &Topology,
        path: P,
    ) -> Result<(), String> {
        let mut root = json::Root::default();

        // 1. Map edges to identify parents
        let mut parent_map = HashMap::new();
        let mut edges_list = Vec::new();
        
        for edge in topology.edges() {
            let s = edge.source().index();
            let t = edge.target().index();
            parent_map.insert(t, s);
            edges_list.push((s, t));
        }

        // 2. Create Nodes
        let mut nodes = Vec::new();
        for i in 0..topology.node_count() {
            let id = NodeId::new(i);
            let display_name = topology.node_name(id).unwrap_or("Unknown");
            
            // CRITICAL: Standardized naming prefix for the viewer
            let gltf_name = format!("APEX_NODE_{}", display_name);
            
            let abs_pos = topology.node_position(id).unwrap_or_default();

            let translation = if let Some(&parent_idx) = parent_map.get(&i) {
                let parent_pos = topology.node_position(NodeId::new(parent_idx)).unwrap_or_default();
                let rel = abs_pos - parent_pos;
                [rel.x as f32, rel.y as f32, rel.z as f32]
            } else {
                [abs_pos.x as f32, abs_pos.y as f32, abs_pos.z as f32]
            };

            nodes.push(json::Node {
                camera: None,
                children: None,
                extensions: Default::default(),
                extras: None,
                matrix: None,
                mesh: None,
                name: Some(gltf_name),
                rotation: None,
                scale: None,
                translation: Some(translation),
                skin: None,
                weights: None,
            });
        }

        // 3. Link Hierarchy (Enforcing unique child entries)
        for (p, t) in edges_list {
            if p < nodes.len() && t < nodes.len() {
                let parent_node = &mut nodes[p];
                if parent_node.children.is_none() {
                    parent_node.children = Some(Vec::new());
                }
                let children = parent_node.children.as_mut().unwrap();
                let child_idx = json::Index::new(t as u32);
                if !children.contains(&child_idx) {
                    children.push(child_idx);
                }
            }
        }

        // 4. Scene setup
        let root_nodes: Vec<_> = (0..nodes.len())
            .filter(|i| !parent_map.contains_key(i))
            .map(|i| json::Index::new(i as u32))
            .collect();

        let scene_idx = root.push(json::Scene {
            extensions: Default::default(),
            extras: Default::default(),
            name: Some("APEX Organism".to_string()),
            nodes: root_nodes,
        });
        root.scene = Some(scene_idx);
        root.nodes = nodes;

        // 5. Package GLB
        let json_data = to_vec(&root).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(b"glTF").map_err(|e| e.to_string())?;
        file.write_all(&2u32.to_le_bytes()).map_err(|e| e.to_string())?;
        
        let padding = (4 - (json_data.len() % 4)) % 4;
        let padded_len = (json_data.len() + padding) as u32;
        file.write_all(&(12 + 8 + padded_len).to_le_bytes()).map_err(|e| e.to_string())?;
        file.write_all(&padded_len.to_le_bytes()).map_err(|e| e.to_string())?;
        file.write_all(b"JSON").map_err(|e| e.to_string())?;
        file.write_all(&json_data).map_err(|e| e.to_string())?;
        for _ in 0..padding { file.write_all(b" ").map_err(|e| e.to_string())?; }

        Ok(())
    }
}

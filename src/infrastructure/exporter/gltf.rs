use crate::domain::air::topology::{NodeId, Topology};
use gltf_json as json;
use serde_json::to_vec;
use std::collections::{HashMap, HashSet};
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
        for edge in topology.edges() {
            parent_map.insert(edge.target().index(), edge.source().index());
        }

        // 2. Create Nodes with Relative Positions
        let mut nodes = Vec::new();
        for i in 0..topology.node_count() {
            let id = NodeId::new(i);
            let name = topology.node_name(id).map(|s| s.to_string());
            let abs_pos = topology.node_position(id).unwrap_or_default();

            let translation = if let Some(&parent_idx) = parent_map.get(&i) {
                let parent_pos = topology
                    .node_position(NodeId::new(parent_idx))
                    .unwrap_or_default();
                let rel_pos = abs_pos - parent_pos;
                [rel_pos.x as f32, rel_pos.y as f32, rel_pos.z as f32]
            } else {
                [abs_pos.x as f32, abs_pos.y as f32, abs_pos.z as f32]
            };

            nodes.push(json::Node {
                camera: None,
                children: None,
                extensions: Default::default(),
                extras: Default::default(),
                matrix: None,
                mesh: None,
                name,
                rotation: None,
                scale: None,
                translation: Some(translation),
                skin: None,
                weights: None,
            });
        }

        // 3. Reconstruct Hierarchy
        let mut child_nodes = HashSet::new();
        for edge in topology.edges() {
            let parent_idx = edge.source().index();
            let child_idx = edge.target().index();

            if parent_idx < nodes.len() && child_idx < nodes.len() {
                let parent_node = &mut nodes[parent_idx];
                if parent_node.children.is_none() {
                    parent_node.children = Some(Vec::new());
                }
                parent_node
                    .children
                    .as_mut()
                    .unwrap()
                    .push(json::Index::new(child_idx as u32));
                child_nodes.insert(child_idx);
            }
        }

        // 4. Create Scene with Root Nodes
        let mut root_nodes = Vec::new();
        for i in 0..nodes.len() {
            if !child_nodes.contains(&i) {
                root_nodes.push(json::Index::new(i as u32));
            }
        }

        let scene_idx = root.push(json::Scene {
            extensions: Default::default(),
            extras: Default::default(),
            name: Some("APEX Organism".to_string()),
            nodes: root_nodes,
        });
        root.scene = Some(scene_idx);
        root.nodes = nodes;

        // 5. Serialize and Package as GLB
        let json_data = to_vec(&root).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(b"glTF").map_err(|e| e.to_string())?;
        file.write_all(&2u32.to_le_bytes())
            .map_err(|e| e.to_string())?;

        let json_len = json_data.len() as u32;
        let padding = (4 - (json_len % 4)) % 4;
        let padded_json_len = json_len + padding;
        let total_len = 12 + 8 + padded_json_len;

        file.write_all(&total_len.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(&padded_json_len.to_le_bytes())
            .map_err(|e| e.to_string())?;
        file.write_all(b"JSON").map_err(|e| e.to_string())?;
        file.write_all(&json_data).map_err(|e| e.to_string())?;
        for _ in 0..padding {
            file.write_all(b" ").map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::air::topology::Topology;
    use crate::domain::biomechanics::rigid_body::Vector3;
    use std::fs;

    #[test]
    fn test_gltf_export_file_creation() {
        let mut topology = Topology::new();
        topology.add_node("Head".to_string(), Vector3::default());

        let exporter = GltfExporter::new();
        let path = "test_export.glb";
        exporter.export_topology(&topology, path).unwrap();

        assert!(Path::new(path).exists());
        let _ = fs::remove_file(path);
    }
}

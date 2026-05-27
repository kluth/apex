use crate::domain::air::topology::{Topology, NodeId};
use gltf_json as json;
use serde_json::to_vec;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::collections::HashSet;

/// Infrastructure Adapter for exporting AIR Topology to GLTF 2.0.
pub struct GltfExporter;

impl GltfExporter {
    pub fn new() -> Self {
        Self
    }

    /// Exports the provided Topology to a .glb file.
    pub fn export_topology<P: AsRef<Path>>(&self, topology: &Topology, path: P) -> Result<(), String> {
        let mut root = json::Root::default();

        // 1. Create Nodes
        let mut nodes = Vec::new();
        for i in 0..topology.node_count() {
            let name = topology.node_name(NodeId::new(i)).map(|s| s.to_string());
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
                translation: None,
                skin: None,
                weights: None,
            });
        }

        // 2. Reconstruct Hierarchy from Edges
        let mut child_nodes = HashSet::new();
        for edge in topology.edges() {
            let parent_idx = edge.source().index();
            let child_idx = edge.target().index();
            
            if parent_idx < nodes.len() && child_idx < nodes.len() {
                let parent_node = &mut nodes[parent_idx];
                if parent_node.children.is_none() {
                    parent_node.children = Some(Vec::new());
                }
                parent_node.children.as_mut().unwrap().push(json::Index::new(child_idx as u32));
                child_nodes.insert(child_idx);
            }
        }

        // 3. Create Scene with Root Nodes (nodes that are not children of any other node)
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

        // 4. Serialize to JSON
        let json_data = to_vec(&root).map_err(|e| e.to_string())?;

        // 5. Package as GLB
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(b"glTF").map_err(|e| e.to_string())?; 
        file.write_all(&2u32.to_le_bytes()).map_err(|e| e.to_string())?; 
        
        let json_len = json_data.len() as u32;
        let padding = (4 - (json_len % 4)) % 4;
        let padded_json_len = json_len + padding;
        let total_len = 12 + 8 + padded_json_len; 

        file.write_all(&total_len.to_le_bytes()).map_err(|e| e.to_string())?;
        file.write_all(&padded_json_len.to_le_bytes()).map_err(|e| e.to_string())?;
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
    use std::fs;

    #[test]
    fn test_gltf_export_file_creation() {
        let mut topology = Topology::new();
        topology.add_node("Head".to_string());
        
        let exporter = GltfExporter::new();
        let path = "test_export.glb";
        exporter.export_topology(&topology, path).unwrap();
        
        assert!(Path::new(path).exists());
        
        // Cleanup
        let _ = fs::remove_file(path);
    }
}

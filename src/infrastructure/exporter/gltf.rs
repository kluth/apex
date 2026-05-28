use crate::domain::air::topology::{NodeId, Topology};
use gltf_json as json;
use serde_json::to_vec;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Infrastructure Adapter for exporting AIR Topology to GLTF 2.0.
/// Uses a 'Flat-Hierarchical' strategy: Absolute coordinates for stability,
/// but restores parent-child links for generalistic rendering.
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

    pub fn export_topology<P: AsRef<Path>>(
        &self,
        topology: &Topology,
        path: P,
    ) -> Result<(), String> {
        let mut root = json::Root::default();

        // 1. Identify parents based on structural edges
        let mut parent_map = HashMap::new();
        let mut children_map: HashMap<usize, Vec<usize>> = HashMap::new();
        
        for edge in topology.edges() {
            let s = edge.source().index();
            let t = edge.target().index();
            parent_map.insert(t, s);
            children_map.entry(s).or_default().push(t);
        }

        // 2. Create Nodes with Absolute Coordinates
        let mut nodes = Vec::new();
        for i in 0..topology.node_count() {
            let id = NodeId::new(i);
            let display_name = topology.node_name(id).unwrap_or("Unknown");
            let pos = topology.node_position(id).unwrap_or_default();

            // We use absolute coordinates in translation. 
            // In a flat hierarchy (all nodes as roots), this is standard.
            // When nesting, we must subtract parent coordinates.
            let translation = if let Some(&parent_idx) = parent_map.get(&i) {
                let p_pos = topology.node_position(NodeId::new(parent_idx)).unwrap_or_default();
                let rel = pos - p_pos;
                [rel.x as f32, rel.y as f32, rel.z as f32]
            } else {
                [pos.x as f32, pos.y as f32, pos.z as f32]
            };

            nodes.push(json::Node {
                camera: None,
                children: None, // Will be filled in next step
                extensions: Default::default(),
                extras: None,
                matrix: None,
                mesh: None,
                name: Some(format!("APEX_NODE_{}", display_name)),
                rotation: None,
                scale: None,
                translation: Some(translation),
                skin: None,
                weights: None,
            });
        }

        // 3. Populate children indices (The Hierarchy)
        let mut all_children = HashSet::new();
        for (parent_idx, children) in children_map {
            let mut child_indices = Vec::new();
            for c_idx in children {
                child_indices.push(json::Index::new(c_idx as u32));
                all_children.insert(c_idx);
            }
            nodes[parent_idx].children = Some(child_indices);
        }

        // 4. Setup Scene (Only nodes without parents are scene roots)
        let root_nodes: Vec<_> = (0..nodes.len())
            .filter(|i| !all_children.contains(i))
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

        // 5. Serialize and Package
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::air::topology::Topology;
    use crate::domain::biomechanics::rigid_body::Vector3;
    use std::fs;

    #[test]
    fn test_gltf_export_file_creation() {
        let mut topology = Topology::new();
        topology.add_node("Head".to_string(), Vector3::default(), None);

        let exporter = GltfExporter::new();
        let path = "test_export.glb";
        exporter.export_topology(&topology, path).unwrap();

        assert!(Path::new(path).exists());
        let _ = fs::remove_file(path);
    }
}

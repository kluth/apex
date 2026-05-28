use crate::domain::air::topology::{NodeId, Topology};
use gltf_json as json;
use serde_json::to_vec;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Infrastructure Adapter for exporting AIR Topology to GLTF 2.0.
/// Uses a 'Point-Cloud' strategy where every node is a root to avoid transform drift.
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

        // 1. Create Nodes (Strictly flat, absolute coordinates)
        let mut nodes = Vec::new();
        for i in 0..topology.node_count() {
            let id = NodeId::new(i);
            let display_name = topology.node_name(id).unwrap_or("Unknown");
            let pos = topology.node_position(id).unwrap_or_default();

            nodes.push(json::Node {
                camera: None,
                children: None, // No hierarchy in the 3D file itself
                extensions: Default::default(),
                extras: None,
                matrix: None,
                mesh: None,
                name: Some(format!("APEX_NODE_{}", display_name)),
                rotation: None,
                scale: None,
                translation: Some([pos.x as f32, pos.y as f32, pos.z as f32]),
                skin: None,
                weights: None,
            });
        }

        // 2. Setup Scene (All nodes are top-level roots)
        let scene_idx = root.push(json::Scene {
            extensions: Default::default(),
            extras: Default::default(),
            name: Some("APEX Point Cloud".to_string()),
            nodes: (0..nodes.len()).map(|i| json::Index::new(i as u32)).collect(),
        });
        root.scene = Some(scene_idx);
        root.nodes = nodes;

        // 3. Package as binary GLB
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

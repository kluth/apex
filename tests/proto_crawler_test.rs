use apex::infrastructure::exporter::gltf::GltfExporter;
use apex::prelude::*;
use std::fs;
use std::path::Path;

#[test]
fn test_compile_and_export_proto_crawler() {
    let source = include_str!("../examples/proto_crawler.apex");
    let pipeline = CompilerPipeline::new();

    let topology = pipeline
        .compile(source)
        .expect("Failed to compile proto_crawler.apex");

    // 1. Verify Topology (2 bones + 1 virtual muscle node)
    assert_eq!(topology.node_count(), 3);

    // 2. Export to GLTF
    let exporter = GltfExporter::new();
    let export_path = "proto_crawler.glb";
    exporter
        .export_topology(&topology, export_path)
        .expect("Failed to export GLB");

    assert!(Path::new(export_path).exists());

    // 3. Verify GLB Header (Magic bytes)
    let bytes = fs::read(export_path).unwrap();
    assert_eq!(&bytes[0..4], b"glTF");

    // Cleanup
    let _ = fs::remove_file(export_path);
}

use apex::prelude::*;
use std::fs;

fn main() -> Result<(), String> {
    println!("APEX Human Anatomy Exporter");
    
    // 1. Load the Human Skeleton model
    let source = fs::read_to_string("examples/human_skeleton.apex")
        .map_err(|e| format!("Failed to read .apex file: {}", e))?;
    
    // 2. Initialize the Compiler Pipeline
    let pipeline = CompilerPipeline::new();
    
    // 3. Compile to AIR Topology
    println!("Compiling Human Anatomy...");
    let topology = pipeline.compile(&source)
        .map_err(|e| format!("Compilation failed: {:?}", e))?;
    
    // 4. Export to GLB for Three.js / Blender
    let exporter = GltfExporter::new();
    let output_path = "human_skeleton.glb";
    
    println!("Exporting to {}...", output_path);
    exporter.export_topology(&topology, output_path)?;
    
    println!("Done! Model exported successfully.");
    Ok(())
}

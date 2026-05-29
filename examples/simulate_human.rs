use apex::prelude::*;
use apex::infrastructure::exporter::gltf::GltfExporter;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), String> {
    println!("APEX Real-Time Locomotion Simulator");

    let pipeline = CompilerPipeline::new();
    let exporter = GltfExporter::new();

    // 1. Compile the Sentient Organism into a Physics World
    let source = fs::read_to_string("examples/human_skeleton.apex")
        .map_err(|e| e.to_string())?;
    
    println!("Compiling complex biomechanical model...");
    let (mut world, mut topology) = pipeline.compile_world(&source)
        .map_err(|e| format!("{:?}", e))?;

    println!("Simulation started. 60 FPS loop active.");
    println!("Outputting live to human_skeleton.glb...");

    let mut time = 0.0;
    let dt = 0.016; // 60 FPS

    loop {
        // 2. Step Physics & Neural Control
        // This now automatically updates CPGs and activates Muscles
        world.step(dt);
        time += dt;

        // 3. Sync positions back to Topology for visual export
        world.sync_to_topology(&mut topology);

        // 4. Periodic Export (Every 0.1s for smoothness in viewer)
        if (time * 100.0).round() % 10.0 < 2.0 {
            if let Err(e) = exporter.export_topology(&topology, "human_skeleton.glb") {
                eprintln!("Export error: {}", e);
            }
            // print!("\rSimulation time: {:.2}s", time);
            // std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }

        thread::sleep(Duration::from_millis(16));
        
        // Safety break after 5 minutes
        if time > 300.0 { break; }
    }

    Ok(())
}

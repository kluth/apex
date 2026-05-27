# ADR-015: GLTF Visualization Egress

## Context
APEX models exist as abstract topological graphs. To visualize these organisms in standard 3D tools (Three.js, Blender, Unity), we need a portable, efficient, and industry-standard interchange format that supports hierarchical skeletons and skeletal animation.

## Decision
APEX will implement a **GLTF 2.0 (GLB)** export adapter in the infrastructure layer.

### 1. Hierarchical Mapping
The flat AIR `Topology` will be reconstructed into a hierarchical tree. Each `NodeId` (Bone) will map to a `gltf::Node`. `EdgeId` (Joint) relationships will dictate the `children` array in the GLTF JSON manifest.

### 2. Coordinate System and Units
- **Mapping:** APEX internal coordinates will be mapped to the GLTF **Right-Handed, Y-Up** system.
- **Scaling:** SI units (meters) will be mapped 1:1 to GLTF units.

### 3. Binary Packaging (GLB)
To ensure ease of use, the exporter will prioritize the binary **.glb** format, which packages the JSON manifest and binary vertex/animation data into a single, self-contained file.

## Rationale
GLTF 2.0 is the "JPEG of 3D" and is natively supported by almost every modern 3D engine and web renderer. By providing a first-class GLTF egress, APEX enables researchers to instantly verify their biological models visually and leverage the vast ecosystem of 3D inspection and rendering tools.

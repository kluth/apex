# ADR-014: DOD Performance Architecture

## Context
As APEX models scale to include thousands of muscle fibers, skin particles, and skeletal nodes, the traditional Object-Oriented (AOS) approach becomes a significant bottleneck. Individual object overhead and non-contiguous memory access (pointer jumping) lead to high cache miss rates and prevent CPU SIMD units from operating at peak efficiency.

## Decision
APEX will transition its core simulation state to a **Data-Oriented Design (DOD)** utilizing a **Structure-of-Arrays (SOA)** memory layout.

### 1. Structure-of-Arrays (SOA)
Instead of a `Vec<RigidBody>`, we will implement a `BodyRegistry` that stores state in flattened, contiguous component arrays:
- `pos_x: Vec<f64>`, `pos_y: Vec<f64>`, `pos_z: Vec<f64>`
- `vel_x: Vec<f64>`, `vel_y: Vec<f64>`, `vel_z: Vec<f64>`
- `inv_mass: Vec<f64>`

### 2. SIMD (Single Instruction, Multiple Data)
The SOA layout enables "vertical" SIMD loading. We will use the `ultraviolet` crate to process blocks of 4 or 8 bodies in a single clock cycle during the prediction and velocity update passes.

### 3. Parallel Constraint Groups (Graph Coloring)
To parallelize the iterative Gauss-Seidel solver, constraints will be grouped into "colors." A color represents a batch of constraints where no two constraints share the same body index. This allows each color batch to be solved in parallel across multiple CPU cores without synchronization overhead or race conditions.

## Rationale
DOD is the modern standard for high-performance physics and biological simulation. Transitioning to SOA/SIMD provides a theoretical 4x-8x speedup on mathematical operations and significantly reduces memory latency. Graph coloring enables deterministic parallelism, ensuring that APEX remains stable and byte-for-byte identical even when executed across different thread counts.

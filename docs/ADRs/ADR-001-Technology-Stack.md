# ADR-001: Technology Stack

## Context
APEX requires a foundational technology stack capable of executing high-performance, deterministic biological simulations with zero garbage collection overhead. It also needs to support cross-platform execution, including WebAssembly for distributed nodes.

## Decision
APEX will be constructed entirely in **Rust** (Edition 2024), leveraging the `cargo` ecosystem. The compilation pipeline will output cross-platform native binaries and WebAssembly (Wasm) targets.

## Resolution (IPC Egress)
To decouple the deterministic simulation core from various rendering engines (Godot, Bevy) without sacrificing execution speed, we implement a strict Hexagonal Egress Port. Local real-time visualization is achieved via Zero-Copy `rkyv` serialization mapped to a lock-free IPC ring buffer over mmap.

## Rationale
Rust is the only mainstream language that enforces the "Parse, Don't Validate" paradigm at zero runtime cost via Algebraic Data Types (ADTs) and the Borrow Checker. It guarantees memory safety without garbage collection, ensuring continuous muscle-activation integration without nanosecond stutter. The zero-copy IPC ensures $O(1)$ overhead for visualizers.
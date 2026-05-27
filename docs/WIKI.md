# APEX Knowledge Base & Wiki

## Architecture Decision Records (ADRs)
Foundational decisions regarding the APEX architecture are permanently recorded here:
1. [ADR-001: Technology Stack](./ADRs/ADR-001-Technology-Stack.md)
2. [ADR-002: Type System Paradigm](./ADRs/ADR-002-Type-System-Paradigm.md)
3. [ADR-003: Unit System](./ADRs/ADR-003-Unit-System.md)
4. [ADR-004: Memory Model and Concurrency](./ADRs/ADR-004-Memory-Model.md)
5. [ADR-005: Determinism and Biological Timescales](./ADRs/ADR-005-Determinism.md)
6. [ADR-006: Fantasy and Alien Boundary](./ADRs/ADR-006-Fantasy-Boundary.md)
7. [ADR-007: Skeletal Joint Constraints](./ADRs/ADR-007-Skeletal-Joint-Constraints.md)
8. [ADR-008: Muscle Model Jacobians](./ADRs/ADR-008-Muscle-Model-Jacobians.md)
9. [ADR-009: Synaptic Wiring and Proprioception](./ADRs/ADR-009-Synaptic-Wiring-Proprioception.md)
10. [ADR-010: Soft-Body Integument and Collision](./ADRs/ADR-010-Soft-Body-Integument.md)
11. [ADR-011: Time-Stepping and Substepping Strategy](./ADRs/ADR-011-Substepping-Strategy.md)
12. [ADR-012: Compiler Architecture](./ADRs/ADR-012-Compiler-Architecture.md)
13. [ADR-013: External Forces and Damping](./ADRs/ADR-013-Forces-and-Damping.md)
14. [ADR-014: DOD Performance Architecture](./ADRs/ADR-014-DOD-Performance-Architecture.md)
15. [ADR-015: GLTF Visualization Egress](./ADRs/ADR-015-GLTF-Visualization-Egress.md)

## The Living Domain Lexicon (Ubiquitous Language)

This glossary defines the explicit terms used within the APEX source code. To maintain the **Screaming Architecture**, these terms must map 1:1 to structs, interfaces, and modules within the system.

...
### Infrastructure: Telemetry & Egress
* **`init_telemetry` (Ecosystem Standard):** Initializes the tracing and logging subscriber.
* **`GltfExporter` (Egress Adapter):** Translates optimized AIR Topology into standard GLTF 2.0 files for external 3D visualization.
* **`prelude` (Ecosystem Standard):** The unified entry point for the APEX standard library, re-exporting all core domain and application types.

## Engineering Directives
...

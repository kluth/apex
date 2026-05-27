# APEX Knowledge Base & Wiki

## Architecture Decision Records (ADRs)
Foundational decisions regarding the APEX architecture are permanently recorded here:
1. [ADR-001: Technology Stack](./ADRs/ADR-001-Technology-Stack.md)
2. [ADR-002: Type System Paradigm](./ADRs/ADR-002-Type-System-Paradigm.md)
3. [ADR-003: Unit System](./ADRs/ADR-003-Unit-System.md)
4. [ADR-004: Memory Model and Concurrency](./ADRs/ADR-004-Memory-Model.md)
5. [ADR-005: Determinism and Biological Timescales](./ADRs/ADR-005-Determinism.md)
6. [ADR-006: Fantasy and Alien Boundary](./ADRs/ADR-006-Fantasy-Boundary.md)

## The Living Domain Lexicon (Ubiquitous Language)

This glossary defines the explicit terms used within the APEX source code. To maintain the **Screaming Architecture**, these terms must map 1:1 to structs, interfaces, and modules within the system.

### Domain: Abstract Syntax Tree (AST)
* **`Bone` (Aggregate Root):** A representation of a rigid physical body in the simulation context. It holds intrinsic properties (e.g., `Mass`).
* **`Mass` (Value Object):** A strictly validated scalar representing weight in kilograms. Mathematically cannot be negative or NaN.

### Domain: Anatomy Intermediate Representation (AIR)
* **`Topology` (Aggregate):** The memory-contiguous, data-oriented graph mapping the entire organism. It abstracts away pointer jumping.
* **`NodeId` (Value Object):** A type-safe array index representing an anatomical node (like a `Bone`) in the Topology arena.
* **`EdgeId` (Value Object):** A type-safe array index representing an anatomical connection (like a `Joint`) in the Topology arena.
* **`Edge` (Entity):** The structured relationship connecting a source `NodeId` to a target `NodeId`.

### Domain: Biomechanics (XPBD)
* **`RigidBody` (Entity):** The core physical solver entity. Isolates spatial arrays ($x$, $v$) from constraints to satisfy XPBD stability requirements.
* **`inverse_mass` (Property):** Stored implicitly. A value of `0.0` mathematically translates to infinite stiffness (a static unmovable object), allowing alien/mythic materials without solver explosions.

### Application: Compiler Pipeline
* **`CompilerPipeline` (Application Service):** Orchestrates the transformation of high-level AST domain objects into optimized AIR topology.
* **`BiologicalValidator` (Domain Service):** Enforces biological plausibility (e.g., uniqueness of identifiers) during the compilation pass.

### Domain: Movement
* **`Cpg` (Value Object/Entity):** Central Pattern Generator. A periodic oscillator that drives biological rhythms (e.g., gait, heart rate) by outputting a rhythmic activation signal.

## Engineering Directives
1. **Mutation & Iteration:** All structural iterations must happen inside `domain`. Visualizations are ejected via the Egress Port (`rkyv` lock-free ring buffers).
2. **Technical Debt:** Any function violating a McCabe score of 10 must be refactored instantly. No PR will merge otherwise.
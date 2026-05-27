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

## The Living Domain Lexicon (Ubiquitous Language)

This glossary defines the explicit terms used within the APEX source code. To maintain the **Screaming Architecture**, these terms must map 1:1 to structs, interfaces, and modules within the system.

### Domain: Abstract Syntax Tree (AST)
* **`Bone` (Aggregate Root):** A representation of a rigid physical body in the simulation context. It holds intrinsic properties (e.g., `Mass`).
* **`Mass` (Value Object):** A strictly validated scalar representing weight in kilograms. Mathematically cannot be negative or NaN.
* **`Joint` (Aggregate Root):** A kinematic connection between two `Bone` entities. It restricts degrees of freedom (DOF) to simulate anatomical joints.
* **`JointType` (Value Object):** Specifies the mechanical behavior of a joint (e.g., `Spherical`, `Revolute`).
* **`JointAttachment` (Value Object):** The local offset from a bone's center where a joint is anchored.

### Domain: Anatomy Intermediate Representation (AIR)
* **`Topology` (Aggregate):** The memory-contiguous, data-oriented graph mapping the entire organism. It abstracts away pointer jumping.
* **`NodeId` (Value Object):** A type-safe array index representing an anatomical node (like a `Bone`) in the Topology arena.
* **`EdgeId` (Value Object):** A type-safe array index representing an anatomical connection (like a `Joint`) in the Topology arena.
* **`Edge` (Entity):** The structured relationship connecting a source `NodeId` to a target `NodeId`.

### Domain: Biomechanics (XPBD)
* **`RigidBody` (Entity):** The core physical solver entity. Isolates spatial arrays ($x$, $v$) from constraints to satisfy XPBD stability requirements.
* **`inverse_mass` (Property):** Stored implicitly. A value of `0.0` mathematically translates to infinite stiffness (a static unmovable object), allowing alien/mythic materials without solver explosions.
* **`XpbdConstraint` (Trait):** The mathematical contract for all physical constraints. Implements the total Lagrange multiplier update rule.
* **`DistanceConstraint` (Entity):** An XPBD constraint enforcing a specific distance (or 0-distance) between two points. Used for Spherical Joints.
* **`AngularConstraint` (Entity):** An XPBD constraint restricting rotation around one or more axes. Used for Revolute Joints.

### Application: Compiler Pipeline
* **`CompilerPipeline` (Application Service):** Orchestrates the transformation of high-level AST domain objects into optimized AIR topology.
* **`BiologicalValidator` (Domain Service):** Enforces biological plausibility (e.g., uniqueness of identifiers) during the compilation pass.

### Domain: Movement
* **`Cpg` (Value Object/Entity):** Central Pattern Generator. A periodic oscillator that drives biological rhythms (e.g., gait, heart rate) by outputting a rhythmic activation signal.

### Domain: Evolution & AI Integration
* **`FitnessEvaluator` (Trait/Port):** Defines the interface for evaluating organism performance.
* **`DistanceFitness` (Domain Service):** A specific fitness metric measuring Euclidean displacement, used to optimize for locomotion efficiency.

### Infrastructure: Telemetry
* **`init_telemetry` (Ecosystem Standard):** Initializes the tracing and logging subscriber.
* **`prelude` (Ecosystem Standard):** The unified entry point for the APEX standard library, re-exporting all core domain and application types.

## Engineering Directives
1. **Mutation & Iteration:** All structural iterations must happen inside `domain`. Visualizations are ejected via the Egress Port (`rkyv` lock-free ring buffers).
2. **Technical Debt:** Any function violating a McCabe score of 10 must be refactored instantly. No PR will merge otherwise.
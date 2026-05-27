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

## The Living Domain Lexicon (Ubiquitous Language)

This glossary defines the explicit terms used within the APEX source code. To maintain the **Screaming Architecture**, these terms must map 1:1 to structs, interfaces, and modules within the system.

### Domain: Abstract Syntax Tree (AST)
* **`Bone` (Aggregate Root):** A representation of a rigid physical body in the simulation context. It holds intrinsic properties (e.g., `Mass`).
* **`Mass` (Value Object):** A strictly validated scalar representing weight in kilograms. Mathematically cannot be negative or NaN.
* **`Joint` (Aggregate Root):** A kinematic connection between two `Bone` entities. It restricts degrees of freedom (DOF) to simulate anatomical joints.
* **`Muscle` (Aggregate Root):** A biological actuator connecting two bones (Origin and Insertion). It converts neural activation into mechanical force.
* **`MuscleAttachment` (Value Object):** Specifies the bone ID and local offset where a muscle is anchored.
* **`Synapse` (Entity/Aggregate):** A neural connection that wires a `Cpg` to a `Muscle`. It scales the neural signal by a `Weight` to determine activation.
* **`Skin` (Entity/Aggregate):** The outer shell of the organism. Composed of soft-body volumetric segments attached to the skeleton.
* **`CollisionHull` (Value Object):** A set of geometric primitives (spheres, boxes) used to calculate environmental interactions.

### Domain: Anatomy Intermediate Representation (AIR)
* **`Topology` (Aggregate):** The memory-contiguous, data-oriented graph mapping the entire organism. It abstracts away pointer jumping.
* **`NodeId` (Value Object):** A type-safe array index representing an anatomical node (like a `Bone`) in the Topology arena.
* **`EdgeId` (Value Object):** A type-safe array index representing an anatomical connection (like a `Joint`) in the Topology arena.
* **`Edge` (Entity):** The structured relationship connecting a source `NodeId` to a target `NodeId`.

### Domain: Biomechanics (XPBD & Myofascia & Integument)
* **`RigidBody` (Entity):** The core physical solver entity. Isolates spatial arrays ($x$, $v$) from constraints to satisfy XPBD stability requirements.
* **`HillCurve` (Domain Service):** Calculates the force output of a muscle based on its contractile and elastic properties.
* **`MuscleJacobian` (Entity/Service):** Maps 1D muscle tension into 3D world-space forces applied to bones.
* **`VolumeConstraint` (Entity):** An XPBD constraint that preserves the 3D volume of a tetrahedral segment of tissue.
* **`CollisionPrimitive` (Value Object):** Geometric shape used for ground contact detection.
* **`World` (Aggregate Root):** The top-level simulation container. Orchestrates the temporal evolution of all anatomical and neural entities.
* **`Solver` (Domain Service):** The XPBD engine responsible for enforcing physical constraints and calculating temporal state transitions.
* **`Substick` (Temporal Unit):** A subdivision of the global time step used to maintain mathematical stability in stiff systems.
* **`inverse_mass` (Property):** Stored implicitly. A value of `0.0` mathematically translates to infinite stiffness (a static unmovable object), allowing alien/mythic materials without solver explosions.

* **`XpbdConstraint` (Trait):** The mathematical contract for all physical constraints. Implements the total Lagrange multiplier update rule.
* **`DistanceConstraint` (Entity):** An XPBD constraint enforcing a specific distance (or 0-distance) between two points. Used for Spherical Joints.
* **`AngularConstraint` (Entity):** An XPBD constraint restricting rotation around one or more axes. Used for Revolute Joints.

### Application: Compiler Pipeline
* **`CompilerPipeline` (Application Service):** Orchestrates the transformation of high-level AST domain objects into optimized AIR topology.
* **`Lexer` (Compiler Component):** Tokenizes APEX source text into a stream of semantic symbols.
* **`Parser` (Compiler Component):** Implements recursive-descent logic to construct the AST from a token stream.
* **`BiologicalValidator` (Domain Service):** Enforces biological plausibility (e.g., uniqueness of identifiers) during the compilation pass.

### Domain: Movement & Nervous System
* **`Cpg` (Value Object/Entity):** Central Pattern Generator. A periodic oscillator that drives biological rhythms (e.g., gait, heart rate) by outputting a rhythmic activation signal.
* **`Proprioception` (Domain Service):** The "sixth sense" of the organism. Measures muscle length/velocity and modulates neural phase for reactive control.

### Domain: Evolution & AI Integration
* **`FitnessEvaluator` (Trait/Port):** Defines the interface for evaluating organism performance.
* **`DistanceFitness` (Domain Service):** A specific fitness metric measuring Euclidean displacement, used to optimize for locomotion efficiency.

### Infrastructure: Telemetry
* **`init_telemetry` (Ecosystem Standard):** Initializes the tracing and logging subscriber.
* **`prelude` (Ecosystem Standard):** The unified entry point for the APEX standard library, re-exporting all core domain and application types.

## Engineering Directives
1. **Mutation & Iteration:** All structural iterations must happen inside `domain`. Visualizations are ejected via the Egress Port (`rkyv` lock-free ring buffers).
2. **Technical Debt:** Any function violating a McCabe score of 10 must be refactored instantly. No PR will merge otherwise.

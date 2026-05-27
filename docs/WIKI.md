# APEX Knowledge Base & Wiki

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

## Engineering Directives
1. **Mutation & Iteration:** All structural iterations must happen inside `domain`. Visualizations are ejected via the Egress Port (`rkyv` lock-free ring buffers).
2. **Technical Debt:** Any function violating a McCabe score of 10 must be refactored instantly. No PR will merge otherwise.
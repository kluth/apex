# APEX
**Anatomical Procedural Expression Language**

APEX is a declarative, biological modeling and simulation operating system. Geometry, physiology, and movement emerge exclusively from anatomical relationships rather than explicit keyframe animations or direct coordinate manipulations.

**Live 3D Visualization:** [https://kluth.github.io/apex/](https://kluth.github.io/apex/)

## State of the Model: The Sentient Organism
The current implementation represents an anatomically complete human organism:
-   **Skeletal Layer**: ~206 high-fidelity bone nodes including full skull, vertebral column, and terminal phalanges.
-   **Myofascial Layer**: 150+ functional muscle groups (Actuators) with anatomical origin/insertion mapping.
-   **Neural Layer**: The 43 major nerve pairs (12 Cranial, 31 Spinal) modeled as Central Pattern Generators (CPGs).
-   **Sensory Layer**: Real-time proprioceptive feedback loops (Muscle Spindles & Golgi Tendon Organs).

## Architecture
APEX strictly adheres to **Hexagonal Architecture** and **Domain-Driven Design (DDD)**. 

### Core Paradigms
-   **Parse, Don't Validate:** Mathematical impossibilities (e.g., negative mass) are caught at compile time.
-   **Data-Oriented Design (DOD):** Memory is laid out in strict Arena-based structures (`Topology`) for cache-line locality and rapid ECS iteration.
-   **Extended Position Based Dynamics (XPBD):** Used for unconditionally stable simulation of extremely stiff materials.
-   **Modular DSL**: Supports nested `include` directives for high-fidelity multi-file anatomical modeling.

## Documentation 
- **[CHANGELOG](./CHANGELOG.md)**: Sequential version tracking.
- **[WIKI](./docs/WIKI.md)**: The Living Domain Lexicon and architectural guidelines.
- **[GEMINI AUDIT LEDGER](./GEMINI.md)**: Immutable chronological AI operational ledger.

### Architecture Decision Records (ADRs)
- [ADR-001: Technology Stack](./docs/ADRs/ADR-001-Technology-Stack.md)
- [ADR-011: Lagrange Multiplier Physics](./docs/ADRs/ADR-011-lagrange-multiplier-physics.md)
- [ADR-014: SIMD Math Integration](./docs/ADRs/ADR-014-simd-math.md)
- [ADR-016: Mesh Asset Binding](./docs/ADRs/ADR-016-mesh-binding.md)

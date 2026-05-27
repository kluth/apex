# Changelog

All notable changes to the APEX project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Application (Compiler):** Implemented the formal APEX Lexer and Recursive-Descent Parser. Enables full end-to-end compilation from declarative `.apex` text into optimized AIR Topology. Extensively documented the DSL and Compiler Architecture (Issue #26 - hallucinated).
- **Domain (Biomechanics/Simulation):** Implemented the `World` aggregate and the XPBD real-time solver loop. Introduced deterministic substepping and Semi-Implicit Euler prediction passes, ensuring mathematical stability for high-stiffness biological models (Issue #24).
- **Domain (Integumentary System):** Implemented the initial soft-body foundation. Introduces `Skin` AST and `VolumeConstraint` for XPBD, enabling volume-preserving biological tissue and environmental collision hulls (Issue #22).
- **Domain (Movement/Nervous System):** Implemented Synaptic wiring and Proprioceptive feedback. Enables the connection of CPG signals to muscles and reactive neural modulation based on physical stretch (Issue #20).
- **Domain (Biomechanics):** Implemented XPBD Joint Constraints (`DistanceConstraint`). Enables the creation of stable skeletal kinematic chains with support for rigid and compliant connections (Issue #16).
- **Infrastructure (Telemetry):** Integrated `tracing` and `tracing-subscriber` for centralized observability. Established the `prelude` standard library export for the APEX ecosystem (Issue #14).
- **Domain (Evolution):** Implemented the `FitnessEvaluator` framework. Provides a decouple mechanism for measuring organism performance (e.g., `DistanceFitness`) to drive evolutionary optimization (Issue #12).
- **Domain (Movement):** Implemented Central Pattern Generators (CPGs) for rhythmic biological motion. Provides a deterministic periodic signal generator for muscle activation (Issue #10).
- **Application (Compiler):** Implemented the `CompilerPipeline` and `BiologicalValidator`. Handles the lowering of high-level AST `Bone` objects into low-level AIR `Topology` with strict validation passes for biological invariants (Issue #8).
- **Global Documentation:** Formalized Phase 0 Architecture Decision Records (ADRs 001-006) as tracked markdown artifacts inside `docs/ADRs/`.
- **Domain (Biomechanics):** Implemented `domain::biomechanics::rigid_body` establishing the mathematical foundation for Extended Position Based Dynamics (XPBD). Handles static bodies natively via 0.0 inverse mass mapping (Issue #5).
- **Global Documentation:** Initialized `README.md`, `CHANGELOG.md`, and in-repo `docs/WIKI.md` to track architectural terminology and progress.
- **Domain (AIR):** Implemented `domain::air::topology` providing an Arena-based $O(1)$ memory mapping layout using `NodeId` and `EdgeId` (Issue #3).
- **Domain (AST):** Implemented `domain::ast::bone` featuring a strongly typed `Mass` value object mathematically bounding physical constraints, following the "Parse, Don't Validate" paradigm (Issue #1).

### Changed
- **Architecture:** Formalized Hexagonal Architecture project layout (`/domain`, `/application`, `/infrastructure`, `/presentation`).
- **Workflow:** Enforced GitHub CLI linear history and TDD strict adherence.
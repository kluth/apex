# Changelog

All notable changes to the APEX project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Domain (Biomechanics):** Implemented `domain::biomechanics::rigid_body` establishing the mathematical foundation for Extended Position Based Dynamics (XPBD). Handles static bodies natively via 0.0 inverse mass mapping (Issue #5).
- **Global Documentation:** Initialized `README.md`, `CHANGELOG.md`, and in-repo `docs/WIKI.md` to track architectural terminology and progress.
- **Domain (AIR):** Implemented `domain::air::topology` providing an Arena-based $O(1)$ memory mapping layout using `NodeId` and `EdgeId` (Issue #3).
- **Domain (AST):** Implemented `domain::ast::bone` featuring a strongly typed `Mass` value object mathematically bounding physical constraints, following the "Parse, Don't Validate" paradigm (Issue #1).

### Changed
- **Architecture:** Formalized Hexagonal Architecture project layout (`/domain`, `/application`, `/infrastructure`, `/presentation`).
- **Workflow:** Enforced GitHub CLI linear history and TDD strict adherence.
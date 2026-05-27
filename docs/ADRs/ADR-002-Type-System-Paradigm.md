# ADR-002: Type System Paradigm

## Context
Biological connections must be anatomically valid. A muscle cannot connect to a nerve without an intersection node, and tissues cannot exist without anchor points. Validating this at runtime is computationally expensive and prone to error.

## Decision
The APEX compiler utilizes **Affine / Linear Types** and **Marker Traits** (Dependent Type approximations) to enforce biological constraints statically.

## Rationale
By utilizing Rust's generic phantom data, state transitions occur at the type level. An illegal connection fails at compile time. This ensures that anatomical attachments are mathematically unrepresentable if they violate biomechanical reality.
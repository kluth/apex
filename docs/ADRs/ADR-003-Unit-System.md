# ADR-003: Unit System

## Context
Physics simulations are highly susceptible to scaling errors and unit mismatches (e.g., adding mass to distance).

## Decision
The system implements **Compile-Time Dimensional Analysis** using a zero-cost abstraction library.

## Rationale
Every variable in APEX is an SI-derived physical quantity. Mixing incompatible units triggers a hard compiler error. This ensures biomechanical integrators operate strictly on mathematically verified units, preventing scaling disasters in the solver.
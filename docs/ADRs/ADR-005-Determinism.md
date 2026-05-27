# ADR-005: Determinism and Biological Timescales

## Context
APEX models operate on divergent timescales: neural action potentials propagate in microseconds ($\mu s$), muscle twitches occur in milliseconds ($ms$), and bone remodeling/growth occurs over days.

## Decision
Execution relies on strict **Fixed-Point Arithmetic** (or rigidly configured IEEE-754) and **Multi-Rate Substepping**.

## Rationale
A monolithic simulation step is computationally unviable. We utilize Symplectic Euler / RK4 integration with isolated, reproducible sub-ticks. The simulation must produce byte-for-byte identical output regardless of the host CPU architecture.
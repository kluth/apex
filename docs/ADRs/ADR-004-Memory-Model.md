# ADR-004: Memory Model and Concurrency

## Context
Biological simulation requires iterating over tens of thousands of muscle fibers, synapses, and rigid bodies simultaneously. The memory layout directly dictates the maximum simulation speed.

## Decision
The simulation engine employs strict **Data-Oriented Design (DOD)** via an **Entity Component System (ECS)**. Concurrency utilizes a deterministic **Data-Parallel Fork-Join** model.

## Rationale
The Actor-Model introduces messaging overhead and non-determinism via race conditions. ECS ensures contiguous memory allocation (Cache Line Locality). All mutations are strictly scheduled in topological order based on anatomical dependency graphs.
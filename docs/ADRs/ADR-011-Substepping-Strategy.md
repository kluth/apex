# ADR-011: Time-Stepping and Substepping Strategy

## Context
Biological simulations involve stiff constraints (Bones, mythic materials) and fast-acting neural signals. A single large time step (e.g., 60Hz) is insufficient for stability in XPBD, especially when dealing with high-stiffness joints and fast muscular impulses.

## Decision
APEX will implement a **Deterministic Substepping Integrator** using a **Semi-Implicit Euler** prediction pass combined with **XPBD Iterative Gauss-Seidel** constraint resolution.

### 1. Fixed Time Steps
The simulation will operate on a fixed global time step $\Delta t$ (e.g., 0.016s for 60Hz visual sync).

### 2. Adaptive Substepping
To maintain stability, the solver will perform $N$ substicks per global step:
$$ dt = \Delta t / N $$
Where $N$ is determined by the maximum stiffness found in the current `Topology` (governed by ADR-006 compliance values).

### 3. XPBD Solver Loop (per substick)
1.  **Prediction:** Predict positions $\mathbf{x}^* = \mathbf{x} + \Delta t \cdot \mathbf{v} + \Delta t^2 \cdot \mathbf{w} \cdot \mathbf{f}_{ext}$.
2.  **Constraint Resolution:** Iteratively solve all constraints (Joints, Muscles, Volume) and update $\mathbf{x}^*$ and $\lambda$.
3.  **Velocity Update:** $\mathbf{v} = (\mathbf{x}^* - \mathbf{x}) / \Delta t$; $\mathbf{x} = \mathbf{x}^*$.

## Rationale
Substepping is the standard solution for stiff physical systems. Semi-Implicit Euler is preferred over RK4 for XPBD because PBD-based methods inherently handle position corrections, making the higher-order velocity integration of RK4 redundant and more computationally expensive. Fixed time steps ensure cross-platform determinism.

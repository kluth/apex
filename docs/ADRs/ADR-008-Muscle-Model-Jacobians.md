# ADR-008: Muscle Model Jacobians

## Context
A skeleton is passive. To generate emergent biological movement, we require actuators (Muscles). These actuators must be scientifically grounded, reactive to neural signals, and stable within the XPBD simulation.

## Decision
APEX will implement muscles using a **1D Hill-Type Muscle Model** mapped to the 3D simulation via **Jacobian Force Application**.

### The Hill-Type Model
We will model the force $F$ of a muscle as:
$$ F = F_{ce}(a, l, v) + F_{pee}(l) $$
Where:
- $F_{ce}$ (Contractile Element): Active force proportional to activation $a \in [0, 1]$, length $l$, and velocity $v$.
- $F_{pee}$ (Parallel Elastic Element): Passive force resisting stretch (non-linear spring).

### The Jacobian Mapping
Since our muscles are 1D actuators connecting 3D bones, we use a Jacobian $\mathbf{J}$ to translate the 1D force magnitude into 3D impulses:
$$ \mathbf{f} = \mathbf{J}^T F $$
This ensures that the forces applied to the bones are equal, opposite, and aligned with the muscle's line of action (Origin to Insertion).

## Rationale
The Hill-Type model is the industry and scientific standard for biomechanical simulation. Using Jacobians allows us to decouple the complex 1D physiological state of the muscle from the 3D rigid body dynamics of the skeleton, maintaining the "Parse, Don't Validate" and modular boundaries of the APEX architecture.

# ADR-010: Soft-Body Integument and Collision

## Context
An organism requires an outer shell (Skin/Fascia) for volumetric representation and environmental interaction (Collision). This soft tissue must remain stable when attached to a moving skeleton and must resist collapsing or self-intersecting.

## Decision
APEX will implement the Integumentary System using **XPBD Volumetric Constraints**.

### 1. Volumetric Preservation
We will use **Tetrahedral Volume Constraints**. Each segment of skin/fascia is modeled as a tetrahedron where the volume $V$ is constrained:
$$ C(\mathbf{x}_1, \mathbf{x}_2, \mathbf{x}_3, \mathbf{x}_4) = V(\mathbf{x}_1, \mathbf{x}_2, \mathbf{x}_3, \mathbf{x}_4) - V_0 = 0 $$
$$ V = \frac{1}{6} |(\mathbf{x}_2 - \mathbf{x}_1) \cdot ((\mathbf{x}_3 - \mathbf{x}_1) \times (\mathbf{x}_4 - \mathbf{x}_1))| $$
This ensures the skin "bulges" correctly and maintains its shape under pressure.

### 2. Collision Foundation
Initial environmental interaction will rely on **Penalty-Free XPBD Collision**. We will use Sphere and AABB primitives to represent high-fidelity collision hulls (e.g., foot pads). Collisions will be resolved as inequality constraints ($C(\mathbf{x}) \geq 0$) within the same XPBD solver loop as the joints and muscles, guaranteeing no jitter or "sinking" into the floor.

## Rationale
Integrating collision and volume into the unified XPBD solver is the most stable approach for complex biological simulations. It avoids the coupling of separate physics engines and ensures that skin deformation and bone movement are resolved in a single, deterministic topological pass.

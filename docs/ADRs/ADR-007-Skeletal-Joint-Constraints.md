# ADR-007: Skeletal Joint Constraints

## Context
A skeleton is a collection of rigid bodies (Bones) connected by kinematic constraints (Joints). Without joints, bones have 6 degrees of freedom (DOF). To simulate anatomically correct skeletons, we must restrict these DOFs using stable, deterministic math that can handle "mythic" stiffness.

## Decision
APEX will implement skeletal joints using **XPBD (Extended Position Based Dynamics)** constraints. We will prioritize two fundamental joint types:

1.  **Spherical Joint (Ball-and-Socket):** 3 rotational DOF, 0 translational DOF.
    - *Math:* Enforced as a point-to-point distance constraint where rest length $d_0 = 0$.
2.  **Revolute Joint (Hinge):** 1 rotational DOF, 0 translational DOF.
    - *Math:* Enforced by a Spherical Joint plus an angular constraint that aligns the bones along a specific hinge axis.

## Implementation Details
- **Positional Constraint:** $C(\mathbf{x}_1, \mathbf{x}_2) = |\mathbf{p}_1 - \mathbf{p}_2|$, where $\mathbf{p}_i$ are the world-space positions of the joint attachment points on each bone.
- **Angular Constraint:** Restricts relative rotation to a single axis $\mathbf{n}$.
- **Compliance ($\alpha$):** Every joint will have a compliance parameter. Earth-organic joints (ligaments/cartilage) will have non-zero compliance ($\alpha > 0$), while "mythic" or mechanical joints can approach perfect rigidity ($\alpha \to 0$).

## Rationale
XPBD is uniquely suited for skeletal chains because it avoids the "stiff differential" explosion seen in penalty-based methods. By using the total Lagrange multiplier update rule, joint stiffness remains consistent regardless of the simulation frequency or iteration count. This ensures that a "mythic" dragon wing or a human knee behaves identically across different hardware.

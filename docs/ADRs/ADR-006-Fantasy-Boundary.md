# ADR-006: Fantasy and Alien Boundary

## Context
Users modeling speculative fauna (e.g., carbon-nanotube musculature) require boundary relaxation beyond Earth-organic limits. However, extreme stiffness disparities cause traditional explicit integrators to explode mathematically (NaN velocities).

## Decision
Biological constraints are layered as **Parametric Boundaries** governed by a compiler pragma `#[apex(plausibility = "mythic")]`. To guarantee solver stability, APEX utilizes **Extended Position Based Dynamics (XPBD)**.

## Rationale
XPBD handles infinite stiffness gracefully by mapping stiffness directly to a constraint compliance value ($\alpha$). If a mythic tendon has infinite stiffness, compliance approaches zero, and the solver resolves it trivially without destabilizing the rigid body constraints. The compiler routes alien substrates exclusively through XPBD solver paths.
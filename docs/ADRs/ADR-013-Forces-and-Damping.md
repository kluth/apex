# ADR-013: External Forces and Damping

## Context
A biological simulation requires environmental forces like gravity to generate realistic motion (e.g., ground contact and gait). Additionally, without damping, biological systems become numerically unstable or physically implausible (infinite oscillations).

## Decision
APEX will integrate external forces and biological damping directly into the XPBD prediction and velocity update passes.

### 1. Gravity and External Forces
We will implement a global gravity vector $\mathbf{g}$ in the `World` aggregate. During the prediction pass, the predicted position $\mathbf{x}^*$ will be:
$$ \mathbf{x}^* = \mathbf{x} + \Delta t \cdot \mathbf{v} + \Delta t^2 \cdot \mathbf{w} \cdot (\mathbf{f}_{ext} + m \cdot \mathbf{g}) $$

### 2. Velocity Update (Post-Constraint)
After constraints are resolved, the true velocity for the next step is calculated from the positional displacement:
$$ \mathbf{v}_{next} = (\mathbf{x}_{new} - \mathbf{x}_{prev}) / \Delta t $$

### 3. Biological Damping
We will implement **Velocity Damping** to simulate air resistance and internal tissue friction. This will be applied at the end of the step:
$$ \mathbf{v}_{final} = \mathbf{v}_{next} \cdot (1 - \gamma \cdot \Delta t) $$
Where $\gamma$ is the damping coefficient.

## Rationale
Coupling damping and external forces with the XPBD loop maintains the method's inherent stability. Calculating velocity from displacement ($x_{new} - x_{prev}$) is a core tenet of PBD/XPBD, as it ensures that the velocity is always consistent with the satisfied constraints, preventing "ghost forces" from accumulating.

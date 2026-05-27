# ADR-009: Synaptic Wiring and Proprioception

## Context
A skeletal system with muscles requires a control layer to coordinate rhythmic and reactive movement. We need a way to route neural signals (CPGs) to actuators (Muscles) and provide sensory feedback (Proprioception) to enable closed-loop control.

## Decision
APEX will implement a **Neural Control Layer** consisting of Synaptic Effector pathways and Proprioceptive Feedback loops.

### 1. Synaptic Effector Path (Brain-to-Body)
We introduce the **Synapse**, which maps a CPG signal to a specific muscle's activation level.
$$ a_{muscle} = \sigma( \sum w_i \cdot s_{cpg, i} ) $$
Where $s$ is the CPG signal, $w$ is the synaptic weight, and $\sigma$ is a squashing function (clamping to $[0, 1]$).

### 2. Proprioceptive Feedback Path (Body-to-Brain)
We introduce **Proprioception**, allowing biological state (like muscle stretch) to modulate neural rhythms.
$$ \phi_{next} = \phi_{prev} + \Delta t \cdot f + K_{p} \cdot (l_{current} - l_{rest}) $$
Where $K_p$ is the feedback gain, effectively allowing the physical state to "pull" or "push" the neural phase.

## Rationale
This approach mirrors biological "Reflex Arcs" and Central Pattern Generators found in nature. By decoupling the control logic into discrete Synapse and Proprioceptor entities, we maintain the "Inner-to-Outer" anatomical integrity while providing a flexible interface for future AI/Evolutionary optimization.

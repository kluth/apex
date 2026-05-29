use crate::domain::air::topology::{NodeId, Topology};
use crate::domain::biomechanics::constraints::{MuscleActuator, XpbdConstraint};
use crate::domain::biomechanics::registry::BodyRegistry;
use crate::domain::biomechanics::rigid_body::Vector3;
use crate::domain::movement::cpg::Cpg;
use std::collections::HashSet;

/// Represents a neural connection within the physics world.
pub struct Synapse {
    pub source_cpg_idx: usize,
    pub target_actuator_idx: usize, // Index in the actuators list
    pub weight: f64,
}

/// The World aggregate root.
/// Orchestrates the temporal evolution of all anatomical and neural entities.
pub struct World {
    registry: BodyRegistry,
    constraints: Vec<Box<dyn XpbdConstraint>>,
    actuators: Vec<MuscleActuator>, // Special list for neural control
    constraint_batches: Vec<Vec<usize>>,
    cpgs: Vec<Cpg>,
    synapses: Vec<Synapse>,
    gravity: Vector3,
    substicks: usize,
    damping: f64,
}

impl World {
    pub fn new(substicks: usize) -> Self {
        Self {
            registry: BodyRegistry::new(),
            constraints: Vec::new(),
            actuators: Vec::new(),
            constraint_batches: Vec::new(),
            cpgs: Vec::new(),
            synapses: Vec::new(),
            gravity: Vector3 {
                x: 0.0,
                y: -9.81,
                z: 0.0,
            },
            substicks,
            damping: 0.01,
        }
    }

    pub fn set_gravity(&mut self, g: Vector3) {
        self.gravity = g;
    }

    pub fn add_body(&mut self, x: f64, y: f64, z: f64, mass: f64) -> usize {
        self.registry.add_body(x, y, z, mass)
    }

    pub fn add_constraint<C: XpbdConstraint + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
        self.rebuild_constraint_batches();
    }

    pub fn add_actuator(&mut self, actuator: MuscleActuator) -> usize {
        let idx = self.actuators.len();
        self.actuators.push(actuator);
        idx
    }

    pub fn add_cpg(&mut self, cpg: Cpg) -> usize {
        let idx = self.cpgs.len();
        self.cpgs.push(cpg);
        idx
    }

    pub fn add_synapse(&mut self, synapse: Synapse) {
        self.synapses.push(synapse);
    }

    fn rebuild_constraint_batches(&mut self) {
        let mut batches: Vec<Vec<usize>> = Vec::new();

        for (i, constraint) in self.constraints.iter().enumerate() {
            let affected = constraint.affected_indices();
            let mut found_batch = false;

            for batch in &mut batches {
                let mut conflict = false;
                for &other_idx in batch.iter() {
                    let other_affected = self.constraints[other_idx].affected_indices();
                    let set: HashSet<_> = affected.iter().collect();
                    if other_affected.iter().any(|idx| set.contains(idx)) {
                        conflict = true;
                        break;
                    }
                }

                if !conflict {
                    batch.push(i);
                    found_batch = true;
                    break;
                }
            }

            if !found_batch {
                batches.push(vec![i]);
            }
        }

        self.constraint_batches = batches;
    }

    pub fn step(&mut self, global_dt: f64) {
        let substick_dt = global_dt / (self.substicks as f64);

        // 1. Update Neural Layer (CPGs)
        for cpg in &mut self.cpgs {
            cpg.step(global_dt); // Neural layer updates at global rate
        }

        // 2. Apply Synaptic Weights to Actuators
        for synapse in &self.synapses {
            let signal = self.cpgs[synapse.source_cpg_idx].step(0.0); // Peek signal
            let activation = (signal * synapse.weight).max(0.0).min(1.0);
            self.actuators[synapse.target_actuator_idx].set_activation(activation);
        }

        // 3. Substepping Loop
        for _ in 0..self.substicks {
            self.substick(substick_dt);
        }
    }

    fn substick(&mut self, dt: f64) {
        // 1. Prediction Pass
        self.registry.predict_simd(dt, self.gravity.y);

        // Ground constraint (Hardcoded for stability)
        for i in 0..self.registry.len() {
            if self.registry.pos_y[i] < 0.0 {
                self.registry.pos_y[i] = 0.0;
                self.registry.vel_y[i] = 0.0;
            }
        }

        // 2. Parallel Constraint Resolution (Structural)
        for batch in &self.constraint_batches {
            for &idx in batch {
                self.constraints[idx].solve(&mut self.registry, dt, 0.0);
            }
        }

        // 3. Actuator Resolution (Muscles)
        for actuator in &self.actuators {
            actuator.solve(&mut self.registry, dt, 0.0);
        }

        // 4. Velocity Update
        let len = self.registry.len();
        let inv_dt = 1.0 / dt;
        for i in 0..len {
            if self.registry.inv_mass[i] > 0.0 {
                let vx = (self.registry.pos_x[i] - self.registry.prev_pos_x[i]) * inv_dt;
                let vy = (self.registry.pos_y[i] - self.registry.prev_pos_y[i]) * inv_dt;
                let vz = (self.registry.pos_z[i] - self.registry.prev_pos_z[i]) * inv_dt;

                self.registry.vel_x[i] = vx * (1.0 - self.damping);
                self.registry.vel_y[i] = vy * (1.0 - self.damping);
                self.registry.vel_z[i] = vz * (1.0 - self.damping);
            }
        }
    }

    pub fn registry(&self) -> &BodyRegistry {
        &self.registry
    }

    pub fn sync_to_topology(&self, topology: &mut Topology) {
        for i in 0..self.registry.len() {
            if let Some(node) = topology.get_node_mut(NodeId::new(i)) {
                node.position = Vector3 {
                    x: self.registry.pos_x[i],
                    y: self.registry.pos_y[i],
                    z: self.registry.pos_z[i],
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::biomechanics::constraints::DistanceConstraint;

    #[test]
    fn test_world_step_soa() {
        let mut world = World::new(10);
        let b1 = world.add_body(0.0, 0.0, 0.0, 1.0);
        let b2 = world.add_body(2.0, 0.0, 0.0, 1.0);

        world.add_constraint(DistanceConstraint::new(b1, b2, 1.0, 0.0));
        world.step(0.1);

        let dx = world.registry().pos_x[b1] - world.registry().pos_x[b2];
        let dist = dx.abs();
        assert!((dist - 1.0).abs() < 1e-1);
    }
}

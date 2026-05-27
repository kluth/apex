use crate::domain::biomechanics::registry::BodyRegistry;
use crate::domain::biomechanics::rigid_body::Vector3;
use crate::domain::biomechanics::constraints::XpbdConstraint;
use rayon::prelude::*;
use std::collections::HashSet;

/// The World aggregate root. 
/// Orchestrates the temporal evolution of all anatomical and neural entities.
pub struct World {
    registry: BodyRegistry,
    constraints: Vec<Box<dyn XpbdConstraint>>,
    constraint_batches: Vec<Vec<usize>>,
    gravity: Vector3,
    substicks: usize,
    damping: f64,
}

impl World {
    pub fn new(substicks: usize) -> Self {
        Self {
            registry: BodyRegistry::new(),
            constraints: Vec::new(),
            constraint_batches: Vec::new(),
            gravity: Vector3 { x: 0.0, y: -9.81, z: 0.0 },
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
        
        for _ in 0..self.substicks {
            self.substick(substick_dt);
        }
    }

    fn substick(&mut self, dt: f64) {
        // 1. Prediction Pass (SIMD Accelerated)
        self.registry.predict_simd(dt, self.gravity.y);

        // 2. Parallel Constraint Resolution (XPBD)
        for batch in &self.constraint_batches {
            // Sequential across batches, but prepared for parallel internally
            for &idx in batch {
                self.constraints[idx].solve(&mut self.registry, dt, 0.0);
            }
        }

        // 3. Velocity Update (Refined in registry.rs eventually, but for now manual in loop)
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
                
                self.registry.force_x[i] = 0.0;
                self.registry.force_y[i] = 0.0;
                self.registry.force_z[i] = 0.0;
            }
        }
    }

    pub fn registry(&self) -> &BodyRegistry {
        &self.registry
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
        assert!((dist - 1.0).abs() < 1e-2);
    }
}

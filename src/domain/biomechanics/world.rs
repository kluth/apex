use crate::domain::biomechanics::rigid_body::RigidBody;
use crate::domain::biomechanics::constraints::XpbdConstraint;

/// The World aggregate root. 
/// Orchestrates the temporal evolution of all anatomical and neural entities.
pub struct World {
    bodies: Vec<RigidBody>,
    constraints: Vec<Box<dyn XpbdConstraint>>,
    substicks: usize,
}

impl World {
    pub fn new(substicks: usize) -> Self {
        Self {
            bodies: Vec::new(),
            constraints: Vec::new(),
            substicks,
        }
    }

    pub fn add_body(&mut self, body: RigidBody) -> usize {
        let idx = self.bodies.len();
        self.bodies.push(body);
        idx
    }

    pub fn add_constraint<C: XpbdConstraint + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
    }

    /// Executes one global time step (dt) by performing N substicks.
    pub fn step(&mut self, global_dt: f64) {
        let substick_dt = global_dt / (self.substicks as f64);
        
        for _ in 0..self.substicks {
            self.substick(substick_dt);
        }
    }

    fn substick(&mut self, dt: f64) {
        // 1. Prediction Pass (Semi-Implicit Euler)
        // For now, we assume zero external forces (gravity follows in later passes)
        for body in &mut self.bodies {
            let vel = *body.velocity();
            let pos = *body.position();
            body.set_position(pos + vel * dt);
        }

        // 2. Constraint Resolution Pass (XPBD)
        // We use a single iteration for the baseline foundation.
        for constraint in &self.constraints {
            // Note: Accumulated lambda starts at 0.0 per global step in full XPBD,
            // but for simplicity in Tier 1, we pass 0.0 per substick.
            constraint.solve(&mut self.bodies, dt, 0.0);
        }

        // 3. Velocity Update (Velocity = (x_new - x_prev) / dt)
        // This will be refined as we track previous positions.
    }

    pub fn bodies(&self) -> &[RigidBody] {
        &self.bodies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::biomechanics::rigid_body::Vector3;
    use crate::domain::biomechanics::constraints::DistanceConstraint;

    #[test]
    fn test_world_step_with_constraint() {
        let mut world = World::new(10); // 10 substicks
        
        let b1_idx = world.add_body(RigidBody::new(Vector3 { x: 0.0, y: 0.0, z: 0.0 }, 1.0));
        let b2_idx = world.add_body(RigidBody::new(Vector3 { x: 2.0, y: 0.0, z: 0.0 }, 1.0));
        
        // Add a rigid distance constraint (dist = 1.0)
        world.add_constraint(DistanceConstraint::new(b1_idx, b2_idx, 1.0, 0.0));
        
        // Execute one step
        world.step(0.1);
        
        // The distance between bodies should be corrected toward 1.0
        let p1 = *world.bodies()[b1_idx].position();
        let p2 = *world.bodies()[b2_idx].position();
        let dist = (p1 - p2).length();
        
        assert!((dist - 1.0).abs() < 1e-2);
    }
}

use crate::domain::biomechanics::rigid_body::{RigidBody, Vector3};
use crate::domain::biomechanics::constraints::XpbdConstraint;

/// The World aggregate root. 
/// Orchestrates the temporal evolution of all anatomical and neural entities.
pub struct World {
    bodies: Vec<RigidBody>,
    constraints: Vec<Box<dyn XpbdConstraint>>,
    gravity: Vector3,
    substicks: usize,
    damping: f64,
}

impl World {
    pub fn new(substicks: usize) -> Self {
        Self {
            bodies: Vec::new(),
            constraints: Vec::new(),
            gravity: Vector3 { x: 0.0, y: -9.81, z: 0.0 }, // Standard Earth Gravity
            substicks,
            damping: 0.01, // Default 1% velocity damping
        }
    }

    pub fn set_gravity(&mut self, g: Vector3) {
        self.gravity = g;
    }

    pub fn add_body(&mut self, body: RigidBody) -> usize {
        let idx = self.bodies.len();
        self.bodies.push(body);
        idx
    }

    pub fn add_constraint<C: XpbdConstraint + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
    }

    /// Executes one global time step (global_dt) by performing N substicks.
    pub fn step(&mut self, global_dt: f64) {
        let substick_dt = global_dt / (self.substicks as f64);
        
        for _ in 0..self.substicks {
            self.substick(substick_dt);
        }
    }

    fn substick(&mut self, dt: f64) {
        // 1. Prediction Pass (Semi-Implicit Euler)
        for body in &mut self.bodies {
            if body.inverse_mass() > 0.0 {
                // Store previous position for velocity update
                body.set_prev_position(*body.position());

                let vel = *body.velocity();
                let pos = *body.position();
                let force = *body.external_force();
                let gravity_force = self.gravity * (1.0 / body.inverse_mass());
                
                // x* = x + dt*v + dt^2 * w * f
                let accel = (force + gravity_force) * body.inverse_mass();
                body.set_position(pos + vel * dt + accel * (dt * dt));
            }
        }

        // 2. Constraint Resolution Pass (XPBD)
        // For foundation, 1 iteration per substick (G-S)
        for constraint in &self.constraints {
            constraint.solve(&mut self.bodies, dt, 0.0);
        }

        // 3. Velocity Update and Damping
        for body in &mut self.bodies {
            if body.inverse_mass() > 0.0 {
                let p_new = *body.position();
                let p_prev = *body.prev_position();
                
                // v = (x_new - x_prev) / dt
                let mut new_vel = (p_new - p_prev) * (1.0 / dt);
                
                // Apply biological/environmental damping
                new_vel = new_vel * (1.0 - self.damping);
                
                body.set_velocity(new_vel);
                body.reset_external_force();
            }
        }
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
    fn test_world_step_with_gravity_and_damping() {
        let mut world = World::new(10);
        
        // Body starts at y=10.0
        let b1_idx = world.add_body(RigidBody::new(Vector3 { x: 0.0, y: 10.0, z: 0.0 }, 1.0));
        
        // Execute steps over 0.5 seconds
        for _ in 0..30 {
            world.step(0.016);
        }
        
        // Body should have fallen due to gravity (y < 10.0)
        assert!(world.bodies()[b1_idx].position().y < 10.0);
        // Velocity should be negative (downwards)
        assert!(world.bodies()[b1_idx].velocity().y < 0.0);
    }

    #[test]
    fn test_world_rigid_link_falls_together() {
        let mut world = World::new(20);
        
        let b1_idx = world.add_body(RigidBody::new(Vector3 { x: 0.0, y: 10.0, z: 0.0 }, 1.0));
        let b2_idx = world.add_body(RigidBody::new(Vector3 { x: 0.0, y: 11.0, z: 0.0 }, 1.0));
        
        // Rigid link of 1.0m
        world.add_constraint(DistanceConstraint::new(b1_idx, b2_idx, 1.0, 0.0));
        
        for _ in 0..10 {
            world.step(0.01);
        }
        
        let dist = (*world.bodies()[b1_idx].position() - *world.bodies()[b2_idx].position()).length();
        assert!((dist - 1.0).abs() < 1e-3);
    }
}

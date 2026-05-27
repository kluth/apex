#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// A Rigid Body designed for Extended Position Based Dynamics (XPBD).
#[derive(Debug, Clone)]
pub struct RigidBody {
    position: Vector3,
    velocity: Vector3,
    inverse_mass: f64,
}

impl RigidBody {
    /// Creates a new RigidBody. 
    /// If mass is 0.0, it is treated as an infinite mass (static body) where inverse_mass = 0.0.
    pub fn new(position: Vector3, mass: f64) -> Self {
        let inverse_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        
        Self {
            position,
            velocity: Vector3::default(),
            inverse_mass,
        }
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    pub fn velocity(&self) -> &Vector3 {
        &self.velocity
    }

    pub fn inverse_mass(&self) -> f64 {
        self.inverse_mass
    }
}

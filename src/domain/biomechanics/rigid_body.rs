use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Vector3 {
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
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

    pub fn set_position(&mut self, pos: Vector3) {
        self.position = pos;
    }

    pub fn velocity(&self) -> &Vector3 {
        &self.velocity
    }

    pub fn inverse_mass(&self) -> f64 {
        self.inverse_mass
    }

    pub fn apply_correction(&mut self, correction: Vector3) {
        if self.inverse_mass > 0.0 {
            self.position += correction;
        }
    }
}

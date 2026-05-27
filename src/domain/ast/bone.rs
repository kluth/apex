#[derive(Debug, Clone, PartialEq)]
pub struct MassError;

/// Value Object representing Mass in kilograms.
/// Mathematically guarantees mass > 0.0.
#[derive(Debug, Clone, PartialEq)]
pub struct Mass(f64);

impl Mass {
    pub fn new(value: f64) -> Result<Self, MassError> {
        if value <= 0.0 || value.is_nan() {
            Err(MassError)
        } else {
            Ok(Mass(value))
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

use crate::domain::biomechanics::rigid_body::Vector3;

/// Aggregate Root representing a Rigid Body.
#[derive(Debug, Clone, PartialEq)]
pub struct Bone {
    id: String,
    mass: Mass,
    position: Vector3,
}

impl Bone {
    pub fn new(id: String, mass: Mass, position: Vector3) -> Self {
        Self { id, mass, position }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn mass(&self) -> &Mass {
        &self.mass
    }

    pub fn position(&self) -> Vector3 {
        self.position
    }
}

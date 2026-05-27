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

/// Aggregate Root representing a Rigid Body.
#[derive(Debug, Clone, PartialEq)]
pub struct Bone {
    id: String,
    mass: Mass,
}

impl Bone {
    pub fn new(id: String, mass: Mass) -> Self {
        Self { id, mass }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn mass(&self) -> &Mass {
        &self.mass
    }
}

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

#[derive(Debug, PartialEq)]
pub enum AssetError {
    InvalidFormat,
}

/// Value Object representing a type-safe path to a biological 3D asset.
#[derive(Debug, Clone, PartialEq)]
pub struct AssetPath(String);

impl AssetPath {
    pub fn new(path: &str) -> Result<Self, AssetError> {
        if path.trim().is_empty() {
            Err(AssetError::InvalidFormat)
        } else {
            Ok(Self(path.to_string()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Value Object representing the visual mesh binding for an anatomical entity.
#[derive(Debug, Clone, PartialEq)]
pub struct MeshReference {
    path: AssetPath,
}

impl MeshReference {
    pub fn new(path: AssetPath) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &AssetPath {
        &self.path
    }
}

use crate::domain::biomechanics::rigid_body::Vector3;

/// Aggregate Root representing a Rigid Body.
#[derive(Debug, Clone, PartialEq)]
pub struct Bone {
    id: String,
    mass: Mass,
    position: Vector3,
    mesh_reference: Option<MeshReference>,
}

impl Bone {
    pub fn new(id: String, mass: Mass, position: Vector3) -> Self {
        Self {
            id,
            mass,
            position,
            mesh_reference: None,
        }
    }

    pub fn with_mesh(mut self, mesh_ref: MeshReference) -> Self {
        self.mesh_reference = Some(mesh_ref);
        self
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

    pub fn mesh_reference(&self) -> Option<&MeshReference> {
        self.mesh_reference.as_ref()
    }
}

#[cfg(test)]
mod mesh_binding_tests {
    use super::*;

    #[test]
    fn test_asset_path_validation() {
        let valid_path = AssetPath::new("anatomy/human/femur_l").expect("Should be valid");
        assert_eq!(valid_path.as_str(), "anatomy/human/femur_l");

        let result = AssetPath::new("   ");
        assert!(matches!(result, Err(AssetError::InvalidFormat)));
    }

    #[test]
    fn test_mesh_reference_instantiation() {
        let path = AssetPath::new("anatomy/human/tibia_l").unwrap();
        let mesh_ref = MeshReference::new(path);
        assert_eq!(mesh_ref.path().as_str(), "anatomy/human/tibia_l");
    }

    #[test]
    fn test_bone_mesh_binding() {
        let mass = Mass::new(1.0).unwrap();
        let pos = Vector3::default();
        let path = AssetPath::new("anatomy/human/femur_l").unwrap();
        let mesh_ref = MeshReference::new(path);

        let bone = Bone::new("FemurL".to_string(), mass, pos).with_mesh(mesh_ref.clone());

        assert_eq!(bone.mesh_reference(), Some(&mesh_ref));
    }
}

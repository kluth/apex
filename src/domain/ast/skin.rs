use crate::domain::ast::bone::Bone;

/// Represents a geometric primitive used for environmental interaction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CollisionPrimitive {
    Sphere { radius: f64 },
    Box { width: f64, height: f64, depth: f64 },
}

/// Value Object representing a collision anchor on a bone.
#[derive(Debug, Clone, PartialEq)]
pub struct CollisionHull {
    pub id: String,
    pub primitive: CollisionPrimitive,
    pub local_offset: (f64, f64, f64),
}

/// Aggregate Root representing the Integumentary System (Skin/Fascia).
#[derive(Debug, Clone, PartialEq)]
pub struct Skin {
    id: String,
    target_bone_id: String,
    hulls: Vec<CollisionHull>,
}

impl Skin {
    pub fn new(id: String, bone: &Bone) -> Self {
        Self {
            id,
            target_bone_id: bone.id().to_string(),
            hulls: Vec::new(),
        }
    }

    pub fn add_hull(&mut self, hull: CollisionHull) {
        self.hulls.push(hull);
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn target_bone_id(&self) -> &str {
        &self.target_bone_id
    }

    pub fn hulls(&self) -> &[CollisionHull] {
        &self.hulls
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ast::bone::{Bone, Mass};
    use crate::domain::biomechanics::rigid_body::Vector3;

    #[test]
    fn test_skin_attachment() {
        let mass = Mass::new(1.0).unwrap();
        let femur = Bone::new("Femur".to_string(), mass, Vector3::default());

        let mut skin = Skin::new("Skin_Femur".to_string(), &femur);
        let hull = CollisionHull {
            id: "Hull_1".to_string(),
            primitive: CollisionPrimitive::Sphere { radius: 0.1 },
            local_offset: (0.0, 0.0, 0.0),
        };

        skin.add_hull(hull);

        assert_eq!(skin.id(), "Skin_Femur");
        assert_eq!(skin.target_bone_id(), "Femur");
        assert_eq!(skin.hulls().len(), 1);
    }
}

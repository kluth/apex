use crate::domain::ast::bone::Bone;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
    DuplicateIdentifier(String),
}

/// Domain Service (orchestrated by Application) to validate biological constraints.
pub struct BiologicalValidator;

impl BiologicalValidator {
    /// Validates that all biological structures in the AST are plausible.
    /// Returns Ok(()) or a ValidationError.
    pub fn validate_bones(bones: &[Bone]) -> Result<(), ValidationError> {
        let mut ids = HashSet::new();

        for bone in bones {
            if !ids.insert(bone.id().to_string()) {
                return Err(ValidationError::DuplicateIdentifier(bone.id().to_string()));
            }
        }

        Ok(())
    }
}

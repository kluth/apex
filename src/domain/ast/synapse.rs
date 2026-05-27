use crate::domain::movement::cpg::Cpg;
use crate::domain::ast::muscle::Muscle;

/// Represents a neural connection between a CPG and a Muscle.
#[derive(Debug, Clone, PartialEq)]
pub struct Synapse {
    id: String,
    source_cpg_id: String, // Simplified mapping via string IDs in AST
    target_muscle_id: String,
    weight: f64, // Synaptic gain
}

impl Synapse {
    pub fn new(id: String, _source: &Cpg, target: &Muscle, weight: f64) -> Self {
        // Cpg doesn't have an explicit ID yet in the struct, using a placeholder for now
        // In a real pass, we'd map this via a registry.
        Self {
            id,
            source_cpg_id: "CPG_0".to_string(), // Placeholder
            target_muscle_id: target.id().to_string(),
            weight,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn target_muscle_id(&self) -> &str {
        &self.target_muscle_id
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ast::bone::{Bone, Mass};
    use crate::domain::movement::cpg::Cpg;

    #[test]
    fn test_synapse_instantiation() {
        let mass = Mass::new(1.0).unwrap();
        let femur = Bone::new("Femur".to_string(), mass.clone());
        let tibia = Bone::new("Tibia".to_string(), mass);
        let muscle = Muscle::new(
            "Biceps".to_string(),
            &femur,
            &tibia,
            (0.0, 0.0, 0.0),
            (0.0, 0.5, 0.0),
            500.0,
        );
        let cpg = Cpg::new(1.0);
        
        let synapse = Synapse::new("Syn_1".to_string(), &cpg, &muscle, 0.8);
        
        assert_eq!(synapse.id(), "Syn_1");
        assert_eq!(synapse.target_muscle_id(), "Biceps");
        assert_eq!(synapse.weight(), 0.8);
    }
}

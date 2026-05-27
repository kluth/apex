use crate::domain::biomechanics::rigid_body::Vector3;

/// Trait defining the contract for evolutionary fitness evaluation.
pub trait FitnessEvaluator {
    /// Evaluates the fitness of an organism state.
    fn evaluate(&self, current_state: Vector3) -> f64;
}

/// A concrete fitness evaluator that measures distance from a starting point.
pub struct DistanceFitness {
    start_position: Vector3,
}

impl DistanceFitness {
    pub fn new(start_position: Vector3) -> Self {
        Self { start_position }
    }
}

impl FitnessEvaluator for DistanceFitness {
    fn evaluate(&self, current_state: Vector3) -> f64 {
        let dx = current_state.x - self.start_position.x;
        let dy = current_state.y - self.start_position.y;
        let dz = current_state.z - self.start_position.z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

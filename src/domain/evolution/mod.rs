pub mod fitness;

#[cfg(test)]
mod tests {
    use super::fitness::{FitnessEvaluator, DistanceFitness};
    use crate::domain::biomechanics::rigid_body::Vector3;

    #[test]
    fn test_distance_fitness_evaluation() {
        // Red Phase: Distance fitness should calculate the Euclidean distance 
        // between start and end positions.
        let start = Vector3 { x: 0.0, y: 0.0, z: 0.0 };
        let end = Vector3 { x: 3.0, y: 4.0, z: 0.0 }; // 3-4-5 triangle
        
        let evaluator = DistanceFitness::new(start);
        let score = evaluator.evaluate(end);
        
        assert_eq!(score, 5.0);
    }
}

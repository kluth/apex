pub mod domain;
pub mod application;
pub mod infrastructure;

/// The APEX Standard Library (Prelude).
/// Re-exports the most commonly used types for ease of use.
pub mod prelude {
    pub use crate::domain::ast::bone::{Bone, Mass};
    pub use crate::domain::air::topology::{Topology, NodeId, EdgeId};
    pub use crate::domain::biomechanics::rigid_body::{RigidBody, Vector3};
    pub use crate::domain::movement::cpg::Cpg;
    pub use crate::domain::evolution::fitness::{FitnessEvaluator, DistanceFitness};
    pub use crate::application::compiler::pipeline::CompilerPipeline;
    pub use crate::infrastructure::telemetry::{init_telemetry, shutdown_telemetry};
}

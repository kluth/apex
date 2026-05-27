pub mod application;
pub mod domain;
pub mod infrastructure;

/// The APEX Standard Library (Prelude).
/// Re-exports the most commonly used types for ease of use.
pub mod prelude {
    pub use crate::application::compiler::pipeline::CompilerPipeline;
    pub use crate::domain::air::topology::{EdgeId, NodeId, Topology};
    pub use crate::domain::ast::bone::{Bone, Mass};
    pub use crate::domain::ast::joint::Joint;
    pub use crate::domain::ast::muscle::Muscle;
    pub use crate::domain::ast::skin::{CollisionHull, CollisionPrimitive, Skin};
    pub use crate::domain::ast::synapse::Synapse;
    pub use crate::domain::biomechanics::muscle_model::HillCurve;
    pub use crate::domain::biomechanics::registry::BodyRegistry;
    pub use crate::domain::biomechanics::rigid_body::{RigidBody, Vector3};
    pub use crate::domain::biomechanics::soft_body::VolumeConstraint;
    pub use crate::domain::biomechanics::world::World;
    pub use crate::domain::evolution::fitness::{DistanceFitness, FitnessEvaluator};
    pub use crate::domain::movement::cpg::Cpg;
    pub use crate::domain::movement::proprioception::Proprioception;
    pub use crate::infrastructure::exporter::gltf::GltfExporter;
    pub use crate::infrastructure::telemetry::{init_telemetry, shutdown_telemetry};
}

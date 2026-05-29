use crate::domain::ast::bone::MeshReference;
use crate::domain::biomechanics::rigid_body::Vector3;

/// Represents an index within the Topology Arena for a Node (Bone).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

impl NodeId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

/// Represents an index within the Topology Arena for an Edge (Joint or Muscle).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(usize);

impl EdgeId {
    pub fn new(index: usize) -> Self {
        Self(index)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

/// Represents the role of a topological connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    /// A structural link (Joint) that defines the anatomical hierarchy.
    Structural,
    /// An active link (Muscle) that provides force between nodes.
    Actuator,
    /// A neural link (Synapse) that carries control signals.
    Neural,
    /// A sensory link (Feedback) that carries data back to controllers.
    Sensory,
    /// An integumentary link (Skin) that defines volumetric envelopes.
    Integument,
    /// A visceral link (Organ) that defines internal volumetric bodies.
    Visceral,
}

/// Represents a connection between two Nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    source: NodeId,
    target: NodeId,
    name: String,
    edge_type: EdgeType,
}

impl Edge {
    pub fn source(&self) -> NodeId {
        self.source
    }

    pub fn target(&self) -> NodeId {
        self.target
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn edge_type(&self) -> EdgeType {
        self.edge_type
    }
}

pub struct Node {
    pub name: String,
    pub position: Vector3,
    pub mesh_reference: Option<MeshReference>,
}

/// The Anatomy Intermediate Representation topological graph.
/// Uses an Arena-based memory layout for DOD compliance.
#[derive(Default)]
pub struct Topology {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Topology {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(
        &mut self,
        name: String,
        position: Vector3,
        mesh_reference: Option<MeshReference>,
    ) -> NodeId {
        let idx = self.nodes.len();
        self.nodes.push(Node {
            name,
            position,
            mesh_reference,
        });
        NodeId(idx)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn node_name(&self, id: NodeId) -> Option<&str> {
        self.nodes.get(id.index()).map(|n| n.name.as_str())
    }

    pub fn node_position(&self, id: NodeId) -> Option<Vector3> {
        self.nodes.get(id.index()).map(|n| n.position)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id.index())
    }

    pub fn node_mesh_reference(&self, id: NodeId) -> Option<&MeshReference> {
        self.nodes
            .get(id.index())
            .and_then(|n| n.mesh_reference.as_ref())
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn add_edge(
        &mut self,
        source: NodeId,
        target: NodeId,
        name: String,
        edge_type: EdgeType,
    ) -> EdgeId {
        let idx = self.edges.len();
        let edge = Edge {
            source,
            target,
            name,
            edge_type,
        };
        self.edges.push(edge);
        EdgeId(idx)
    }

    pub fn get_edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(id.index())
    }
}

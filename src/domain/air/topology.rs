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

/// Represents an index within the Topology Arena for an Edge (Joint).
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

/// Represents a structural connection between two Nodes.
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    source: NodeId,
    target: NodeId,
    name: String,
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

    pub fn node_mesh_reference(&self, id: NodeId) -> Option<&MeshReference> {
        self.nodes
            .get(id.index())
            .and_then(|n| n.mesh_reference.as_ref())
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn add_edge(&mut self, source: NodeId, target: NodeId, name: String) -> EdgeId {
        let idx = self.edges.len();
        let edge = Edge {
            source,
            target,
            name,
        };
        self.edges.push(edge);
        EdgeId(idx)
    }

    pub fn get_edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(id.index())
    }
}

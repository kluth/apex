pub mod topology;

#[cfg(test)]
mod tests {
    use super::topology::{Topology, EdgeId};

    #[test]
    fn test_air_topology_insertion_and_linkage() {
        // Red Phase: We expect to insert nodes and link them via edges.
        let mut topology = Topology::new();
        
        let femur_id = topology.add_node("Femur".to_string());
        let tibia_id = topology.add_node("Tibia".to_string());
        
        assert_eq!(femur_id.index(), 0);
        assert_eq!(tibia_id.index(), 1);

        // Link them with a knee joint.
        let joint_id = topology.add_edge(femur_id, tibia_id, "KneeJoint".to_string());
        
        // Assert edge exists
        let edge = topology.get_edge(joint_id).expect("Edge should exist");
        assert_eq!(edge.source(), femur_id);
        assert_eq!(edge.target(), tibia_id);
        
        // Ensure illegal edge queries fail gracefully using Result/Option
        let invalid_edge = topology.get_edge(EdgeId::new(999));
        assert!(invalid_edge.is_none());
    }
}

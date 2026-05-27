use apex::prelude::*;

#[test]
fn test_compile_proto_crawler() {
    let source = include_str!("../examples/proto_crawler.apex");
    let pipeline = CompilerPipeline::new();
    
    let topology = pipeline.compile(source).expect("Failed to compile proto_crawler.apex");
    
    // 2 bones = 2 nodes
    assert_eq!(topology.node_count(), 2);
    
    // We can also verify that we can initialize a world from this
    let mut world = World::new(10);
    
    // Setup would normally be automated via a 'Loader' service, 
    // but we can manually verify the body registry size.
    world.add_body(0.0, 0.0, 0.0, 1.2);
    world.add_body(1.0, 0.0, 0.0, 0.8);
    
    assert_eq!(world.registry().len(), 2);
}

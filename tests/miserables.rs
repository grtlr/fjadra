use fjadra::{Center, Link, ManyBody, Node, SimulationBuilder};
use fjadra_data::miserables;

#[test]
fn test_miserables() {
    let graph = miserables::Graph::load().unwrap();

    let mut simulation = SimulationBuilder::default()
        .build(graph.nodes.iter().map(|_| Node::default()))
        .add_force(
            "link",
            Link::new(graph.links.iter().map(|link| (link.source, link.target))),
        )
        .add_force("charge", ManyBody::new())
        .add_force("center", Center::new());

    let positions = simulation.iter().last();

    insta::assert_debug_snapshot!(positions);
}

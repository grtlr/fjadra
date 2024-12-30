use fjadra::{Center, Link, ManyBody, Node, SimulationBuilder};
use fjadra_data::random_tree;

#[test]
fn test_random_tree() {
    let (nodes, edges) = random_tree::generate(300);

    let mut simulation = SimulationBuilder::default()
        .build(nodes.iter().map(|_| Node::default()))
        .add_force(
            "link",
            Link::new(edges.iter().map(|(source, target)| (*source, *target)))
                .strength(1.0)
                .distance(60.0)
                .iterations(10),
        )
        .add_force("charge", ManyBody::new())
        .add_force("center", Center::new());

    let positions = simulation.iter().last().unwrap();

    insta::assert_json_snapshot!(positions, {
        "[][]" => insta::rounded_redaction(5)
    });
}

use std::env;

use fjadra::{Link, ManyBody, Node, SimulationBuilder};
use fjadra_data::random_tree;

const DEFAULT_NUM_NODES: usize = 300;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let num_nodes = args
        .get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(DEFAULT_NUM_NODES);

    let rec =
        rerun::RecordingStreamBuilder::new(format!("fjadra_random_tree_{num_nodes}")).spawn()?;

    let (nodes, edges) = random_tree::generate(num_nodes);

    let mut simulation = SimulationBuilder::default()
        .build(nodes.iter().map(|_| Node::default()))
        .add_force(
            "link",
            Link::new(edges.clone().into_iter())
                .strength(1.0)
                .distance(60.0)
                .iterations(10),
        )
        .add_force("charge", ManyBody::new());

    let positions = simulation
        .iter()
        .last()
        .expect("simulation should always return");

    rec.log_static(
        "tree",
        &[
            &rerun::GraphNodes::new(nodes)
                .with_positions(positions.into_iter().map(|[x, y]| [x as f32, y as f32]))
                as &dyn rerun::AsComponents,
            &rerun::GraphEdges::new(
                edges
                    .iter()
                    .map(|(source, target)| (source.to_string(), target.to_string())),
            ),
        ],
    )?;

    Ok(())
}

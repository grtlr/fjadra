use fjadra::{Center, Link, ManyBody, Node, SimulationBuilder};
use fjadra_data::miserables;
use rerun as rr;

mod scale_chromatic;
use scale_chromatic::{Color, ScaleOrdinal, SchemeCategory10};

fn main() -> anyhow::Result<()> {
    let graph = miserables::Graph::load()?;

    let mut simulation = SimulationBuilder::default()
        .build(graph.nodes.iter().map(|_| Node::default()))
        .add_force(
            "link",
            Link::new(graph.links.iter().map(|link| (link.source, link.target))),
        )
        .add_force("charge", ManyBody::new())
        .add_force("center", Center::new());

    let rec = rr::RecordingStreamBuilder::new("fjadra_miserables").spawn()?;

    let colors = ScaleOrdinal::from(SchemeCategory10)
        .iter()
        .cycle()
        .take(graph.nodes.len())
        .map(|Color { r, g, b }| rr::Color::from_rgb(r, g, b))
        .collect::<Vec<_>>();

    let positions = simulation.iter().last().unwrap();

    rec.log_static(
        "/miserables",
        &rr::GraphNodes::new(graph.nodes.iter().map(|n| n.name.clone()))
            .with_positions(positions.into_iter().map(|[x, y]| [x as f32, y as f32]))
            .with_colors(graph.nodes.iter().map(|n| colors[n.group])),
    )?;

    rec.log_static(
        "/miserables",
        &rr::GraphEdges::new(
            graph
                .links
                .iter()
                .map(|miserables::Edge { source, target }| {
                    (
                        graph.nodes[*source].name.clone(),
                        graph.nodes[*target].name.clone(),
                    )
                }),
        ),
    )?;

    Ok(())
}

use fjadra::{Center, Link, ManyBody, SimulationBuilder};
use rerun as rr;

mod scale_chromatic;
use scale_chromatic::{Color, ScaleOrdinal, SchemeCategory10};

#[derive(Debug, Clone, serde::Deserialize)]
struct Graph {
    pub nodes: Vec<Node>,
    pub links: Vec<Edge>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Node {
    pub name: String,
    pub group: usize,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Edge {
    pub source: usize,
    pub target: usize,
}

fn main() -> anyhow::Result<()> {
    let file = std::fs::File::open("examples/data/miserables.json")?;
    let graph: Graph = serde_json::from_reader(file)?;

    let mut simulation = SimulationBuilder::default()
        .build(graph.nodes.iter().map(|_| Option::<[f64; 2]>::None))
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

    for positions in simulation.iter() {
        rec.log(
            "/miserables",
            &rr::GraphNodes::new(graph.nodes.iter().map(|n| n.name.clone()))
                .with_positions(positions.into_iter().map(|[x, y]| [x as f32, y as f32]))
                .with_colors(graph.nodes.iter().map(|n| colors[n.group])),
        )?;
    }

    rec.log(
        "/miserables",
        &rr::GraphEdges::new(graph.links.iter().map(|Edge { source, target }| {
            (
                graph.nodes[*source].name.clone(),
                graph.nodes[*target].name.clone(),
            )
        })),
    )?;

    Ok(())
}

use itertools::Itertools;

use fjadra::force::{Link, ManyBody, SimulationBuilder};
use rerun::{components::GraphType, Color, GraphEdges, GraphNodes};

const NUM_NODES: usize = 10;

fn main() -> anyhow::Result<()> {
    let rec = rerun::RecordingStreamBuilder::new("fjadra_lattice").spawn()?;

    let coordinates = (0..NUM_NODES).cartesian_product(0..NUM_NODES);

    let (nodes, colors): (Vec<_>, Vec<_>) = coordinates
        .clone()
        .enumerate()
        .map(|(i, (x, y))| {
            let r = ((x as f32 / (NUM_NODES - 1) as f32) * 255.0).round() as u8;
            let g = ((y as f32 / (NUM_NODES - 1) as f32) * 255.0).round() as u8;
            (i, Color::from_rgb(r, g, 0))
        })
        .unzip();

    let mut edges = Vec::new();
    for (x, y) in coordinates.clone() {
        if y > 0 {
            let source = (y - 1) * NUM_NODES + x;
            let target = y * NUM_NODES + x;
            edges.push((source, target));
        }
        if x > 0 {
            let source = y * NUM_NODES + (x - 1);
            let target = y * NUM_NODES + x;
            edges.push((source, target));
        }
    }

    let mut simulation = SimulationBuilder::default()
        .build(nodes.iter().map(|_| Option::<[f64; 2]>::None))
        .add_force_link(
            "link",
            Link::new(edges.clone().into_iter())
                .strength(1.0)
                .distance(60.0)
                .iterations(10),
        )
        .add_force_many_body("charge", ManyBody::new().strength(-70.0));

    while !simulation.finished() {
        simulation.tick(3);

        rec.log(
            "/lattice",
            &GraphNodes::new(nodes.iter().map(|key| key.to_string()))
                .with_positions(simulation.positions().map(|[x, y]| [x as f32, y as f32]))
                .with_colors(colors.clone())
                .with_labels(coordinates.clone().map(|(x, y)| format!("({}, {})", x, y))),
        )?;
    }

    // We log one final time after the layout is finished
    rec.log(
        "/lattice",
        &GraphNodes::new(nodes.iter().map(|key| key.to_string()))
            .with_positions(simulation.positions().map(|[x, y]| [x as f32, y as f32]))
            .with_colors(colors)
            .with_labels(coordinates.clone().map(|(x, y)| format!("({}, {})", x, y))),
    )?;

    rec.log(
        "/lattice",
        &GraphEdges::new(
            edges
                .iter()
                .map(|(source, target)| (source.to_string(), target.to_string())),
        )
        .with_graph_type(GraphType::Directed),
    )?;

    Ok(())
}

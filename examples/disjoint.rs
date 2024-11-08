use fjadra::{
    force::{Collide, SimulationBuilder},
    PositionX, PositionY,
};
use rerun::GraphNodes;

const NUM_NODES: usize = 10;

fn main() -> anyhow::Result<()> {
    let rec = rerun::RecordingStreamBuilder::new("fjadra_disjoint").spawn()?;

    let nodes = (0..NUM_NODES)
        .map(|i| (format!("node{i}"), None))
        .collect::<Vec<(_, Option<[f64; 2]>)>>();

    let mut simulation = SimulationBuilder::default()
        // .with_alpha_target(0.3)
        .with_velocity_decay(0.1)
        .build(nodes.iter().cloned().map(|(_, p)| p))
        .add_force("collide", Collide::new().radius(|_| 20.0).iterations(3))
        .add_force("x", PositionX::new())
        .add_force("y", PositionY::new());

    for positions in simulation.iter() {
        rec.log(
            "/nodes",
            &GraphNodes::new(nodes.iter().map(|(key, _)| key.clone()))
                .with_positions(positions.into_iter().map(|[x, y]| [x as f32, y as f32])),
        )?;
    }

    Ok(())
}

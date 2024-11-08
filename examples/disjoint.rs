use fjadra::force::{Collide, SimulationBuilder};
use rerun::GraphNodes;

const NUM_NODES: usize = 10;

fn main() -> anyhow::Result<()> {
    let rec = rerun::RecordingStreamBuilder::new("fjadra_disjoint").spawn()?;

    let nodes = (0..NUM_NODES)
        .map(|i| (format!("node{}", i), None))
        .collect::<Vec<(_, Option<[f64; 2]>)>>();

    let mut simulation = SimulationBuilder::default()
        // .with_alpha_target(0.3)
        .with_velocity_decay(0.1)
        .build(nodes.iter().cloned().map(|(_, p)| p))
        .add_force_collide("collide", Collide::new().radius(|_| 20.0).iterations(3))
        .add_force_x("x", Default::default())
        .add_force_y("y", Default::default());

    while !simulation.finished() {
        simulation.tick(3);

        let nodes = nodes.iter().map(|(key, _)| key.clone()).collect::<Vec<_>>();
        rec.log(
            "/nodes",
            &GraphNodes::new(nodes)
                .with_positions(simulation.positions().map(|[x, y]| [x as f32, y as f32])),
        )?;
    }

    // We log one final time after the layout is finished.
    let nodes = nodes.iter().map(|(key, _)| key.clone()).collect::<Vec<_>>();
    rec.log(
        "/nodes",
        &GraphNodes::new(nodes)
            .with_positions(simulation.positions().map(|[x, y]| [x as f32, y as f32])),
    )?;

    Ok(())
}

use rand_distr::{Distribution, Weibull};

use fjadra::{Collide, ManyBody, PositionX, PositionY, SimulationBuilder};
use rerun as rr;

mod scale_chromatic;
use scale_chromatic::{Color, ScaleOrdinal, SchemeCategory10};

const NUM_NODES: usize = 10;
const SHAPE: f32 = 1.5;
const SCALE: f32 = 20.0;

fn main() -> anyhow::Result<()> {
    let rec = rr::RecordingStreamBuilder::new("fjadra_collision").spawn()?;

    let colors = ScaleOrdinal::from(SchemeCategory10)
        .iter()
        .cycle()
        .take(NUM_NODES)
        .map(|Color { r, g, b }| rr::Color::from_rgb(r, g, b))
        .collect::<Vec<_>>();

    // Create a Weibull distribution with the specified shape and scale
    let weibull = Weibull::<f32>::new(SCALE, SHAPE).expect("Failed to create Weibull distribution");

    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate `NUM_NODES` particles following the Weibull distribution
    let (ids, radii): (Vec<_>, Vec<_>) = (0..NUM_NODES)
        .map(|i| (i.to_string(), weibull.sample(&mut rng)))
        .unzip();

    let cloned = radii.clone();

    let mut simulation = SimulationBuilder::default()
        .with_velocity_decay(0.1)
        .build(ids.iter().map(|_| Option::<[f64; 2]>::None))
        .add_force("x", PositionX::new().strength(0.1))
        .add_force("y", PositionY::new().strength(0.1))
        .add_force(
            "collide",
            Collide::new().radius(move |i| cloned[i] as f64 + 1.0),
        )
        .add_force("charge", ManyBody::new().strength(0.));

    for positions in simulation.iter() {
        rec.log(
            "/collision",
            &rr::GraphNodes::new(ids.clone())
                .with_positions(positions.into_iter().map(|[x, y]| [x as f32, y as f32]))
                .with_radii(radii.clone())
                .with_colors(colors.clone()),
        )?;
    }

    Ok(())
}

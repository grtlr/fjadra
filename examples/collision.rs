use rand_distr::{Distribution, Weibull};

use fjadra::{Collide, ManyBody, PositionX, PositionY, SimulationBuilder};
use rerun::{Color, GraphNodes};

const NUM_NODES: usize = 10;
const SHAPE: f32 = 1.5;
const SCALE: f32 = 20.0;

fn main() -> anyhow::Result<()> {
    let rec = rerun::RecordingStreamBuilder::new("fjadra_collision").spawn()?;

    let brewer_colors = [
        (166, 206, 227),
        (31, 120, 180),
        (178, 223, 138),
        (51, 160, 44),
        (251, 154, 153),
        (227, 26, 28),
        (253, 191, 111),
        (255, 127, 0),
        (202, 178, 214),
        (106, 61, 154),
    ];

    let colors = brewer_colors
        .iter()
        .cycle()
        .take(NUM_NODES)
        .map(|&(r, g, b)| Color::from_rgb(r, g, b))
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
        .add_force_x("x".into(), PositionX::new().with_strength(0.1))
        .add_force_y("y".into(), PositionY::new().with_strength(0.1))
        .add_force_collide(
            "collide".into(),
            Collide::new().with_radius(move |i| cloned[i] as f64 + 1.0),
        )
        .add_force_many_body("charge".into(), ManyBody::new().with_strength(0.));

    while !simulation.finished() {
        simulation.tick(1);

        rec.log(
            "/collision",
            &GraphNodes::new(ids.clone())
                .with_positions(simulation.positions().map(|[x, y]| [x as f32, y as f32]))
                .with_radii(radii.clone())
                .with_colors(colors.clone()),
        )?;
    }

    // We log one final time after the layout is finished
    rec.log(
        "/collision",
        &GraphNodes::new(ids)
            .with_positions(simulation.positions().map(|[x, y]| [x as f32, y as f32]))
            .with_radii(radii)
            .with_colors(colors),
    )?;

    Ok(())
}

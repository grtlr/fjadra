use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fjadra::{Center, Link, ManyBody, Node, SimulationBuilder};
use fjadra_data::miserables;

fn bench_simulation(c: &mut Criterion) {
    let graph = miserables::Graph::load().unwrap();

    c.bench_function("miserables (init)", |b| {
        b.iter(|| {
            let simulation = SimulationBuilder::default()
                .build(graph.nodes.iter().map(|_| Node::default()))
                .add_force(
                    "link",
                    Link::new(graph.links.iter().map(|link| (link.source, link.target))),
                )
                .add_force("charge", ManyBody::new())
                .add_force("center", Center::new());

            black_box(simulation)
        });
    });

    c.bench_function("miserables (full)", |b| {
        b.iter(|| {
            let mut simulation = SimulationBuilder::default()
                .build(graph.nodes.iter().map(|_| Node::default()))
                .add_force(
                    "link",
                    Link::new(graph.links.iter().map(|link| (link.source, link.target))),
                )
                .add_force("charge", ManyBody::new())
                .add_force("center", Center::new());

            simulation.iter().last()
        });
    });
}

criterion_group!(benches, bench_simulation);
criterion_main!(benches);

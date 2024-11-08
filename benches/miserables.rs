use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fjadra::{Center, Link, ManyBody, SimulationBuilder};

#[derive(Debug, Clone, serde::Deserialize)]
struct Graph {
    pub nodes: Vec<Node>,
    pub links: Vec<Edge>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Node {
    #[allow(unused)]
    pub name: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct Edge {
    pub source: usize,
    pub target: usize,
}

fn bench_simulation(c: &mut Criterion) {
    let file = std::fs::File::open("examples/data/miserables.json").unwrap();
    let graph: Graph = serde_json::from_reader(file).unwrap();

    c.bench_function("miserables (init)", |b| {
        b.iter(|| {
            let simulation = SimulationBuilder::default()
                .build(graph.nodes.iter().map(|_| Option::<[f64; 2]>::None))
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
                .build(graph.nodes.iter().map(|_| Option::<[f64; 2]>::None))
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

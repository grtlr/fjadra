use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fjadra::{Center, Link, ManyBody, Node, SimulationBuilder};
use fjadra_data::random_tree;

fn random_tree(c: &mut Criterion) {
    let (nodes, edges) = random_tree::generate(10_000);

    let mut group = c.benchmark_group("random_tree");
    group.sample_size(10);

    for size in [50, 100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut simulation = SimulationBuilder::default()
                    .build(nodes[0..size].iter().map(|_| Node::default()))
                    .add_force("link", Link::new(edges[0..size - 1].iter().copied()))
                    .add_force("charge", ManyBody::new())
                    .add_force("center", Center::new());

                simulation.iter().last()
            });
        });
    }

    group.finish();
}

criterion_group!(benches, random_tree);
criterion_main!(benches);

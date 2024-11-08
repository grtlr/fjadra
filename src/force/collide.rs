use crate::{
    force::particle::NodeIndex,
    lcg::Lcg,
    quadtree::{Entry, Quad, Quadtree, Visit},
};

use super::{
    jiggle::jiggle,
    particle::Particle,
    simulation::{Force, ForceBuilder},
};

pub struct Collide {
    strength: f64,
    iterations: usize,
    radius_fn: Box<dyn Fn(usize) -> f64>,
}

impl Default for Collide {
    fn default() -> Self {
        Self {
            strength: 1.0,
            iterations: 1,
            radius_fn: Box::new(|_| 1.0),
        }
    }
}

impl Collide {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn radius<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> f64 + 'static,
    {
        self.radius_fn = Box::new(f);
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }
}

impl ForceBuilder for Collide {
    fn initialize(self, particles: &[Particle]) -> Force {
        Force::Collide(CollideForce {
            radii: particles
                .iter()
                .map(|n| (self.radius_fn)(n.index.into()))
                .collect(),
            strength: self.strength,
            iterations: self.iterations,
        })
    }
}

#[derive(Clone, Debug)]
pub struct CollideForce {
    radii: Vec<f64>,
    strength: f64,
    iterations: usize,
}

impl CollideForce {
    pub fn force(&self, random: &mut Lcg, particles: &mut [Particle]) {
        let iterations = self.iterations;

        let prepare = |mut quad: Quad<'_, f64, NodeIndex>| match quad.inner() {
            Entry::Leaf { data, .. } => {
                // We only look at the data from the first leaf.
                *quad.value_mut() = self.radii[usize::from(*data)];
            }
            Entry::Internal { children } => {
                let max_radius = children
                    .iter()
                    .filter_map(|c| c.map(|c| c))
                    .max_by(|a, b| a.partial_cmp(b).expect("radii should be comparable"));
                *quad.value_mut() = *max_radius.expect("the radius should be well-defined");
            }
        };

        let mut apply = |index: NodeIndex,
                         xi: f64,
                         yi: f64,
                         ri: f64,
                         particles: &mut [Particle],
                         quad: Quad<'_, f64, NodeIndex>|
         -> Visit {
            let [x0, y0, x1, y1] = quad.extent().into();
            let rj = quad.value();
            let r = ri + rj;
            match quad.inner() {
                // We only look at the first value in the leafs. Because we visit all particles, we will
                // resolve the others eventually as well.
                Entry::Leaf { data, .. } => {
                    if *data > index {
                        // Avoid the mutable borrow.
                        let (left, right) = particles.split_at_mut(usize::from(*data));
                        let node = &mut left[usize::from(index)];
                        let data = &mut right[0];

                        let mut x = xi - data.x - data.vx;
                        let mut y = yi - data.y - data.vy;
                        let mut l = x * x + y * y;
                        if l < r * r {
                            if x == 0.0 {
                                x = jiggle(random);
                                l += x * x;
                            }
                            if y == 0.0 {
                                y = jiggle(random);
                                l += y * y;
                            }
                            l = (r - l.sqrt()) / l.sqrt() * self.strength;
                            x *= l;
                            y *= l;
                            let rj2 = rj * rj;
                            let frac = rj2 / (ri * ri + rj2);
                            node.vx += x * frac;
                            node.vy += y * frac;
                            data.vx -= x * (1.0 - frac);
                            data.vy -= y * (1.0 - frac);
                        }
                    }
                }
                Entry::Internal { .. } => {
                    // We don't consider quads that are further away than the combined radii.
                    if x0 > xi + r || x1 < xi - r || y0 > yi + r || y1 < yi - r {
                        return Visit::Skip;
                    }
                }
            }
            Visit::Continue
        };

        for _ in 0..iterations {
            // TODO(grtlr): get rid of this!
            let tmp = particles
                .iter()
                .map(|node| (node.x, node.y, node.index))
                .collect::<Vec<_>>();
            let mut tree = Quadtree::<f64, NodeIndex>::from_particles(tmp.iter().copied());
            tree.visit_after(prepare);

            for (xi, yi, index) in tmp {
                let ri = self.radii[usize::from(index)];
                tree.visit(|quad| apply(index, xi, yi, ri, particles, quad));
            }
        }
    }
}

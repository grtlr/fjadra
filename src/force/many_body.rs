use crate::{
    lcg::Lcg,
    quadtree::{Entry, Quad, Quadtree, Visit},
};

use super::{
    jiggle::jiggle,
    particle::{Particle, ParticleIndex},
    simulation::{Force, ForceBuilder},
};

pub struct NodeFn(Box<dyn Fn(ParticleIndex, usize) -> f64>);

impl From<f64> for NodeFn {
    fn from(value: f64) -> Self {
        Self(Box::new(move |_, _| value))
    }
}

impl<F> From<F> for NodeFn
where
    F: Fn(ParticleIndex, usize) -> f64 + 'static,
{
    fn from(f: F) -> Self {
        Self(Box::new(f))
    }
}

pub struct ManyBody {
    strength: NodeFn,
    distance_min: f64,
    distance_max: f64,
    theta: f64,
}

impl Default for ManyBody {
    fn default() -> Self {
        Self {
            strength: NodeFn::from(-30.0),
            distance_min: 1.0,
            distance_max: f64::INFINITY,
            theta: 0.9,
        }
    }
}

impl ManyBody {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn strength(mut self, f: impl Into<NodeFn>) -> Self {
        self.strength = f.into();
        self
    }
}

impl ForceBuilder for ManyBody {
    fn initialize(self, particles: &[Particle]) -> Force {
        let strengths = particles
            .iter()
            .enumerate()
            .map(|(i, node)| (self.strength.0)(node.index, i))
            .collect();

        Force::ManyBody(ManyBodyForce {
            strengths,
            distance_min_2: self.distance_min * self.distance_min,
            distance_max_2: self.distance_max * self.distance_max,
            theta_2: self.theta * self.theta,
        })
    }
}

pub struct ManyBodyForce {
    strengths: Vec<f64>,
    distance_min_2: f64,
    distance_max_2: f64,
    theta_2: f64,
}

#[derive(Default)]
struct Charge {
    x: f64,
    y: f64,
    strength: f64,
}

impl ManyBodyForce {
    pub fn force(&self, alpha: f64, random: &mut Lcg, particles: &mut [Particle]) {
        let accumulate = |mut quad: Quad<'_, Charge, ParticleIndex>| match quad.inner() {
            Entry::Leaf { data, others, x, y } => {
                let strength = self.strengths[usize::from(*data)]
                    + others
                        .unwrap_or_default()
                        .iter()
                        .map(|&&d| self.strengths[usize::from(d)])
                        .sum::<f64>();

                *quad.value_mut() = Charge { x, y, strength };
            }
            Entry::Internal { children } => {
                let mut result = Charge::default();
                let mut weight = 0.0;

                for &c in &children {
                    if let Some(q) = c {
                        let c = q.strength.abs();
                        result.strength += q.strength;
                        weight += c;
                        result.x += c * q.x;
                        result.y += c * q.y;
                    }
                }
                result.x /= weight;
                result.y /= weight;
                *quad.value_mut() = result;
            }
        };

        let mut apply = |index: ParticleIndex,
                         particles: &mut [Particle],
                         quad: Quad<'_, Charge, ParticleIndex>|
         -> Visit {
            let node = &mut particles[usize::from(index)];
            let mut x = quad.value().x - node.x;
            let mut y = quad.value().y - node.y;

            let mut l = x * x + y * y;
            let mut w = quad.extent().x1 - quad.extent().x0;

            if w * w / self.theta_2 < l {
                if l < self.distance_max_2 {
                    if x == 0.0 {
                        x = jiggle(random);
                        l += x * x;
                    }
                    if y == 0.0 {
                        y = jiggle(random);
                        l += y * y;
                    }
                    if l < self.distance_min_2 {
                        l = (self.distance_min_2 * l).sqrt();
                    }
                    node.vx += x * quad.value().strength * alpha / l;
                    node.vy += y * quad.value().strength * alpha / l;
                }
                return Visit::Skip;
            } else if (matches!(quad.inner(), Entry::Internal { .. }) || l >= self.distance_max_2) {
                // We visit the points directly.
                return Visit::Continue;
            }

            let Entry::Leaf { data, others, .. } = quad.inner() else {
                // TODO(grtlr): clean this up!
                unreachable!();
            };

            if *data != index || others.is_some() {
                if x == 0.0 {
                    x = jiggle(random);
                    l += x * x;
                }
                if y == 0.0 {
                    y = jiggle(random);
                    l += y * y;
                }
                if l < self.distance_min_2 {
                    l = (self.distance_min_2 * l).sqrt();
                }
            }

            let rest = &others.unwrap_or_default()[0..];
            for quad_index in [&[data], rest]
                .concat()
                .into_iter()
                .filter(|&&i| i != index)
            {
                w = self.strengths[usize::from(*quad_index)] * alpha / l;
                node.vx += x * w;
                node.vy += y * w;
            }

            Visit::Continue
        };

        let mut tree = Quadtree::<Charge, ParticleIndex>::from_particles(
            particles.iter().map(|n| (n.x, n.y, n.index)),
        );
        tree.visit_after(accumulate);

        let tmp = particles
            .iter()
            .map(|node| (node.index))
            .collect::<Vec<_>>();

        for index in tmp {
            tree.visit(|quad| apply(index, particles, quad));
        }
    }
}

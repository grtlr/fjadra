use std::cmp;
use std::collections::HashMap;

use crate::lcg::LCG;

use super::{
    jiggle::jiggle,
    particle::{NodeIndex, Particle},
};

pub struct LinkFn(Box<dyn Fn(&(NodeIndex, NodeIndex), usize) -> f64>);

impl From<f64> for LinkFn {
    fn from(value: f64) -> Self {
        Self(Box::new(move |_, _| value))
    }
}

impl<F> From<F> for LinkFn
where
    F: Fn(&(NodeIndex, NodeIndex), usize) -> f64 + 'static,
{
    fn from(f: F) -> Self {
        LinkFn(Box::new(f))
    }
}

pub struct Link {
    links: Vec<(NodeIndex, NodeIndex)>,
    strength_fn: Option<LinkFn>,
    distance_fn: LinkFn,
    iterations: usize,
}

impl Link {
    pub fn new(links: impl Iterator<Item = (usize, usize)>) -> Self {
        Self {
            links: links.map(|(a, b)| (a.into(), b.into())).collect(),
            distance_fn: 30.0.into(),
            strength_fn: None,
            iterations: 1,
        }
    }

    pub fn distance(mut self, f: impl Into<LinkFn>) -> Self {
        self.distance_fn = f.into();
        self
    }

    pub fn strength(mut self, f: impl Into<LinkFn>) -> Self {
        self.strength_fn = Some(f.into());
        self
    }

    pub fn iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    pub(super) fn initialize(mut self, particles: &[Particle]) -> Option<LinkForce> {
        if particles.is_empty() {
            return None;
        }

        // TODO(grtlr): This is in array d3.
        let mut count = HashMap::new();
        for link in &self.links {
            *count.entry(link.0.clone()).or_insert(0) += 1;
            *count.entry(link.1.clone()).or_insert(0) += 1;
        }

        let bias = self
            .links
            .iter()
            .cloned()
            .map(|link| count[&link.0] as f64 / (count[&link.0] + count[&link.1]) as f64)
            .collect();

        let default_strength = LinkFn::from(move |link: &(NodeIndex, NodeIndex), _| {
            1.0 / usize::min(count[&link.0], count[&link.1]) as f64
        });

        let strength = self.strength_fn.take().unwrap_or(default_strength);

        let strengths = self
            .links
            .iter()
            .enumerate()
            .map(|(i, link)| strength.0(link, i))
            .collect();

        let distances = self
            .links
            .iter()
            .enumerate()
            .map(|(i, link)| self.distance_fn.0(link, i))
            .collect();

        Some(LinkForce {
            links: self.links,
            bias,
            strengths,
            distances,
            iterations: self.iterations,
        })
    }
}

#[derive(Debug)]
pub struct LinkForce {
    links: Vec<(NodeIndex, NodeIndex)>,

    bias: Vec<f64>,

    strengths: Vec<f64>,
    distances: Vec<f64>,
    iterations: usize,
}

fn get_pair_mut(
    slice: &mut [Particle],
    i: NodeIndex,
    j: NodeIndex,
) -> Option<(&mut Particle, &mut Particle)> {
    if i == j {
        return None;
    }

    let first = usize::from(cmp::min(i, j));
    let second = usize::from(cmp::max(i, j));

    let (left, right) = slice.split_at_mut(second);

    if i < j {
        Some((&mut left[first], &mut right[0]))
    } else {
        Some((&mut right[0], &mut left[first]))
    }
}

impl LinkForce {
    pub fn force(&mut self, alpha: f64, random: &mut LCG, particles: &mut [Particle]) {
        for _ in 0..self.iterations {
            for (i, link) in self.links.iter().enumerate() {
                let (source, target) = link;
                let Some((source, target)) = get_pair_mut(particles, *source, *target) else {
                    // Don't apply forces if we an edge where `source == target`.
                    continue;
                };

                let mut x = target.x + target.vx - source.x - source.vx;
                if x == 0.0 {
                    x = jiggle(random);
                }
                let mut y = target.y + target.vy - source.y - source.vy;
                if y == 0.0 {
                    y = jiggle(random);
                }
                let l = x.hypot(y);
                let l = (l - self.distances[i]) / l * alpha * self.strengths[i];

                let bias_target = self.bias[i];
                let bias_source = 1.0 - bias_target;

                target.vx -= x * l * bias_target;
                target.vy -= y * l * bias_target;
                source.vx += x * l * bias_source;
                source.vy += y * l * bias_source;
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn retrieve_two_mutable_borrows() {
        let mut particles = vec![
            Particle::new(0, 0.0, 0.0),
            Particle::new(1, 1.0, 1.0),
            Particle::new(2, 2.0, 2.0),
        ];

        let (a, b) = get_pair_mut(&mut particles, 0.into(), 1.into()).unwrap();

        assert_eq!(a.index, 0.into());
        assert_eq!(b.index, 1.into());
    }

    #[test]
    fn retrieve_two_mutable_borrows_reverse() {
        let mut particles = vec![
            Particle::new(0, 0.0, 0.0),
            Particle::new(1, 1.0, 1.0),
            Particle::new(2, 2.0, 2.0),
        ];

        let (a, b) = get_pair_mut(&mut particles, 1.into(), 0.into()).unwrap();

        assert_eq!(a.index, 1.into());
        assert_eq!(b.index, 0.into());
    }
}

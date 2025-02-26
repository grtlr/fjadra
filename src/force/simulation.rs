use std::collections::BTreeMap;

use crate::lcg::Lcg;

use super::center::CenterForce;
use super::node::Node;
use super::position::{PositionXForce, PositionYForce};
use super::{collide::CollideForce, link::LinkForce, many_body::ManyBodyForce, particle::Particle};

pub trait ForceBuilder {
    fn initialize(self, particles: &[Particle]) -> Force;
}

pub enum Force {
    Collide(CollideForce),
    Center(CenterForce),
    PositionX(PositionXForce),
    PositionY(PositionYForce),
    Link(LinkForce),
    ManyBody(ManyBodyForce),
}

#[derive(Debug)]
pub struct SimulationBuilder {
    alpha: f64,
    alpha_min: f64,
    alpha_decay: f64,
    alpha_target: f64,
    velocity_decay: f64,
    random: Lcg,
}

impl Default for SimulationBuilder {
    fn default() -> Self {
        let alpha_min = 0.001;
        Self {
            alpha: 1.0,
            alpha_min,
            alpha_decay: 1.0 - alpha_min.powf(1.0 / 300.0),
            alpha_target: 0.0,
            velocity_decay: 0.6,
            random: Lcg::default(),
        }
    }
}

impl SimulationBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn with_alpha_min(mut self, alpha_min: f64) -> Self {
        self.alpha_min = alpha_min;
        self
    }

    pub fn with_alpha_decay(mut self, alpha_decay: f64) -> Self {
        self.alpha_decay = alpha_decay;
        self
    }

    pub fn with_alpha_target(mut self, alpha_target: f64) -> Self {
        self.alpha_target = alpha_target;
        self
    }

    pub fn with_velocity_decay(mut self, velocity_decay: f64) -> Self {
        self.velocity_decay = velocity_decay;
        self
    }

    pub fn with_random(mut self, random: Lcg) -> Self {
        self.random = random;
        self
    }
}

/// Creates the initial position of particles.
fn initial_position(index: usize) -> [f64; 2] {
    let initial_radius = 10.0;
    let initial_angle = std::f64::consts::PI * (3.0 - (5.0f64).sqrt());

    let radius = initial_radius * (0.5 + index as f64).sqrt();
    let angle = index as f64 * initial_angle;
    [radius * angle.cos(), radius * angle.sin()]
}

impl SimulationBuilder {
    pub fn build<N>(&self, particles: impl IntoIterator<Item = N>) -> Simulation
    where
        N: Into<Node>,
    {
        let particles = particles
            .into_iter()
            .enumerate()
            .map(|(ix, p)| p.into().build_with_pos(ix.into(), || initial_position(ix)))
            .collect();

        Simulation {
            alpha: self.alpha,
            alpha_min: self.alpha_min,
            alpha_decay: self.alpha_decay,
            alpha_target: self.alpha_target,
            velocity_decay: self.velocity_decay,
            particles,
            random: self.random.clone(),
            forces: Default::default(),
        }
    }
}

pub struct Simulation {
    alpha: f64,
    alpha_min: f64,
    alpha_decay: f64,
    alpha_target: f64,
    velocity_decay: f64,
    random: Lcg,
    forces: BTreeMap<String, Force>,
    particles: Vec<Particle>,
}

pub struct SimulationIter<'a> {
    simulation: &'a mut Simulation,
    finished: bool,
    emitted: bool,
}

impl<'a> Iterator for SimulationIter<'a> {
    type Item = Vec<[f64; 2]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.emitted {
            return None;
        }

        if self.finished {
            self.emitted = true;
            return Some(self.simulation.positions().collect());
        } else {
            self.simulation.tick(1);
            self.finished = self.simulation.is_finished();
            Some(self.simulation.positions().collect())
        }
    }
}

impl Simulation {
    /// Performs a full simulation, until the alpha value reaches the minimum.
    pub fn step(&mut self) {
        while self.alpha > self.alpha_min {
            self.tick(1);
        }
    }

    /// Checks if the simulation has finished.
    pub fn is_finished(&self) -> bool {
        self.alpha <= self.alpha_min
    }

    /// Advances the simulation by a number of iterations.
    pub fn tick(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.alpha += (self.alpha_target - self.alpha) * self.alpha_decay;

            for force in &mut self.forces.values_mut() {
                match force {
                    Force::Collide(c) => c.force(&mut self.random, &mut self.particles),
                    Force::Center(c) => c.force(&mut self.particles),
                    Force::PositionX(p) => p.force(self.alpha, &mut self.particles),
                    Force::PositionY(p) => p.force(self.alpha, &mut self.particles),
                    Force::Link(l) => l.force(self.alpha, &mut self.random, &mut self.particles),
                    Force::ManyBody(m) => {
                        m.force(self.alpha, &mut self.random, &mut self.particles);
                    }
                }
            }

            for n in &mut self.particles {
                n.apply_velocities(self.velocity_decay);
            }
        }
    }

    /// Returns the names of the forces in the simulation.
    pub fn forces(&self) -> impl Iterator<Item = &str> {
        self.forces.keys().map(|k| k.as_str())
    }

    /// Returns the positions of the particles in the simulation.
    ///
    /// The ordering of the nodes in the simulation is stable, so the order of
    /// the positions will be the same as initially supplied.
    pub fn positions(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.particles.iter().map(|n: &Particle| [n.x, n.y])
    }

    /// Adds a force, defined by a [`ForceBuilder`], to the simulation.
    ///
    /// The [`ForceBuilder`] usually does some initialization of auxiliary data structures.
    ///
    /// Some examples are:
    /// * [`Center`](crate::force::Center)
    /// * [`PositionX`](crate::force::position::PositionX) and [`PositionY`](crate::force::position::PositionY)
    /// * [`Link`](crate::force::link::Link) and [`ManyBody`](crate::force::many_body::ManyBody)
    pub fn add_force(mut self, name: impl AsRef<str>, force: impl ForceBuilder) -> Self {
        let force = force.initialize(&self.particles);
        self.forces.insert(name.as_ref().to_owned(), force);
        self
    }

    /// Removes a force from the simulation.
    ///
    /// Returns `true` if the force was removed, `false` otherwise.
    pub fn remove_force(&mut self, name: impl AsRef<str>) -> bool {
        self.forces.remove(name.as_ref()).is_some()
    }

    pub fn iter(&mut self) -> SimulationIter<'_> {
        let emitted = self.is_finished();
        SimulationIter {
            simulation: self,
            finished: false,
            emitted,
        }
    }

    /// Sets the alpha value of the simulation.
    ///
    /// This is can be used to restart the simulation.
    pub fn set_alpha(&mut self, alpha: f64) {
        self.alpha = alpha;
    }
}

#[cfg(test)]
mod test {
    use crate::{ManyBody, PositionX, PositionY};

    use super::*;

    #[test]
    fn respects_fixed_positions() {
        let mut simulation = SimulationBuilder::default()
            .build([
                Node::default().fixed_position(100.0, 100.0),
                Node::default().fixed_position(-100.0, -100.0),
                Node::default().position(42.0, 42.0),
                Node::default(),
            ])
            // The following should normally pull these nodes to (0,0).
            .add_force("x", PositionX::default())
            .add_force("y", PositionY::default());

        let positions = simulation.iter().last().unwrap();

        assert_eq!(positions[0][0], 100.0);
        assert_eq!(positions[0][1], 100.0);
        assert_eq!(positions[1][0], -100.0);
        assert_eq!(positions[1][1], -100.0);

        approx::assert_abs_diff_eq!(positions[2][0], 0.0, epsilon = 0.0001);
        approx::assert_abs_diff_eq!(positions[2][1], 0.0, epsilon = 0.0001);

        approx::assert_abs_diff_eq!(positions[3][0], 0.0, epsilon = 0.0001);
        approx::assert_abs_diff_eq!(positions[3][1], 0.0, epsilon = 0.0001);
    }

    #[test]
    fn prevent_crash_for_large_values() {
        let mut simulation = SimulationBuilder::default()
            .build([
                Node::default(),
                Node::default(),
                Node::default(),
                Node::default(),
            ])
            // This force should send the nodes flying away from each other indefinetly.
            .add_force("charge", ManyBody::default().strength(f64::MIN));

        let _ = simulation.iter().last().unwrap();
    }
}

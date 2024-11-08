use std::collections::BTreeMap;

use crate::lcg::LCG;

use super::collide::Collide;
use super::position::{PositionXForce, PositionYForce};
use super::{
    collide::CollideForce,
    link::LinkForce,
    many_body::{ManyBody, ManyBodyForce},
    particle::Particle,
    position::{PositionX, PositionY},
    Link,
};

enum Force {
    Collide(CollideForce),
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
    random: LCG,
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
            random: LCG::default(),
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

    pub fn with_random(mut self, random: LCG) -> Self {
        self.random = random;
        self
    }
}

impl SimulationBuilder {
    // TODO(grtlr): build with fixed positions!

    pub fn build<P>(&self, particles: impl IntoIterator<Item = Option<P>>) -> Simulation
    where
        P: Into<[f64; 2]>,
    {
        let initial_radius = 10.0;
        let initial_angle = std::f64::consts::PI * (3.0 - (5.0f64).sqrt());

        let particles = particles.into_iter().enumerate().map(|(ix, p)| {
            let [x, y] = p.map(|x| x.into()).unwrap_or_else(|| {
                let radius = initial_radius * (0.5 + ix as f64).sqrt();
                let angle = ix as f64 * initial_angle;
                [radius * angle.cos(), radius * angle.sin()]
            });
            Particle::new(ix, x, y)
        });

        Simulation {
            alpha: self.alpha,
            alpha_min: self.alpha_min,
            alpha_decay: self.alpha_decay,
            alpha_target: self.alpha_target,
            velocity_decay: self.velocity_decay,
            particles: particles.collect(),
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
    random: LCG,
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
            self.finished = self.simulation.finished();
            Some(self.simulation.positions().collect())
        }
    }
}

impl Simulation {
    pub fn step(&mut self) {
        while self.alpha > self.alpha_min {
            self.tick(1);
        }
    }

    pub fn finished(&self) -> bool {
        self.alpha <= self.alpha_min
    }

    pub fn tick(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.alpha += (self.alpha_target - self.alpha) * self.alpha_decay;

            for force in &mut self.forces.values_mut() {
                match force {
                    Force::Collide(c) => c.force(&mut self.random, &mut self.particles),
                    Force::PositionX(p) => p.force(self.alpha, &mut self.particles),
                    Force::PositionY(p) => p.force(self.alpha, &mut self.particles),
                    Force::Link(l) => l.force(self.alpha, &mut self.random, &mut self.particles),
                    Force::ManyBody(m) => {
                        m.force(self.alpha, &mut self.random, &mut self.particles)
                    }
                }
            }

            for n in &mut self.particles {
                n.apply_velocities(self.velocity_decay);
            }
        }
    }

    pub fn positions(&self) -> impl Iterator<Item = [f64; 2]> + '_ {
        self.particles.iter().map(|n: &Particle| [n.x, n.y])
    }

    pub fn add_force_collide(mut self, name: impl ToString, force: Collide) -> Self {
        let force = force.initialize(&self.particles);
        self.forces.insert(name.to_string(), Force::Collide(force));
        self
    }

    pub fn add_force_x(mut self, name: impl ToString, force: PositionX) -> Self {
        let force = force.initialize();
        self.forces
            .insert(name.to_string(), Force::PositionX(force));
        self
    }

    pub fn add_force_y(mut self, name: impl ToString, force: PositionY) -> Self {
        let force = force.initialize();
        self.forces
            .insert(name.to_string(), Force::PositionY(force));
        self
    }

    pub fn add_force_link(mut self, name: impl ToString, force: Link) -> Self {
        if let Some(force) = force.initialize(&self.particles) {
            self.forces.insert(name.to_string(), Force::Link(force));
        }
        self
    }

    pub fn add_force_many_body(mut self, name: impl ToString, force: ManyBody) -> Self {
        let force = force.initialize(&self.particles);
        self.forces.insert(name.to_string(), Force::ManyBody(force));
        self
    }

    pub fn iter(&mut self) -> SimulationIter<'_> {
        let emitted = self.finished();
        SimulationIter {
            simulation: self,
            finished: false,
            emitted,
        }
    }
}

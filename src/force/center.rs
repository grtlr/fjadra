use super::{
    particle::Particle,
    simulation::{Force, ForceBuilder},
};

#[derive(Clone, Debug)]
pub struct Center {
    strength: f64,
    x: f64,
    y: f64,
}

impl Default for Center {
    fn default() -> Self {
        Self {
            strength: 1.0,
            x: 0.0,
            y: 0.0,
        }
    }
}

impl Center {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }

    pub fn strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }
}

impl ForceBuilder for Center {
    fn initialize(self, _: &[super::particle::Particle]) -> Force {
        Force::Center(CenterForce {
            strength: self.strength,
            x: self.x,
            y: self.y,
        })
    }
}

#[derive(Clone, Debug)]
pub struct CenterForce {
    strength: f64,
    x: f64,
    y: f64,
}

impl CenterForce {
    pub fn force(&self, particles: &mut [Particle]) {
        let mut sx = 0.;
        let mut sy = 0.;

        for node in particles.iter() {
            sx += node.x;
            sy += node.y;
        }

        sx = (sx / particles.len() as f64 - self.x) * self.strength;
        sy = (sy / particles.len() as f64 - self.y) * self.strength;

        for node in particles.iter_mut() {
            node.x -= sx;
            node.y -= sy;
        }
    }
}

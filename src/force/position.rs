use super::{particle::Particle, simulation::Force, ForceBuilder};

pub struct PositionX {
    strength: f64,
    x: f64,
}

impl Default for PositionX {
    fn default() -> Self {
        Self {
            strength: 0.1,
            x: 0.0,
        }
    }
}

impl PositionX {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }

    pub fn x(mut self, x: f64) -> Self {
        self.x = x;
        self
    }
}

impl ForceBuilder for PositionX {
    fn initialize(self, _: &[Particle]) -> Force {
        Force::PositionX(PositionXForce {
            strength: self.strength,
            x: self.x,
        })
    }
}

#[derive(Clone, Debug)]
pub struct PositionXForce {
    strength: f64,
    x: f64,
}

impl PositionXForce {
    pub fn force(&self, alpha: f64, particles: &mut [Particle]) {
        let strengths = std::iter::repeat(self.strength);

        for (node, si) in particles.iter_mut().zip(strengths) {
            let d = self.x - node.x;
            node.vx += d * si * alpha;
        }
    }
}

#[derive(Clone, Debug)]
pub struct PositionY {
    strength: f64,
    y: f64,
}

impl Default for PositionY {
    fn default() -> Self {
        Self {
            strength: 0.1,
            y: 0.0,
        }
    }
}

impl PositionY {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }

    pub fn y(mut self, y: f64) -> Self {
        self.y = y;
        self
    }
}

impl ForceBuilder for PositionY {
    fn initialize(self, _: &[Particle]) -> Force {
        Force::PositionY(PositionYForce {
            strength: self.strength,
            y: self.y,
        })
    }
}

#[derive(Clone, Debug)]
pub struct PositionYForce {
    strength: f64,
    y: f64,
}

impl PositionYForce {
    pub fn force(&self, alpha: f64, particles: &mut [Particle]) {
        let strengths = std::iter::repeat(self.strength);

        for (node, si) in particles.iter_mut().zip(strengths) {
            let d = self.y - node.y;
            node.vy += d * si * alpha;
        }
    }
}

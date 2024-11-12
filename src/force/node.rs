use super::particle::{Particle, ParticleIndex};

/// A builder for creating particles.
#[derive(Clone, Debug, Default)]
pub struct Node {
    position: Option<[f64; 2]>,
    velocity: [f64; 2],
    fixed: bool,
}

impl Node {
    /// Set the initial position of the particle.
    #[inline(always)]
    pub fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Some([x, y]);
        self
    }

    /// Sets the initial position of the particle and prevents it from moving.
    #[inline(always)]
    pub fn fixed_position(mut self, x: f64, y: f64) -> Self {
        self = self.position(x, y);
        self.fixed = true;
        self
    }

    /// Builds a particle with a given position if it is not already set.
    pub(super) fn build_with_pos(
        self,
        index: ParticleIndex,
        pos_fn: impl FnMut() -> [f64; 2],
    ) -> Particle {
        let [x, y] = self.position.unwrap_or_else(pos_fn);
        let [vx, vy] = self.velocity;
        Particle {
            x,
            y,
            vx,
            vy,
            index,
            fx: if self.fixed { Some(x) } else { None },
            fy: if self.fixed { Some(y) } else { None },
        }
    }
}

impl From<[f64; 2]> for Node {
    fn from(position: [f64; 2]) -> Self {
        Self {
            position: Some(position),
            velocity: Default::default(),
            fixed: false,
        }
    }
}

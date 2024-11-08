use std::hash::Hash;

/// Reflects the index in the input list of particles.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NodeIndex(usize);

impl From<NodeIndex> for usize {
    fn from(index: NodeIndex) -> Self {
        index.0
    }
}

impl From<usize> for NodeIndex {
    fn from(index: usize) -> Self {
        NodeIndex(index)
    }
}

#[derive(Clone, Debug)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub index: NodeIndex,
    // The following fields signal that a node is fixed in a certain direction.
    // TODO(grtlr): Move this to a separate `Vec` in the simulation to improve the memory layout.
    pub fx: Option<f64>,
    pub fy: Option<f64>,
}

impl Particle {
    pub fn new(index: impl Into<NodeIndex>, x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            fx: None,
            fy: None,
            index: index.into(),
        }
    }

    pub fn with_fixed_x(mut self) -> Self {
        self.fx = Some(self.x);
        self
    }

    pub fn with_fixed_y(mut self) -> Self {
        self.fx = Some(self.x);
        self
    }

    /// Applies the velocity to the vectors, while respecting fixed positions.
    pub(crate) fn apply_velocities(&mut self, velocity_decay: f64) {
        if let Some(fx) = self.fx {
            self.x = fx;
            self.vx = 0.0;
        } else {
            self.x += self.vx;
            self.vx *= velocity_decay;
        }

        if let Some(fy) = self.fy {
            self.y = fy;
            self.vy = 0.0;
        } else {
            self.y += self.vy;
            self.vy *= velocity_decay;
        }
    }
}

impl From<Particle> for [f64; 2] {
    fn from(p: Particle) -> Self {
        [p.x, p.y]
    }
}

impl<I> From<(I, [f64; 2])> for Particle
where
    I: Into<NodeIndex>,
{
    fn from((ix, p): (I, [f64; 2])) -> Self {
        Self::new(ix.into(), p[0], p[1])
    }
}

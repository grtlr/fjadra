#[derive(Debug)]
pub struct Indexer {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    xm: f64,
    ym: f64,
}

impl Indexer {
    pub fn with_extent(min: [f64; 2], max: [f64; 2]) -> Self {
        Self {
            x0: min[0],
            y0: min[1],
            x1: max[0],
            y1: max[1],
            xm: (min[0] + max[0]) / 2.0,
            ym: (min[1] + max[1]) / 2.0,
        }
    }

    pub fn get(&self, x: f64, y: f64) -> usize {
        let right = x >= self.xm;
        let bottom = y >= self.ym;
        (bottom as usize) << 1 | right as usize
    }

    pub fn get_and_descend(&mut self, x: f64, y: f64) -> usize {
        let right = if x >= self.xm {
            self.x0 = self.xm;
            true
        } else {
            self.x1 = self.xm;
            false
        };

        let bottom = if y >= self.ym {
            self.y0 = self.ym;
            true
        } else {
            self.y1 = self.ym;
            false
        };

        self.xm = (self.x0 + self.x1) / 2.0;
        self.ym = (self.y0 + self.y1) / 2.0;

        (bottom as usize) << 1 | right as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Indexer {
        pub fn extent(&self) -> ([f64; 2], [f64; 2]) {
            ([self.x0, self.y0], [self.x1, self.y1])
        }
    }

    #[test]
    fn simple_indexation() {
        let ix = Indexer::with_extent([0.0, 0.0], [1.0, 1.0]);
        assert_eq!(ix.get(0.1, 0.1), 0);
        assert_eq!(ix.get(0.9, 0.1), 1);
        assert_eq!(ix.get(0.1, 0.9), 2);
        assert_eq!(ix.get(0.9, 0.9), 3);
    }

    #[test]
    fn nested_indexation() {
        let mut ix = Indexer::with_extent([0.0, 0.0], [1.0, 1.0]);
        assert_eq!(ix.get(0.1, 0.1), 0);
        assert_eq!(ix.get(0.9, 0.1), 1);
        assert_eq!(ix.get(0.1, 0.9), 2);
        assert_eq!(ix.get(0.9, 0.9), 3);
        assert_eq!(ix.get(0.4, 0.4), 0);
        assert_eq!(ix.get_and_descend(0.4, 0.4), 0);
        assert_eq!(ix.extent(), ([0.0, 0.0], [0.5, 0.5]));
        assert_eq!(ix.xm, 0.25);
        assert_eq!(ix.ym, 0.25);
        assert_eq!(ix.get(0.1, 0.1), 0);
        assert_eq!(ix.get(0.4, 0.4), 3);
    }
}

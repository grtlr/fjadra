use super::{
    quad::{Quad, QuadInner},
    Node, Quadtree,
};

impl<Q: Default, T> Quadtree<Q, T> {
    pub fn visit_after<F>(&mut self, mut callback: F)
    where
        F: FnMut(Quad<'_, Q, T>),
    {
        let Some(root) = self.root.as_ref() else {
            return;
        };

        let mut quads = vec![QuadInner {
            extent: [
                [self.x0 as f64, self.y0 as f64],
                [self.x1 as f64, self.y1 as f64],
            ]
            .into(),
            node: root,
        }];
        let mut next = Vec::new();

        while let Some(q) = quads.pop() {
            if let Node::Internal {
                children: [c0, c1, c2, c3],
                ..
            } = q.node
            {
                let xm = (q.extent.x0 + q.extent.x1) / 2.0;
                let ym = (q.extent.y0 + q.extent.y1) / 2.0;
                if let Some(node) = c0.as_ref() {
                    quads.push(QuadInner {
                        extent: [q.extent.x0, q.extent.y0, xm, ym].into(),
                        node,
                    });
                }
                if let Some(node) = c1.as_ref() {
                    quads.push(QuadInner {
                        extent: [xm, q.extent.y0, q.extent.x1, ym].into(),
                        node,
                    });
                }
                if let Some(node) = c2.as_ref() {
                    quads.push(QuadInner {
                        extent: [q.extent.x0, ym, xm, q.extent.y1].into(),
                        node,
                    });
                }
                if let Some(node) = c3.as_ref() {
                    quads.push(QuadInner {
                        extent: [xm, ym, q.extent.x1, q.extent.y1].into(),
                        node,
                    });
                }
            }
            next.push(q);
        }
        while let Some(q) = next.pop() {
            callback(Quad::from_quad(&mut self.store, &q));
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lcg::LCG, quadtree::quad::Entry};

    type Quadtree = super::Quadtree<(), ()>;

    #[test]
    fn vists_each_node_in_the_quadtree() {
        let mut results = Vec::new();

        let mut q = Quadtree::default();
        q.add_all([(0., 0., ()), (1., 0., ()), (0., 1., ()), (1., 1., ())]);
        q.visit_after(|quad| {
            results.push([
                quad.extent().x0,
                quad.extent().y0,
                quad.extent().x1,
                quad.extent().y1,
            ]);
        });
        assert_eq!(
            &results,
            &[
                [0., 0., 1., 1.],
                [1., 0., 2., 1.],
                [0., 1., 1., 2.],
                [1., 1., 2., 2.],
                [0., 0., 2., 2.],
            ]
        )
    }

    #[test]
    fn applies_post_order_traversal() {
        let mut results = Vec::new();

        let mut q = Quadtree::with_extent([0., 0.], [960., 960.]);
        q.add_all([(100., 100., ()), (200., 200., ()), (300., 300., ())]);
        q.visit_after(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
        });
        assert_eq!(
            &results,
            &[
                [0., 0., 128., 128.],
                [128., 128., 256., 256.],
                [0., 0., 256., 256.],
                [256., 256., 512., 512.],
                [0., 0., 512., 512.],
                [0., 0., 1024., 1024.],
            ]
        );
    }

    #[test]
    fn empty_quadtree_with_no_bounds_does_nothing() {
        let mut results = Vec::new();

        let mut q = Quadtree::default();
        q.visit_after(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
        });
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn empty_quadtree_with_bounds_does_nothing() {
        let mut results = Vec::new();

        let mut q = Quadtree::with_extent([0., 0.], [960., 960.]);
        q.visit_after(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
        });
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn accumulate_particles_count() {
        let mut q = super::Quadtree::<usize>::default();

        let n: usize = 20;
        let mut random = LCG::new(1);

        for _ in 0..n {
            let (x, y) = (random.next().unwrap(), random.next().unwrap());
            q.insert_default(x, y);
        }

        q.visit_after(|mut quad| {
            let r = match quad.inner() {
                // We need to add one to the end because leafs are guaranteed to have at least one
                // value.
                Entry::Leaf { others, .. } => others.map(|o| o.len()).unwrap_or(0) + 1,
                Entry::Internal { children } => children.iter().filter_map(|&c| c).sum(),
            };
            *quad.value_mut() = r;
        });
        assert_eq!(*q.store.get(q.root().unwrap().handle()), n);
    }
}

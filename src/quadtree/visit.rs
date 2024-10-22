use super::{
    quad::{Quad, QuadInner},
    Node, Quadtree,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visit {
    Continue,
    Skip,
}

impl Visit {
    pub fn continue_if(condition: bool) -> Self {
        if condition {
            Visit::Continue
        } else {
            Visit::Skip
        }
    }

    pub fn stop_if(condition: bool) -> Self {
        if condition {
            Visit::Skip
        } else {
            Visit::Continue
        }
    }
}

impl<Q: Default, T> Quadtree<Q, T> {
    pub fn visit<F>(&mut self, mut callback: F)
    where
        F: FnMut(Quad<'_, Q, T>) -> Visit,
    {
        let Some(root) = self.root.as_ref() else {
            return;
        };

        let mut stack = vec![QuadInner {
            extent: [
                [self.x0 as f64, self.y0 as f64],
                [self.x1 as f64, self.y1 as f64],
            ]
            .into(),
            node: root,
        }];

        while let Some(q) = stack.pop() {
            let extent = q.extent;
            if callback(Quad::from_quad(&mut self.store, &q)) == Visit::Continue {
                if let Node::Internal {
                    children: [c0, c1, c2, c3],
                    ..
                } = q.node
                {
                    let xm = (extent.x0 + extent.x1) / 2.0;
                    let ym = (extent.y0 + extent.y1) / 2.0;
                    if let Some(node) = c3.as_ref() {
                        stack.push(QuadInner::new(node, xm, ym, extent.x1, extent.y1));
                    }
                    if let Some(node) = c2.as_ref() {
                        stack.push(QuadInner::new(node, extent.x0, ym, xm, extent.y1));
                    }
                    if let Some(node) = c1.as_ref() {
                        stack.push(QuadInner::new(node, xm, extent.y0, extent.x1, ym));
                    }
                    if let Some(node) = c0.as_ref() {
                        stack.push(QuadInner::new(node, extent.x0, extent.y0, xm, ym));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::quadtree::visit::Visit;

    type Quadtree = super::Quadtree<(), ()>;

    #[test]
    fn vists_each_node_in_the_quadtree() {
        let mut results = Vec::new();

        let mut q = Quadtree::default();
        q.add_all([(0., 0., ()), (1., 0., ()), (0., 1., ()), (1., 1., ())]);
        q.visit(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
            Visit::Continue
        });
        assert_eq!(
            &results,
            &[
                [0., 0., 2., 2.],
                [0., 0., 1., 1.],
                [1., 0., 2., 1.],
                [0., 1., 1., 2.],
                [1., 1., 2., 2.]
            ]
        )
    }

    #[test]
    fn vists_each_node_in_the_quadtree_for_small_and_negative_positions() {
        let mut results = Vec::new();

        let mut q = Quadtree::default();
        q.add_all([
            (-0.5, -0.5, ()),
            (-0.5, 0.5, ()),
            (0.5, -0.5, ()),
            (0.5, 0.5, ()),
        ]);
        q.visit(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
            Visit::Continue
        });
        assert_eq!(
            &results,
            &[
                [-1.0, -1.0, 1.0, 1.0],
                [-1.0, -1.0, 0.0, 0.0],
                [0.0, -1.0, 1.0, 0.0],
                [-1.0, 0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 1.0]
            ]
        )
    }

    #[test]
    fn applies_pre_order_traversal() {
        let mut results = Vec::new();

        let mut q = Quadtree::with_extent([0., 0.], [960., 960.]);
        q.add_all([(100., 100., ()), (200., 200., ()), (300., 300., ())]);
        q.visit(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
            Visit::Continue
        });
        assert_eq!(
            &results,
            &[
                [0., 0., 1024., 1024.],
                [0., 0., 512., 512.],
                [0., 0., 256., 256.],
                [0., 0., 128., 128.],
                [128., 128., 256., 256.],
                [256., 256., 512., 512.]
            ]
        );
    }

    #[test]
    fn does_not_recurse_if_callback_returns_true() {
        let mut results = Vec::new();

        let mut q = Quadtree::with_extent([0., 0.], [960., 960.]);
        q.add_all([(100., 100., ()), (700., 700., ()), (800., 800., ())]);
        q.visit(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
            Visit::stop_if(quad.extent().x0 > 0.0)
        });
        assert_eq!(
            &results,
            &[
                [0., 0., 1024., 1024.],
                [0., 0., 512., 512.],
                [512., 512., 1024., 1024.]
            ]
        );
    }

    #[test]
    fn empty_quadtree_with_no_bounds_does_nothing() {
        let mut results = Vec::new();

        let mut q = Quadtree::default();
        q.visit(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
            Visit::Continue
        });
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn empty_quadtree_with_bounds_does_nothing() {
        let mut results = Vec::new();

        let mut q = Quadtree::with_extent([0., 0.], [960., 960.]);
        q.visit(|quad| {
            results.push(<[f64; 4]>::from(quad.extent()));
            Visit::Continue
        });
        assert_eq!(results.len(), 0);
    }
}

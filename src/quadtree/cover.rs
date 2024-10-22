use super::{Node, Quadtree};

impl<Q: Default, T> Quadtree<Q, T> {
    pub fn cover(&mut self, x: f64, y: f64) {
        assert!(!f64::is_nan(x), "Encountered NaN value for x");
        assert!(!f64::is_nan(y), "Encountered NaN value for y");

        let x = x.floor() as i64;
        let y = y.floor() as i64;

        let mut x0 = self.x0;
        let mut y0 = self.y0;
        let mut x1 = self.x1;
        let mut y1 = self.y1;

        if x0 > x1 {
            x0 = x;
            x1 = x0 + 1;
            y0 = y;
            y1 = y0 + 1;
        } else {
            // Otherwise, double repeatedly to cover.
            let mut z = if (x1 - x0).is_positive() { x1 - x0 } else { 1 };

            let node = if matches!(self.root(), Some(&Node::Internal { .. })) {
                &mut self.root
            } else {
                &mut None
            };

            while x0 > x || x >= x1 || y0 > y || y >= y1 {
                let i = ((y < y0) as usize) << 1 | ((x < x0) as usize);

                let mut children = [None, None, None, None];
                children[i] = node.take();
                *node = Some(Box::new(Node::Internal {
                    children,
                    value: None,
                    handle: self.store.insert(Default::default()),
                }));

                z *= 2;
                match i {
                    0 => {
                        x1 = x0 + z;
                        y1 = y0 + z;
                    }
                    1 => {
                        x0 = x1 - z;
                        y1 = y0 + z;
                    }
                    2 => {
                        x1 = x0 + z;
                        y0 = y1 - z;
                    }
                    3 => {
                        x0 = x1 - z;
                        y0 = y1 - z;
                    }
                    _ => unreachable!(),
                }
            }
        }

        self.x0 = x0;
        self.y0 = y0;
        self.x1 = x1;
        self.y1 = y1;
    }
}

#[cfg(test)]
mod test {
    use super::Node;

    type Quadtree = super::Quadtree<()>;

    #[test]
    fn sets_a_trivial_extent_if_the_extent_was_undefined() {
        let mut q = Quadtree::default();
        q.cover(1., 2.);
        assert_eq!(q.extent(), ([1, 2], [2, 3]));
    }

    #[test]
    fn sets_a_non_trivial_squarified_and_centered_extent_if_the_extent_was_trivial() {
        let mut q = Quadtree::default();
        q.cover(0., 0.);
        q.cover(1., 2.);
        assert_eq!(q.extent(), ([0, 0], [4, 4]));
    }

    #[test]
    #[should_panic(expected = "Encountered NaN value for x")]
    fn ignores_panics_on_invalid_points() {
        let mut q = Quadtree::default();
        q.cover(0., 0.);
        q.cover(f64::NAN, 2.);
    }

    #[test]
    fn repeatedly_doubles_the_existing_extent_if_the_extent_was_non_trivial() {
        fn cover_multiple(q: &mut Quadtree, ps: &[[f64; 2]]) {
            for p in ps {
                q.cover(p[0], p[1]);
            }
        }

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [-1., -1.]]);
        assert_eq!(q.extent(), ([-4, -4], [4, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [1., -1.]]);
        assert_eq!(q.extent(), ([0, -4], [8, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [3., -1.]]);
        assert_eq!(q.extent(), ([0, -4], [8, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [3., 1.]]);
        assert_eq!(q.extent(), ([0, 0], [4, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [3., 3.]]);
        assert_eq!(q.extent(), ([0, 0], [4, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [1., 3.]]);
        assert_eq!(q.extent(), ([0, 0], [4, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [-1., 3.]]);
        assert_eq!(q.extent(), ([-4, 0], [4, 8]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [-1., 1.]]);
        assert_eq!(q.extent(), ([-4, 0], [4, 8]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [-3., -3.]]);
        assert_eq!(q.extent(), ([-4, -4], [4, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [3., -3.]]);
        assert_eq!(q.extent(), ([0, -4], [8, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [5., -3.]]);
        assert_eq!(q.extent(), ([0, -4], [8, 4]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [5., 3.]]);
        assert_eq!(q.extent(), ([0, 0], [8, 8]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [5., 5.]]);
        assert_eq!(q.extent(), ([0, 0], [8, 8]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [3., 5.]]);
        assert_eq!(q.extent(), ([0, 0], [8, 8]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [-3., 5.]]);
        assert_eq!(q.extent(), ([-4, 0], [4, 8]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [-3., 3.]]);
        assert_eq!(q.extent(), ([-4, 0], [4, 8]));

        let mut q = Quadtree::default();
        cover_multiple(&mut q, &[[0., 0.], [2., 2.], [-1., 1.]]);
        assert_eq!(q.extent(), ([-4, 0], [4, 8]));
    }

    #[test]
    fn repeatedly_wraps_the_root_node_if_it_has_children() {
        let mut q = Quadtree::default();
        q.insert_default(0., 0.);
        q.insert_default(2., 2.);

        let mut tmp = q.clone();
        tmp.cover(3., 3.);
        assert!(matches!(
            tmp.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 2., y: 2., .. }),
            ]
        ));

        let mut tmp = q.clone();
        tmp.cover(-1., 3.);
        assert!(matches!(
            tmp.root().unwrap().children().unwrap()[1]
                .unwrap()
                .children(),
            Some([
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 2., y: 2., .. }),
            ])
        ));

        let mut tmp = q.clone();
        tmp.cover(3., -1.);
        assert!(matches!(
            tmp.root().unwrap().children().unwrap()[2]
                .unwrap()
                .children(),
            Some([
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 2., y: 2., .. }),
            ])
        ));

        let mut tmp = q.clone();
        tmp.cover(-1., -1.);
        assert!(matches!(
            tmp.root().unwrap().children().unwrap()[3]
                .unwrap()
                .children(),
            Some([
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 2., y: 2., .. }),
            ])
        ));

        let mut tmp = q.clone();
        tmp.cover(5., 5.);
        assert!(matches!(
            tmp.root().unwrap().children().unwrap()[0]
                .unwrap()
                .children(),
            Some([
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 2., y: 2., .. }),
            ])
        ));

        let mut tmp = q.clone();
        tmp.cover(-3., 5.);
        assert!(matches!(
            tmp.root().unwrap().children().unwrap()[1]
                .unwrap()
                .children(),
            Some([
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 2., y: 2., .. }),
            ])
        ));

        let mut tmp = q.clone();
        tmp.cover(5., -3.);
        assert!(matches!(
            tmp.root().unwrap().children().unwrap()[2]
                .unwrap()
                .children(),
            Some([
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 2., y: 2., .. }),
            ])
        ));

        let mut tmp = q.clone();
        tmp.cover(-3., -3.);
        assert!(matches!(
            tmp.root().unwrap().children().unwrap()[3]
                .unwrap()
                .children(),
            Some([
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 2., y: 2., .. }),
            ])
        ));
    }

    #[test]
    fn does_not_wrap_root_node_if_it_is_a_leaf() {
        fn test_point(mut q: Quadtree, p: [f64; 2]) {
            q.cover(p[0], p[1]);
            assert!(matches!(q.root(), Some(Node::Leaf { x: 2., y: 2., .. })));
        }

        let mut q = Quadtree::default();
        q.cover(0., 0.);
        q.insert_default(2., 2.);
        assert!(matches!(q.root(), Some(Node::Leaf { x: 2., y: 2., .. })));
        test_point(q.clone(), [3., 3.]);
        test_point(q.clone(), [-1., 3.]);
        test_point(q.clone(), [3., -1.]);
        test_point(q.clone(), [-1., -1.]);
        test_point(q.clone(), [5., 5.]);
        test_point(q.clone(), [-3., 5.]);
        test_point(q.clone(), [5., -3.]);
        test_point(q.clone(), [-3., -3.]);
    }

    #[test]
    fn does_not_wrap_root_node_if_it_is_undefined() {
        fn cover_root(mut q: Quadtree, p: [f64; 2]) -> Option<Box<Node<(), ()>>> {
            q.cover(p[0], p[1]);
            q.root
        }

        let mut q = Quadtree::default();
        q.cover(0., 0.);
        q.cover(2., 2.);
        assert!(q.root().is_none());
        assert_eq!(cover_root(q.clone(), [3., 3.]), None);
        assert_eq!(cover_root(q.clone(), [-1., 3.]), None);
        assert_eq!(cover_root(q.clone(), [3., -1.]), None);
        assert_eq!(cover_root(q.clone(), [-1., -1.]), None);
        assert_eq!(cover_root(q.clone(), [5., 5.]), None);
        assert_eq!(cover_root(q.clone(), [-3., 5.]), None);
        assert_eq!(cover_root(q.clone(), [5., -3.]), None);
        assert_eq!(cover_root(q.clone(), [-3., -3.]), None);
    }

    // TODO(grtlr): We currently don't handle extents that exceed `i64::MAX`.
    #[test]
    #[ignore]
    fn does_not_crash_on_huge_values() {
        let mut q = Quadtree::default();
        q.insert_default(1e23, 0.);
    }
}

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

    #[ignore = "We currently don't handle extents that exceed `i64::MAX`."]
    #[test]
    fn does_not_crash_on_huge_values() {
        let mut q = Quadtree::default();
        q.insert_default(1e19, 0.);
        q.insert_default(-1e19, 0.);
        q.insert_default(0., 1e19);
        q.insert_default(0., -1e19);
    }

    // -----------------------------------------------------------------------
    // Integer overflow tests
    //
    // These tests document that the current implementation overflows on extreme
    // coordinate values.  They only make sense (and only compile to panicking
    // code) when debug-assertions are enabled – the default for `cargo test`.
    //
    // Overflow sites in cover.rs (line numbers are approximate):
    //
    //   L18  x1 = x0 + 1        — initial extent setup for empty tree
    //   L20  y1 = y0 + 1        — initial extent setup for empty tree
    //   L42  z *= 2             — doubling loop, z reaches 2^63
    //   L45  x1 = x0 + z       — x0 large positive, z reaches i64::MAX/2+1
    //   L46  y1 = y0 + z       — y0 large positive, z reaches i64::MAX/2+1
    //
    // The subtraction arms (x0 = x1 - z, y0 = y1 - z) cannot be made to
    // overflow before z *= 2 does through any representable f64 input, because
    // reaching below i64::MIN would require an x or y value smaller than
    // i64::MIN, which a saturating f64-to-i64 cast makes impossible.
    // -----------------------------------------------------------------------

    /// Covering `i64::MAX` on an **empty** tree overflows at `x1 = x0 + 1`
    /// (line 18): `floor(i64::MAX as f64) as i64` saturates to `i64::MAX`, so
    /// `x0 + 1 = i64::MAX + 1` overflows.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn overflow_x_plus_one_on_empty_tree() {
        let mut q = Quadtree::default();
        q.cover(i64::MAX as f64, 0.0);
    }

    /// Covering `i64::MAX` in the y-axis on an **empty** tree overflows at
    /// `y1 = y0 + 1` (line 20) for the same reason as the x-axis case above.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn overflow_y_plus_one_on_empty_tree() {
        let mut q = Quadtree::default();
        q.cover(0.0, i64::MAX as f64);
    }

    /// With a non-empty tree anchored at 0, covering `i64::MAX` in x requires
    /// ~62 doublings of `z`.  The last `z *= 2` overflows when `z` would
    /// become `2^63 > i64::MAX`.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn overflow_z_doubling_toward_positive_x() {
        let mut q = Quadtree::default();
        q.cover(0.0, 0.0);
        q.cover(i64::MAX as f64, 0.0);
    }

    /// Same as `overflow_z_doubling_toward_positive_x` but along the y axis.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn overflow_z_doubling_toward_positive_y() {
        let mut q = Quadtree::default();
        q.cover(0.0, 0.0);
        q.cover(0.0, i64::MAX as f64);
    }

    /// With a non-empty tree anchored at 0, covering `i64::MIN` in x forces
    /// ~62 left-expanding doublings.  The last `z *= 2` overflows when `z`
    /// would become `2^63 > i64::MAX`.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn overflow_z_doubling_toward_negative_x() {
        let mut q = Quadtree::default();
        q.cover(0.0, 0.0);
        q.cover(i64::MIN as f64, 0.0);
    }

    /// Same as `overflow_z_doubling_toward_negative_x` but along the y axis.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn overflow_z_doubling_toward_negative_y() {
        let mut q = Quadtree::default();
        q.cover(0.0, 0.0);
        q.cover(0.0, i64::MIN as f64);
    }

    /// When `x0 = 2^62`, `z` doubles to `2^62` without itself overflowing,
    /// but `x1 = x0 + z = 2^62 + 2^62 = 2^63` overflows at line 45 — one
    /// iteration *before* `z *= 2` would have overflowed.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn overflow_x1_equals_x0_plus_z() {
        // i64::MAX / 2 + 1 = 2^62, exactly representable as f64.
        let half_max = (i64::MAX / 2 + 1) as f64;
        let mut q = Quadtree::default();
        q.cover(half_max, 0.0); // empty-tree path: x0 = 2^62, x1 = 2^62 + 1
        q.cover(i64::MAX as f64, 0.0); // doubling loop: x1 = 2^62 + 2^62 overflows
    }

    /// Same as `overflow_x1_equals_x0_plus_z` but for `y1 = y0 + z` at
    /// line 46: when `y0 = 2^62` and `z` reaches `2^62`, the addition
    /// `y0 + z = 2^63` overflows before `z *= 2` would.
    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn overflow_y1_equals_y0_plus_z() {
        let half_max = (i64::MAX / 2 + 1) as f64;
        let mut q = Quadtree::default();
        q.cover(0.0, half_max); // empty-tree path: y0 = 2^62, y1 = 2^62 + 1
        q.cover(0.0, i64::MAX as f64); // doubling loop: y1 = 2^62 + 2^62 overflows
    }
}

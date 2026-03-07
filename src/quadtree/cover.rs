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
            x1 = x0.saturating_add(1);
            y0 = y;
            y1 = y0.saturating_add(1);
        } else {
            // Otherwise, double repeatedly to cover.
            //
            // `saturating_sub` guards against the (rare) case where a previous
            // saturation left x1 - x0 > i64::MAX (e.g. x0 < 0, x1 = i64::MAX).
            let mut z = {
                let span = x1.saturating_sub(x0);
                if span.is_positive() { span } else { 1 }
            };

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

                // Double z with saturation.  When z can no longer grow the
                // bounds cannot cover the target coordinate exactly; clamp the
                // affected border(s) to the i64 extremes and stop.  The
                // out-of-range point will sit in the border cell, which is the
                // intended "weird-but-no-crash" behaviour.
                let new_z = z.saturating_mul(2);
                if new_z == z {
                    match i {
                        0 => { x1 = i64::MAX; y1 = i64::MAX; }
                        1 => { x0 = i64::MIN; y1 = i64::MAX; }
                        2 => { x1 = i64::MAX; y0 = i64::MIN; }
                        3 => { x0 = i64::MIN; y0 = i64::MIN; }
                        _ => unreachable!(),
                    }
                    break;
                }
                z = new_z;
                match i {
                    0 => {
                        x1 = x0.saturating_add(z);
                        y1 = y0.saturating_add(z);
                    }
                    1 => {
                        x0 = x1.saturating_sub(z);
                        y1 = y0.saturating_add(z);
                    }
                    2 => {
                        x1 = x0.saturating_add(z);
                        y0 = y1.saturating_sub(z);
                    }
                    3 => {
                        x0 = x1.saturating_sub(z);
                        y0 = y1.saturating_sub(z);
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

    /// Previously ignored because the implementation panicked on integer
    /// overflow.  With saturating arithmetic the tree now completes without
    /// crashing; extreme coordinates land in the border cells.
    #[test]
    fn does_not_crash_on_huge_values() {
        let mut q = Quadtree::default();
        q.insert_default(1e19, 0.);
        q.insert_default(-1e19, 0.);
        q.insert_default(0., 1e19);
        q.insert_default(0., -1e19);
    }

    // -----------------------------------------------------------------------
    // Saturating-arithmetic boundary tests
    //
    // The cover function uses saturating_add / saturating_sub / saturating_mul
    // so that extreme i64 coordinates never panic.  The tests below verify
    // that every previously-overflowing code path now completes and that the
    // resulting extent is self-consistent (x0 <= x1, y0 <= y1).
    //
    // Overflow sites that are now saturated (line numbers approximate):
    //
    //   L18  x1 = x0.saturating_add(1)   — initial extent, x at i64::MAX
    //   L20  y1 = y0.saturating_add(1)   — initial extent, y at i64::MAX
    //   L42  z = z.saturating_mul(2)     — doubling loop; break on saturation
    //   L45  x1 = x0.saturating_add(z)  — large positive x0
    //   L46  y1 = y0.saturating_add(z)  — large positive y0
    // -----------------------------------------------------------------------

    fn extent_is_consistent(q: &Quadtree) {
        let ([x0, y0], [x1, y1]) = q.extent();
        assert!(x0 <= x1, "x0={x0} > x1={x1}");
        assert!(y0 <= y1, "y0={y0} > y1={y1}");
    }

    /// `floor(i64::MAX as f64) as i64` saturates to `i64::MAX`, so the
    /// initial `x0.saturating_add(1)` clamps to `i64::MAX` rather than
    /// panicking.
    #[test]
    fn saturating_x_plus_one_on_empty_tree() {
        let mut q = Quadtree::default();
        q.cover(i64::MAX as f64, 0.0);
        extent_is_consistent(&q);
    }

    /// Same for the y axis.
    #[test]
    fn saturating_y_plus_one_on_empty_tree() {
        let mut q = Quadtree::default();
        q.cover(0.0, i64::MAX as f64);
        extent_is_consistent(&q);
    }

    /// Covering `i64::MAX` in x from a near-zero tree saturates `z` after
    /// ~62 doublings; the loop breaks and the border cell absorbs the point.
    #[test]
    fn saturating_z_doubling_toward_positive_x() {
        let mut q = Quadtree::default();
        q.cover(0.0, 0.0);
        q.cover(i64::MAX as f64, 0.0);
        extent_is_consistent(&q);
    }

    /// Same along the y axis.
    #[test]
    fn saturating_z_doubling_toward_positive_y() {
        let mut q = Quadtree::default();
        q.cover(0.0, 0.0);
        q.cover(0.0, i64::MAX as f64);
        extent_is_consistent(&q);
    }

    /// Covering `i64::MIN` in x saturates `z` during left-expanding doublings.
    #[test]
    fn saturating_z_doubling_toward_negative_x() {
        let mut q = Quadtree::default();
        q.cover(0.0, 0.0);
        q.cover(i64::MIN as f64, 0.0);
        extent_is_consistent(&q);
    }

    /// Same along the y axis.
    #[test]
    fn saturating_z_doubling_toward_negative_y() {
        let mut q = Quadtree::default();
        q.cover(0.0, 0.0);
        q.cover(0.0, i64::MIN as f64);
        extent_is_consistent(&q);
    }

    /// With `x0 = 2^62`, `z` reaches `2^62` and `x0.saturating_add(z)` clamps
    /// to `i64::MAX` instead of overflowing.
    #[test]
    fn saturating_x1_equals_x0_plus_z() {
        // i64::MAX / 2 + 1 = 2^62, exactly representable as f64.
        let half_max = (i64::MAX / 2 + 1) as f64;
        let mut q = Quadtree::default();
        q.cover(half_max, 0.0);
        q.cover(i64::MAX as f64, 0.0);
        extent_is_consistent(&q);
    }

    /// Same for `y1 = y0.saturating_add(z)` when `y0 = 2^62`.
    #[test]
    fn saturating_y1_equals_y0_plus_z() {
        let half_max = (i64::MAX / 2 + 1) as f64;
        let mut q = Quadtree::default();
        q.cover(0.0, half_max);
        q.cover(0.0, i64::MAX as f64);
        extent_is_consistent(&q);
    }
}

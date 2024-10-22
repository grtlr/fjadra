use crate::quadtree::create_empty_internal;

use super::{create_leaf, indexer::Indexer, Node, Quadtree};

impl<Q: Default, T> Quadtree<Q, T> {
    pub fn insert(&mut self, x: f64, y: f64, value: T) {
        self.cover(x, y);

        let node = self.root.as_mut();

        let Some(mut node) = node else {
            self.root = Some(create_leaf(&mut self.store, x, y, value).into());
            return;
        };

        // TODO(grtlr): confirm correctness of this conversion w.r.t. floating-point
        // precision.
        let mut ix = Indexer::with_extent(
            [self.x0 as f64, self.y0 as f64],
            [self.x1 as f64, self.y1 as f64],
        );

        '_descend: loop {
            match node.as_mut() {
                Node::Internal {
                    ref mut children, ..
                } => {
                    let i = ix.get_and_descend(x, y);
                    if let Some(ref mut n) = children[i] {
                        node = n;
                    } else {
                        children[i] = Some(create_leaf(&mut self.store, x, y, value).into());
                        return;
                    }
                }
                // The new point coincides with the existing point.
                Node::Leaf {
                    ref mut data,
                    x: xp,
                    y: yp,
                    ..
                } if x == *xp && y == *yp => {
                    data.insert(value);
                    return;
                }
                old_leaf @ Node::Leaf { .. } => {
                    let inner = std::mem::replace(old_leaf, create_empty_internal(&mut self.store));
                    if let Node::Leaf { x: xp, y: yp, .. } = inner {
                        let mut new_internal = old_leaf;

                        loop {
                            let Node::Internal {
                                children: ref mut parent,
                                ..
                            } = new_internal
                            else {
                                unreachable!()
                            };

                            let j = ix.get(xp, yp);
                            let i = ix.get_and_descend(x, y);

                            debug_assert!(i < 4);
                            debug_assert!(j < 4);

                            if i != j {
                                parent[i] = Some(create_leaf(&mut self.store, x, y, value).into());
                                parent[j] = Some(inner.into());
                                return;
                            }

                            parent[i] = Some(create_empty_internal(&mut self.store).into());
                            new_internal = parent[i].as_mut().unwrap();
                        }
                    }
                    unreachable!()
                }
            }
        }
    }
}

impl<Q: Default, T: Default> Quadtree<Q, T> {
    pub fn insert_default(&mut self, x: f64, y: f64) {
        self.insert(x, y, T::default());
    }
}

#[cfg(test)]
mod test {
    use super::Node;

    type Quadtree = super::Quadtree<()>;

    #[test]
    fn creates_a_new_point_and_adds_it_to_the_quadtree() {
        let mut q = Quadtree::default();

        q.insert_default(0., 0.);
        assert!(matches!(
            q.root().unwrap(),
            &Node::Leaf { x: 0., y: 0., .. }
        ));

        q.insert_default(0.9, 0.9);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 0.9, y: 0.9, .. }),
            ]
        ));

        q.insert_default(0.9, 0.0);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                Some(&Node::Leaf { x: 0.9, y: 0., .. }),
                None,
                Some(&Node::Leaf { x: 0.9, y: 0.9, .. }),
            ]
        ));

        q.insert_default(0., 0.9);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                Some(&Node::Leaf { x: 0.9, y: 0., .. }),
                Some(&Node::Leaf { x: 0.0, y: 0.9, .. }),
                Some(&Node::Leaf { x: 0.9, y: 0.9, .. }),
            ]
        ));

        q.insert_default(0.4, 0.4);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Internal { .. }),
                Some(&Node::Leaf { x: 0.9, y: 0., .. }),
                Some(&Node::Leaf { x: 0.0, y: 0.9, .. }),
                Some(&Node::Leaf { x: 0.9, y: 0.9, .. }),
            ]
        ));
        assert!(matches!(
            q.root().unwrap().children().unwrap()[0].unwrap().children(),
            Some([
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 0.4, y: 0.4, .. }),
            ])
        ));
    }

    #[test]
    fn handles_points_being_on_the_perimeter_of_the_quadtree_bounds() {
        let mut q = Quadtree::with_extent([0., 0.], [1., 1.]);
        q.insert_default(0., 0.);
        assert!(matches!(q.root(), Some(&Node::Leaf { x: 0., y: 0., .. })));

        q.insert_default(1., 1.);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                None,
                None,
                Some(&Node::Leaf { x: 1., y: 1., .. }),
            ]
        ));

        q.insert_default(1., 0.);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                Some(&Node::Leaf { x: 1., y: 0., .. }),
                None,
                Some(&Node::Leaf { x: 1., y: 1., .. }),
            ]
        ));

        q.insert_default(0., 1.);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                Some(&Node::Leaf { x: 1., y: 0., .. }),
                Some(&Node::Leaf { x: 0., y: 1., .. }),
                Some(&Node::Leaf { x: 1., y: 1., .. }),
            ]
        ));
    }

    #[test]
    fn handles_points_being_to_the_left_of_quadtree_bounds() {
        let mut q = Quadtree::with_extent([0., 0.], [2., 2.]);
        q.insert_default(-1., 1.);
        assert_eq!(dbg!(q).extent(), ([-4, 0], [4, 8]));
    }

    #[test]
    fn handles_coincident_points_by_creating_linked_list() {
        let mut q = Quadtree::with_extent([0., 0.], [1., 1.]);
        q.insert_default(0., 0.);
        assert!(matches!(
            q.root().unwrap(),
            &Node::Leaf { x: 0., y: 0., .. }
        ));

        q.insert_default(1., 0.);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                Some(&Node::Leaf { x: 1., y: 0., .. }),
                None,
                None,
            ]
        ));

        q.insert_default(0., 1.);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                Some(&Node::Leaf { x: 1., y: 0., .. }),
                Some(&Node::Leaf { x: 0., y: 1., .. }),
                None,
            ]
        ));

        q.insert_default(0., 1.);
        assert!(matches!(
            q.root().unwrap().children().unwrap(),
            [
                Some(&Node::Leaf { x: 0., y: 0., .. }),
                Some(&Node::Leaf { x: 1., y: 0., .. }),
                Some(&Node::Leaf { x: 0., y: 1., .. }),
                None,
            ]
        ));
        assert_eq!(
            q.root().unwrap().children().unwrap()[2]
                .unwrap()
                .leaf()
                .unwrap()
                .iter()
                .collect::<Vec<_>>(),
            vec![&(), &()],
        )
    }

    #[test]
    fn trivial_bounds_for_first_point() {
        let mut q = Quadtree::default();
        q.insert_default(1.0, 2.0);
        assert_eq!(q.extent(), ([1, 2], [2, 3]));
        assert!(matches!(
            q.root().unwrap(),
            Node::Leaf { x: 1.0, y: 2.0, .. }
        ));
    }
}

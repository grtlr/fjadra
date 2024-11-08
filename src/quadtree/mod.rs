mod add;
mod add_all;
mod cover;
mod indexer;
mod quad;
mod store;
mod visit;
mod visit_after;

use store::{Handle, Store};

pub use quad::{Entry, Quad};
pub use visit::Visit;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeafEntry<T> {
    value: T,
    next: Option<Box<LeafEntry<T>>>,
}

impl<T> LeafEntry<T> {
    fn new(data: T) -> Self {
        Self {
            value: data,
            next: None,
        }
    }

    fn insert(&mut self, data: T) {
        let mut node = self;
        loop {
            match node.next {
                Some(ref mut next) => {
                    node = next;
                }
                None => {
                    node.next = Some(Box::new(LeafEntry::new(data)));
                    return;
                }
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        LeafListIterator { next: Some(self) }
    }
}

struct LeafListIterator<'a, T> {
    next: Option<&'a LeafEntry<T>>,
}

impl<'a, T> Iterator for LeafListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;
        self.next = next.and_then(|node| node.next.as_deref());
        next.map(|node| &node.value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node<Q, T> {
    Leaf {
        x: f64,
        y: f64,
        data: LeafEntry<T>,
        handle: Handle<Q>,
    },
    Internal {
        value: Option<T>,
        children: [Option<Box<Node<Q, T>>>; 4],
        handle: Handle<Q>,
    },
}

impl<Q, T> Node<Q, T> {
    #[cfg(test)]
    fn leaf(&self) -> Option<&LeafEntry<T>> {
        match self {
            Node::Leaf { data, .. } => Some(data),
            _ => None,
        }
    }

    #[cfg(test)]
    fn children(&self) -> Option<[Option<&Node<Q, T>>; 4]> {
        match self {
            Node::Leaf { .. } => None,
            Node::Internal { children, .. } => Some([
                children[0].as_deref(),
                children[1].as_deref(),
                children[2].as_deref(),
                children[3].as_deref(),
            ]),
        }
    }

    fn handle(&self) -> Handle<Q> {
        match self {
            Node::Leaf { handle, .. } => *handle,
            Node::Internal { handle, .. } => *handle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Quadtree<Q: Default = (), T = ()> {
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
    root: Option<Box<Node<Q, T>>>,
    store: Store<Q>,
}

impl<Q: Default, T> Default for Quadtree<Q, T> {
    fn default() -> Self {
        Self {
            x0: i64::MAX,
            y0: 0,
            x1: i64::MIN,
            y1: 0,
            root: None,
            store: Store::new(),
        }
    }
}

impl<Q: Default, T> Quadtree<Q, T> {
    pub fn from_particles(particles: impl Iterator<Item = (f64, f64, T)>) -> Self {
        let mut tree = Self::default();
        for (x, y, value) in particles {
            tree.insert(x, y, value);
        }
        tree
    }

    pub fn with_extent(min: [f64; 2], max: [f64; 2]) -> Self {
        let mut tree = Self::default();
        tree.cover(min[0], min[1]);
        tree.cover(max[0], max[1]);
        tree
    }

    pub fn extent(&self) -> ([i64; 2], [i64; 2]) {
        ([self.x0, self.y0], [self.x1, self.y1])
    }

    fn root(&self) -> Option<&Node<Q, T>> {
        self.root.as_ref().map(|node| &**node)
    }
}

pub(crate) fn create_empty_internal<Q: Default, T>(store: &mut Store<Q>) -> Node<Q, T> {
    Node::Internal {
        children: [None, None, None, None],
        value: None,
        handle: store.insert(Default::default()),
    }
}

pub(crate) fn create_leaf<Q: Default, T>(
    store: &mut Store<Q>,
    x: f64,
    y: f64,
    value: T,
) -> Node<Q, T> {
    Node::Leaf {
        x,
        y,
        data: LeafEntry::new(value),
        handle: store.insert(Default::default()),
    }
}

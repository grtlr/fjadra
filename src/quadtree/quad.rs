use crate::extent::Extent;

use super::{store::Store, Node};

pub struct QuadInner<'a, Q, T> {
    pub extent: Extent,
    pub node: &'a Node<Q, T>,
}

impl<'a, Q, T> QuadInner<'a, Q, T> {
    pub fn new(node: &'a Node<Q, T>, x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self {
            extent: [x0, y0, x1, y1].into(),
            node,
        }
    }
}

pub enum Entry<'a, T, Q> {
    Internal {
        children: [Option<&'a Q>; 4],
    },
    Leaf {
        x: f64,
        y: f64,
        data: &'a T,
        others: Option<Vec<&'a T>>,
    },
}

pub struct Quad<'a, Q, T>(&'a mut Store<Q>, &'a QuadInner<'a, Q, T>);

impl<'a, Q, T> Quad<'a, Q, T> {
    pub fn from_quad(store: &'a mut Store<Q>, quad: &'a QuadInner<'a, Q, T>) -> Self {
        Self(store, quad)
    }

    pub fn value(&self) -> &Q {
        self.0.get(self.1.node.handle())
    }

    pub fn value_mut(&mut self) -> &mut Q {
        self.0.get_mut(self.1.node.handle())
    }

    pub fn inner(&'a self) -> Entry<'a, T, Q> {
        match self.1.node {
            Node::Internal { children, .. } => {
                let c = [
                    children[0].as_ref().map(|h| self.0.get(h.handle())),
                    children[1].as_ref().map(|h| self.0.get(h.handle())),
                    children[2].as_ref().map(|h| self.0.get(h.handle())),
                    children[3].as_ref().map(|h| self.0.get(h.handle())),
                ];
                Entry::Internal { children: c }
            }
            Node::Leaf { data, x, y, .. } => Entry::Leaf {
                x: *x,
                y: *y,
                data: &data.value,
                others: data.next.as_ref().map(|n| n.iter().collect()),
            },
        }
    }

    pub fn extent(&self) -> Extent {
        self.1.extent
    }
}

use super::Quadtree;

impl<Q: Default, T> Quadtree<Q, T> {
    pub fn add_all(&mut self, values: impl IntoIterator<Item = (f64, f64, T)>) {
        // TODO(grtlr): Improve the performance by computing the extent only
        // once. Similar to how it's done in d3.
        for value in values {
            self.insert(value.0, value.1, value.2);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Handle<T>(usize, std::marker::PhantomData<T>);

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Handle<T> {}

// For now we only support inserting data to the tree, so we can get away with a
// simple `Vec`-based store.
#[derive(Clone)]
pub struct Store<T>(Vec<T>);

impl<T> Store<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(&mut self, value: T) -> Handle<T> {
        let handle = Handle(self.0.len(), std::marker::PhantomData);
        self.0.push(value);
        handle
    }

    pub fn get(&self, handle: Handle<T>) -> &T {
        &self.0[handle.0]
    }

    pub fn get_mut(&mut self, handle: Handle<T>) -> &mut T {
        &mut self.0[handle.0]
    }

    pub fn iter(&self) -> impl Iterator<Item = (Handle<T>, &T)> {
        self.0
            .iter()
            .enumerate()
            .map(|(i, v)| (Handle(i, std::marker::PhantomData), v))
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Store<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vs = self.iter().collect::<Vec<_>>();
        write!(f, "Store({:?})", vs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut store = Store::new();
        let handle = store.insert(42);
        assert_eq!(*store.get(handle), 42);
    }

    #[test]
    fn insert_and_get_mut() {
        let mut store = Store::new();
        let handle = store.insert(42);
        *store.get_mut(handle) = 43;
        assert_eq!(*store.get(handle), 43);
    }
}

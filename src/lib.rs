use std::ops::{Deref, DerefMut};

/// Cycles over a vector as if it were an iterator. That's it.
#[derive(Clone)]
pub struct VecCycle<T: Clone> {
    index: usize,
    inner: Vec<T>,
}

impl<T: Clone> Iterator for VecCycle<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.len() == 0 {
            return None;
        }

        self.index %= self.inner.len();

        let value = self.inner.get(self.index).map(|x| x.clone());

        self.index = (self.index + 1) % self.inner.len();

        value
    }
}

impl<T: Clone> VecCycle<T> {
    /// Creates a `VecCycle` from a `Vec`
    pub fn new(vector: Vec<T>) -> Self {
        Self::from(vector)
    }

    /// Returns the index. Panics if the vector is empty.
    pub fn index(&self) -> usize {
        assert!(self.inner.len() != 0, "The contained vector is empty!");
        self.index % self.inner.len()
    }

    /// Sets the index. Panics if the index is out of range.
    pub fn set_index(&mut self, new_index: usize) {
        assert!(
            new_index < self.inner.len(),
            "The given index is too large!"
        );
        self.index = new_index;
    }

    /// Resets the index, making the cycle restart from index zero.
    pub fn restart(&mut self) {
        self.index = 0;
    }
}

impl<T: Clone> Deref for VecCycle<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Clone> DerefMut for VecCycle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Clone> From<Vec<T>> for VecCycle<T> {
    fn from(vector: Vec<T>) -> Self {
        Self {
            index: 0,
            inner: vector,
        }
    }
}

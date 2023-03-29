/// Cycles over a vector as if it were an iterator. That's it.
#[derive(Clone)]
pub struct VecCycle<T: Clone> {
    index: usize,
    inner: Vec<T>,
}

impl<T: Clone> Iterator for VecCycle<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.inner.get(self.index).map(|x| x.clone());
        self.index = (self.index + 1) % self.inner.len();
        value
    }
}

impl<T: Clone> VecCycle<T> {
    pub fn new(vector: Vec<T>) -> Self {
        Self {
            index: 0,
            inner: vector,
        }
    }

    pub fn inner(&self) -> &Vec<T> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Vec<T> {
        &mut self.inner
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn set_index(&mut self, new_index: usize) {
        assert!(new_index < self.inner.len(), "The given index is too large!");
        self.index = new_index;
    }

    /// Resets the index, making the cycle restart from index zero.
    pub fn restart(&mut self) {
        self.index = 0;
    }
}

use std::collections::VecDeque;

pub struct BoundedQueue<T> {
    inner: VecDeque<T>,
    capacity: usize,
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.inner.len() >= self.capacity {
            let to_remove_idx = self.inner.len() + 1 - self.capacity;
            self.inner.drain(0..to_remove_idx);
        }
        self.inner.push_back(item);
    }

    pub fn maybe_shrink(&mut self) {
        if self.inner.is_empty() && self.inner.capacity() > self.capacity {
            self.inner = VecDeque::with_capacity(self.capacity);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }
}

use color_eyre::eyre::{Result, eyre};
use std::slice::Iter;

pub struct Cursor<T> {
    vec: Vec<T>,
    idx: usize,
}

impl<T> Cursor<T> {
    pub fn new(vec: Vec<T>) -> Result<Self> {
        if vec.is_empty() {
            return Err(eyre!("Empty vec not allowed"));
        }
        Ok(Self { vec, idx: 0 })
    }

    pub fn current(&self) -> &T {
        &self.vec[self.idx]
    }

    pub fn index(&self) -> usize {
        self.idx
    }

    pub fn next(&mut self) -> &T {
        self.idx = (self.idx + 1) % self.vec.len();
        self.current()
    }

    pub fn next_back(&mut self) -> &T {
        let len = self.vec.len();
        self.idx = (len + self.idx - 1) % len;
        self.current()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }
}

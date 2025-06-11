use crate::shared::GetterOpt;
use std::slice::Iter;

pub struct Cursor<T> {
    vec: Vec<T>,
    idx: usize,
}

impl<T> Cursor<T> {
    pub fn new(vec: Vec<T>) -> Self {
        if vec.is_empty() {
            panic!("Empty vec provided");
        }
        Self { vec, idx: 0 }
    }

    pub fn current(&self) -> Option<&T> {
        self.vec.get(self.idx)
    }

    pub fn index(&self) -> usize {
        self.idx
    }

    pub fn next(&mut self) {
        self.idx = (self.idx + 1) % self.vec.len();
    }

    pub fn next_back(&mut self) {
        let len = self.vec.len();
        self.idx = (len + self.idx - 1) % len;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }
}

impl<T> GetterOpt<T> for Cursor<T> {
    fn get(&self) -> Option<&T> {
        self.current()
    }
}

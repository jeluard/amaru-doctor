use crate::model::buffer_list::BufferList;
use std::fmt;
use tracing::debug;

/// A pure data model that provides a persistent, growing buffer over a lazy
/// iterator.
pub struct StreamingIter<T> {
    iter: Box<dyn Iterator<Item = T>>,
    buffer: Vec<T>,
    exhausted: bool,
}

impl<T> fmt::Debug for StreamingIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamingIter")
            .field("buffer len", &self.buffer.len())
            .field("exhausted", &self.exhausted)
            .finish()
    }
}

impl<T> StreamingIter<T> {
    pub fn new(iter: impl Iterator<Item = T> + 'static, initial_buffer_size: usize) -> Self {
        let mut s = Self {
            iter: Box::new(iter),
            buffer: Vec::with_capacity(initial_buffer_size),
            exhausted: false,
        };
        s.load_up_to(initial_buffer_size.saturating_sub(1));
        s
    }
}

impl<T> BufferList<T> for StreamingIter<T> {
    fn load_up_to(&mut self, index: usize) {
        debug!("Will load up to {}: {:?}", index, self);
        if self.exhausted || index < self.buffer.len() {
            return;
        }

        // Extend the buffer until it's large enough.
        while self.buffer.len() <= index {
            if let Some(item) = self.iter.next() {
                self.buffer.push(item);
            } else {
                self.exhausted = true;
                break;
            }
        }
        debug!("Did load up to {}: {:?}", index, self);
    }

    fn buffer(&self) -> &[T] {
        &self.buffer
    }

    fn total_len(&self) -> Option<usize> {
        if self.exhausted {
            Some(self.buffer.len())
        } else {
            None
        }
    }
}

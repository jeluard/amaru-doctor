use std::cell::{Ref, RefCell, RefMut};

pub struct WindowIter<'a, I> {
    iter: RefCell<Box<dyn Iterator<Item = I> + 'a>>,
    buffer: RefCell<Vec<I>>,
    window_start: usize,
    window_size: usize,
}

impl<'a, I> WindowIter<'a, I> {
    pub fn new<T: Iterator<Item = I> + 'a>(iter: T, window_size: usize) -> Self {
        Self {
            iter: RefCell::new(Box::new(iter)),
            buffer: RefCell::new(Vec::new()),
            window_start: 0,
            window_size,
        }
    }

    pub fn set_window_size(&mut self, new_size: usize) {
        self.window_size = new_size;
        self.fill_buffer();
    }

    pub fn view(&self) -> Ref<[I]> {
        self.fill_buffer();
        let start = self.window_start;
        let end = (start + self.window_size).min(self.buffer.borrow().len());
        Ref::map(self.buffer.borrow(), |buf| &buf[start..end])
    }

    pub fn view_mut(&self) -> RefMut<[I]> {
        self.fill_buffer();
        let start = self.window_start;
        let end = (start + self.window_size).min(self.buffer.borrow().len());
        RefMut::map(self.buffer.borrow_mut(), |buf| &mut buf[start..end])
    }

    pub fn shift_up(&mut self) {
        if self.window_start > 0 {
            self.window_start -= 1;
        }
    }

    pub fn shift_down(&mut self) {
        self.fill_buffer();
        let buffer_len = self.buffer.borrow().len();
        if self.window_start + self.window_size < buffer_len {
            self.window_start += 1;
        } else if let Some(item) = self.iter.borrow_mut().next() {
            self.buffer.borrow_mut().push(item);
            self.window_start += 1;
        }
    }

    fn fill_buffer(&self) {
        let mut buffer = self.buffer.borrow_mut();
        let mut iter = self.iter.borrow_mut();
        while buffer.len() < self.window_start + self.window_size {
            if let Some(item) = iter.next() {
                buffer.push(item);
            } else {
                break;
            }
        }
    }
}

use std::cell::{Cell, RefCell};
use std::{cell::Ref, rc::Rc};

pub trait WindowSource<T> {
    fn view(&self, start: usize, size: usize) -> Ref<[T]>;
    fn len(&self) -> usize;
}

pub struct WindowState<T> {
    source: Rc<dyn WindowSource<T>>,
    cursor: usize,
    window_start: usize,
    window_size: usize,
}

impl<T> WindowState<T> {
    pub fn new(source: Rc<dyn WindowSource<T>>, window_size: usize) -> Self {
        let mut s = Self {
            source,
            cursor: 0,
            window_start: 0,
            window_size,
        };
        s.clamp_all();
        s
    }

    pub fn set_window_size(&mut self, new_size: usize) {
        self.window_size = new_size;
        self.clamp_all();
    }

    pub fn scroll_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            if self.cursor < self.window_start {
                self.window_start = self.cursor;
            }
        }
    }

    pub fn scroll_down(&mut self) {
        let len = self.source.len();
        if self.cursor + 1 < len {
            self.cursor += 1;
            if self.cursor >= self.window_start + self.window_size {
                self.window_start = self.cursor + 1 - self.window_size;
            }
        }
    }

    fn clamp_all(&mut self) {
        let len = self.source.len();
        if len == 0 {
            self.cursor = 0;
        } else if self.cursor >= len {
            self.cursor = len - 1;
        }
        if self.window_size >= len {
            self.window_start = 0;
        } else if self.window_start + self.window_size > len {
            self.window_start = len - self.window_size;
        }
        if self.cursor < self.window_start {
            self.window_start = self.cursor;
        } else if self.cursor >= self.window_start + self.window_size {
            self.window_start = self.cursor + 1 - self.window_size;
        }
    }

    pub fn window_with_selected_index(&self) -> (Ref<[T]>, usize) {
        let view = self.source.view(self.window_start, self.window_size);
        let idx = (self.cursor - self.window_start).min(view.len().saturating_sub(1));
        (view, idx)
    }

    pub fn selected_item(&self) -> Option<Ref<T>> {
        let (view, idx) = self.window_with_selected_index();
        if view.is_empty() {
            None
        } else {
            Some(Ref::map(view, move |v| &v[idx]))
        }
    }
}
pub struct IteratorSource<T, I> {
    iter: RefCell<I>,
    buffer: RefCell<Vec<T>>,
    exhausted: Cell<bool>,
}

impl<T, I> IteratorSource<T, I>
where
    I: Iterator<Item = T>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: RefCell::new(iter),
            buffer: RefCell::new(Vec::new()),
            exhausted: Cell::new(false),
        }
    }

    fn fill_buffer(&self, up_to: usize) {
        if self.exhausted.get() {
            return;
        }
        let mut buf = self.buffer.borrow_mut();
        let mut it = self.iter.borrow_mut();
        while buf.len() < up_to {
            if let Some(item) = it.next() {
                buf.push(item);
            } else {
                self.exhausted.set(true);
                break;
            }
        }
    }
}

impl<T, I> WindowSource<T> for IteratorSource<T, I>
where
    I: Iterator<Item = T>,
{
    fn view(&self, start: usize, size: usize) -> Ref<[T]> {
        self.fill_buffer(start + size);
        let end = (start + size).min(self.buffer.borrow().len());
        Ref::map(self.buffer.borrow(), move |b| &b[start..end])
    }

    fn len(&self) -> usize {
        if self.exhausted.get() {
            self.buffer.borrow().len()
        } else {
            usize::MAX
        }
    }
}

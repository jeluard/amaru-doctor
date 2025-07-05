use crate::update::scroll::Scrollable;
use tracing::trace;

pub struct WindowState<T> {
    iter: Box<dyn Iterator<Item = T>>,
    buffer: Vec<T>,
    exhausted: bool,
    cursor: usize,
    window_start: usize,
    window_size: usize,
}

impl<T> WindowState<T> {
    pub fn from_box(iter: Box<dyn Iterator<Item = T>>) -> Self {
        let mut s = Self {
            iter,
            buffer: Vec::new(),
            exhausted: false,
            cursor: 0,
            window_start: 0,
            window_size: 1,
        };
        s.fill_buffer(s.window_size);
        s
    }

    pub fn from_iter<I: Iterator<Item = T> + 'static>(iter: I) -> Self {
        WindowState::from_box(Box::new(iter))
    }

    fn fill_buffer(&mut self, up_to: usize) {
        if self.exhausted {
            return;
        }
        while self.buffer.len() < up_to {
            match self.iter.next() {
                Some(item) => self.buffer.push(item),
                None => {
                    self.exhausted = true;
                    break;
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        if self.exhausted {
            self.buffer.len()
        } else {
            usize::MAX
        }
    }

    pub fn set_window_size(&mut self, new_size: usize) {
        trace!("Setting window size to {}", new_size);
        self.window_size = new_size;
        self.clamp_all();
        self.fill_buffer(self.window_start + self.window_size);
    }

    fn clamp_all(&mut self) {
        let len = self.len();
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

    pub fn window_view(&self) -> (&[T], Option<usize>) {
        let end = (self.window_start + self.window_size).min(self.buffer.len());
        let slice = &self.buffer[self.window_start..end];
        let idx = if slice.is_empty() {
            None
        } else {
            Some(
                self.cursor
                    .saturating_sub(self.window_start)
                    .min(slice.len().saturating_sub(1)),
            )
        };
        (slice, idx)
    }

    pub fn selected(&self) -> Option<&T> {
        let (view, idx) = self.window_view();
        idx.and_then(|i| view.get(i))
    }
}

impl<T> Scrollable for WindowState<T> {
    fn scroll_up(&mut self) {
        trace!("Scrolling up");
        if self.cursor > 0 {
            self.cursor -= 1;
            if self.cursor < self.window_start {
                self.window_start = self.cursor;
            }
        }
    }

    fn scroll_down(&mut self) {
        trace!("Scrolling down");
        let len = self.len();
        if self.cursor + 1 < len {
            self.cursor += 1;
            if self.cursor >= self.window_start + self.window_size {
                self.window_start = self.cursor + 1 - self.window_size;
            }
            self.fill_buffer(self.window_start + self.window_size);
        }
    }
}

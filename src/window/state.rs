use std::iter::Iterator;

use crate::window::iter::WindowIter;

pub struct WindowState<T, I>
where
    I: Iterator<Item = T>,
{
    window: WindowIter<T, I>,
    selected: usize,
}

impl<T, I> WindowState<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    pub fn new(iter: I, window_size: usize) -> Self {
        Self {
            window: WindowIter::new(iter, window_size),
            selected: 0,
        }
    }

    pub fn set_window_size(&mut self, new_size: usize) {
        self.window.set_window_size(new_size);
        self.selected = self
            .selected
            .min(self.window.view().len().saturating_sub(1));
    }

    pub fn window_with_selected_index(&mut self) -> (&[T], usize) {
        let view = self.window.view();
        (view, self.selected)
    }

    pub fn selected_item(&mut self) -> Option<&T> {
        self.window.view().get(self.selected)
    }

    pub fn scroll_up(&mut self) {
        if self.selected == 0 {
            self.window.shift_up();
        } else {
            self.selected -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        let len = self.window.view().len();
        if self.selected + 1 < len {
            self.selected += 1;
        } else {
            self.window.shift_down();
            let new_len = self.window.view().len();
            if new_len > len {
                self.selected += 1;
            }
        }
    }
}

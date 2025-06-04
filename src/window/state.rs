use crate::window::iter::WindowIter;
use std::cell::{Ref, RefMut};

pub struct WindowState<'a, I> {
    window: WindowIter<'a, I>,
    selected: usize,
}

impl<'a, I> WindowState<'a, I> {
    pub fn new<T: Iterator<Item = I> + 'a>(iter: T, window_size: usize) -> Self {
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

    pub fn window_with_selected_index(&self) -> (Ref<[I]>, usize) {
        let view = self.window.view();
        (view, self.selected)
    }

    pub fn selected_item(&self) -> Option<Ref<I>> {
        let view = self.window.view();
        if self.selected < view.len() {
            Some(Ref::map(view, |v| &v[self.selected]))
        } else {
            None
        }
    }

    pub fn selected_item_mut(&self) -> Option<RefMut<I>> {
        let view = self.window.view_mut();
        if self.selected < view.len() {
            Some(RefMut::map(view, |v| &mut v[self.selected]))
        } else {
            None
        }
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

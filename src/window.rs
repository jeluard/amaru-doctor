use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};
pub trait WindowSource<T> {
    fn view(&self, start: usize, size: usize) -> Ref<[T]>;
    fn len(&self) -> usize;
}

pub struct VecSource<T> {
    pub data: RefCell<Vec<T>>,
}

impl<T> WindowSource<T> for VecSource<T> {
    fn view(&self, start: usize, size: usize) -> Ref<[T]> {
        Ref::map(self.data.borrow(), move |v| {
            let end = (start + size).min(v.len());
            &v[start..end]
        })
    }

    fn len(&self) -> usize {
        self.data.borrow().len()
    }
}

impl<T> WindowSource<T> for RefCell<Vec<T>> {
    fn view(&self, start: usize, size: usize) -> Ref<[T]> {
        Ref::map(self.borrow(), move |v| {
            let end = (start + size).min(v.len());
            &v[start..end]
        })
    }

    fn len(&self) -> usize {
        self.borrow().len()
    }
}
pub struct WindowState<'a, T> {
    start: usize,
    size: usize,
    selected: usize,
    source: Rc<dyn WindowSource<T> + 'a>,
}

impl<'a, T> WindowState<'a, T> {
    pub fn new(source: Rc<dyn WindowSource<T> + 'a>, size: usize) -> Self {
        Self {
            start: 0,
            size,
            selected: 0,
            source,
        }
    }

    pub fn set_window_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn window_with_selected_index(&self) -> (Ref<[T]>, usize) {
        let view = self.source.view(self.start, self.size);
        let index = self.selected.min(view.len().saturating_sub(1));
        (view, index)
    }

    pub fn selected_item(&self) -> Option<Ref<T>> {
        let view = self.source.view(self.start, self.size);
        if self.selected < view.len() {
            Some(Ref::map(view, |v| &v[self.selected]))
        } else {
            None
        }
    }

    pub fn scroll_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else if self.start > 0 {
            self.start -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        if self.selected + 1 < self.source.view(self.start, self.size).len() {
            self.selected += 1;
        } else if self.start + self.size < self.source.len() {
            self.start += 1;
        }
    }
}

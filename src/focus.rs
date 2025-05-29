use crate::shared::Shared;

pub trait Focusable {
    fn set_focus(&mut self, _: bool) {}
    fn has_focus(&self) -> bool {
        false
    }
}

#[derive(Default)]
pub struct FocusManager<'a> {
    index: usize,
    components: Vec<Shared<'a, dyn Focusable + 'a>>,
}

impl<'a> FocusManager<'a> {
    pub fn new(components: Vec<Shared<'a, dyn Focusable + 'a>>) -> Self {
        if !components.is_empty() {
            components[0].borrow_mut().set_focus(true);
        }
        Self {
            index: 0,
            components,
        }
    }

    pub fn shift_prev(&mut self) {
        self.components[self.index].borrow_mut().set_focus(false);
        self.index = (self.index + self.components.len() - 1) % self.components.len();
        self.components[self.index].borrow_mut().set_focus(true);
    }

    pub fn shift_next(&mut self) {
        self.components[self.index].borrow_mut().set_focus(false);
        self.index = (self.index + 1) % self.components.len();
        self.components[self.index].borrow_mut().set_focus(true);
    }
}

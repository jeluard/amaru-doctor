use crate::shared::Shared;

pub trait Focusable {
    /// Set the component's focus.
    fn set_focus(&mut self, _: bool) {}
    /// Query the component's focus.
    fn has_focus(&self) -> bool {
        false
    }
}

pub type FocusableRef<'a> = Shared<'a, dyn Focusable + 'a>;

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

    fn focus_next(&mut self) {
        self.components[self.index].borrow_mut().set_focus(false);
        self.index = (self.index + 1) % self.components.len();
        self.components[self.index].borrow_mut().set_focus(true);
    }

    fn focus_prev(&mut self) {
        self.components[self.index].borrow_mut().set_focus(false);
        self.index = (self.index + self.components.len() - 1) % self.components.len();
        self.components[self.index].borrow_mut().set_focus(true);
    }
}

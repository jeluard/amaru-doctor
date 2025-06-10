use crate::{components::Component, shared::Shared};
use tracing::trace;

#[derive(Default)]
pub struct FocusState {
    has_focus: bool,
}

impl FocusState {
    pub fn set(&mut self, b: bool) {
        self.has_focus = b;
    }

    pub fn get(&self) -> bool {
        self.has_focus
    }
}

pub trait FocusableComponent: Component {
    fn focus_state(&self) -> &FocusState;
    fn focus_state_mut(&mut self) -> &mut FocusState;

    fn set_focus(&mut self, b: bool) {
        trace!("{}: set focus to {}", self.debug_name(), b);
        self.focus_state_mut().set(b);
    }

    fn has_focus(&self) -> bool {
        self.focus_state().get()
    }
}

#[derive(Default)]
pub struct FocusManager {
    index: usize,
    components: Vec<Shared<dyn FocusableComponent>>,
}

impl FocusManager {
    pub fn new(components: Vec<Shared<dyn FocusableComponent>>) -> Self {
        if !components.is_empty() {
            components[0].borrow_mut().set_focus(true);
        }
        Self {
            index: 0,
            components,
        }
    }

    pub fn shift_prev(&mut self) {
        trace!("FocusManager:: Will shift focus prev");
        self.components[self.index].borrow_mut().set_focus(false);
        self.index = (self.index + self.components.len() - 1) % self.components.len();
        self.components[self.index].borrow_mut().set_focus(true);
    }

    pub fn shift_next(&mut self) {
        trace!("FocusManager:: Will shift focus next");
        self.components[self.index].borrow_mut().set_focus(false);
        self.index = (self.index + 1) % self.components.len();
        self.components[self.index].borrow_mut().set_focus(true);
    }
}

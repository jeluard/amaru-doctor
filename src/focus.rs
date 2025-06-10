use crate::{app_state::AppState, components::Component, shared::Shared};
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

pub struct FocusManager<'a> {
    index: usize,
    app_state: &'a AppState,
}

impl<'a> FocusManager<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        Self {
            index: 0,
            app_state,
        }
    }

    pub fn shift_prev(&mut self) {
        // TODO: fix
        // trace!("FocusManager:: Will shift focus prev");
        // self.components[self.index].borrow_mut().set_focus(false);
        // self.index = (self.index + self.components.len() - 1) % self.components.len();
        // self.components[self.index].borrow_mut().set_focus(true);
    }

    pub fn shift_next(&mut self) {
        // TODO: fix
        // trace!("FocusManager:: Will shift focus next");
        // self.components[self.index].borrow_mut().set_focus(false);
        // self.index = (self.index + 1) % self.components.len();
        // self.components[self.index].borrow_mut().set_focus(true);
    }
}

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

// pub trait FocusableComponent: Component {
//     // fn focus_state(&self) -> &FocusState;
//     // fn focus_state_mut(&mut self) -> &mut FocusState;

//     // fn set_focus(&mut self, b: bool) {
//     //     trace!("{}: set focus to {}", self.debug_name(), b);
//     //     self.focus_state_mut().set(b);
//     // }

//     fn has_focus(&self) -> bool;
// }

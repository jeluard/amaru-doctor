use crate::{app_state::AppState, states::Action, update::Update};
use tracing::trace;

pub struct FocusUpdate {}

impl Update for FocusUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action> {
        match action {
            Action::FocusPrev => {
                trace!(
                    "Will shift focus back from {}",
                    app_state.slot_focus.current()
                );
                let new_focus = app_state.slot_focus.next_back();
                trace!("Did shift focus back to {:?}", new_focus);
            }
            Action::FocusNext => {
                trace!(
                    "Will shift focus forward from {}",
                    app_state.slot_focus.current()
                );
                let new_focus = app_state.slot_focus.next();
                trace!("Did shift focus forward to {:?}", new_focus);
            }
            _ => {}
        }
        None
    }
}

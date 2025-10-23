use crate::{app_state::AppState, states::Action, update::Update};
use crossterm::event::MouseEventKind;

pub struct MouseFocusUpdate;

impl Update for MouseFocusUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::MouseEvent(mouse_event) = action else {
            return Vec::new();
        };

        if mouse_event.kind != MouseEventKind::Moved {
            // We only care about mouse move events
            return Vec::new();
        }
        s.layout_model
            .set_focus_by_location(mouse_event.column, mouse_event.row);
        Vec::new()
    }
}

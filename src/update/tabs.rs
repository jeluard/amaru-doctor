use crate::{
    app_state::AppState,
    states::{Action, WidgetSlot},
    update::Update,
};
use crossterm::event::KeyCode;
use strum::Display;
use tracing::trace;

#[derive(Display, Debug, Clone, Copy)]
enum ScrollDirection {
    Left,
    Right,
}

pub struct TabsUpdate;
impl Update for TabsUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Some(direction) = (match action {
            Action::Key(KeyCode::Left) => Some(ScrollDirection::Left),
            Action::Key(KeyCode::Right) => Some(ScrollDirection::Right),
            _ => None,
        }) else {
            return Vec::new();
        };

        match s.layout_model.get_focus() {
            WidgetSlot::InspectOption => {
                match direction {
                    ScrollDirection::Left => s.get_inspect_tabs_mut().cursor.next_back(),
                    ScrollDirection::Right => s.get_inspect_tabs_mut().cursor.non_empty_next(),
                };
                return vec![Action::UpdateLayout(s.frame_area)];
            }
            WidgetSlot::LedgerMode => {
                match direction {
                    ScrollDirection::Left => s.ledger_tabs.cursor.next_back(),
                    ScrollDirection::Right => s.ledger_tabs.cursor.non_empty_next(),
                };
                return vec![Action::UpdateLayout(s.frame_area)];
            }

            _ => trace!("No scroll logic for slot {:?}", s.layout_model.get_focus()),
        }
        Vec::new()
    }
}

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

        let slot = s
            .component_id_to_widget_slot(s.layout_model.get_focus())
            .unwrap_or_else(|| {
                panic!(
                    "No widget slot for component id {}",
                    s.layout_model.get_focus()
                )
            });
        match slot {
            WidgetSlot::InspectOption => {
                match direction {
                    ScrollDirection::Left => s.get_inspect_tabs_mut().cursor.next_back(),
                    ScrollDirection::Right => s.get_inspect_tabs_mut().cursor.non_empty_next(),
                };
                return vec![Action::UpdateLayout(s.frame_area)];
            }
            WidgetSlot::LedgerMode => {
                match direction {
                    ScrollDirection::Left => s.get_ledger_mode_tabs_mut().cursor.next_back(),
                    ScrollDirection::Right => s.get_ledger_mode_tabs_mut().cursor.non_empty_next(),
                };
                return vec![Action::UpdateLayout(s.frame_area)];
            }

            _ => trace!("No scroll logic for slot {:?}", s.layout_model.get_focus()),
        }
        Vec::new()
    }
}

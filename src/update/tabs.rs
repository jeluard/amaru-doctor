use crate::{
    app_state::AppState,
    states::{Action, ComponentId},
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

        let focus = s.layout_model.get_focus();

        match focus {
            ComponentId::InspectTabs => {
                match direction {
                    ScrollDirection::Left => s.get_inspect_tabs_mut().cursor.next_back(),
                    ScrollDirection::Right => s.get_inspect_tabs_mut().cursor.non_empty_next(),
                };
                vec![Action::UpdateLayout(s.frame_area)]
            }
            ComponentId::LedgerModeTabs => {
                match direction {
                    ScrollDirection::Left => s.get_ledger_mode_tabs_mut().cursor.next_back(),
                    ScrollDirection::Right => s.get_ledger_mode_tabs_mut().cursor.non_empty_next(),
                };
                vec![Action::UpdateLayout(s.frame_area)]
            }
            _ => {
                trace!("No tab scroll logic for component {:?}", focus);
                Vec::new()
            }
        }
    }
}

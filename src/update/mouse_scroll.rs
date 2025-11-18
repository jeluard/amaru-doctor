use crate::{
    app_state::AppState,
    states::{Action, WidgetSlot},
    update::Update,
};
use crossterm::event::MouseEventKind;
use tracing::debug;

pub struct MouseScrollUpdate;
impl Update for MouseScrollUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::MouseEvent(mouse_event) = action else {
            return Vec::new();
        };

        if mouse_event.kind != MouseEventKind::ScrollUp
            && mouse_event.kind != MouseEventKind::ScrollDown
        {
            // We only care about scrolling
            return Vec::new();
        }

        let Some((component_id, rect)) = s
            .layout_model
            .find_hovered_slot(mouse_event.column, mouse_event.row)
        else {
            debug!("Couldn't find slot for mouse event {:?}", mouse_event);
            return Vec::new();
        };
        debug!(
            "Found slot {} and rect {} for mouse event {:?}",
            component_id, rect, mouse_event
        );

        let slot = s
            .component_id_to_widget_slot(component_id)
            .unwrap_or_else(|| panic!("No widget slot for component id {}", component_id));
        match (slot, mouse_event.kind) {
            (WidgetSlot::LedgerOptions, MouseEventKind::ScrollUp) => vec![Action::ScrollUp],
            (WidgetSlot::LedgerOptions, MouseEventKind::ScrollDown) => vec![Action::ScrollDown],
            (WidgetSlot::List, MouseEventKind::ScrollUp) => vec![Action::ScrollUp],
            (WidgetSlot::List, MouseEventKind::ScrollDown) => vec![Action::ScrollDown],
            _ => Vec::new(),
        }
    }
}

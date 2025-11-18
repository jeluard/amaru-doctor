use crate::{
    app_state::AppState,
    states::{Action, InspectOption, WidgetSlot},
    update::Update,
};
use ratatui::crossterm::event::KeyCode;

/// The Update fn for selecting the focused span.
pub struct SelectSpanUpdate;
impl Update for SelectSpanUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        if !matches!(action, Action::Key(KeyCode::Enter)) {
            return Vec::new();
        }

        let slot = s
            .component_id_to_widget_slot(s.layout_model.get_focus())
            .unwrap_or_else(|| {
                panic!(
                    "No widget slot for component id {}",
                    s.layout_model.get_focus()
                )
            });
        if slot != WidgetSlot::Details
            || *s.get_inspect_tabs().cursor.current() != InspectOption::Otel
        {
            return Vec::new();
        }

        if let Some(focused_span) = &s.otel_view.focused_span {
            // Set the currently focused span as the selected
            s.otel_view.selected_span = Some(focused_span.clone());
        }

        Vec::new()
    }
}

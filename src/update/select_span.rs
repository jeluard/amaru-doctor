use crate::{
    app_state::AppState,
    components::root::RootComponent,
    states::{Action, ComponentId},
    update::Update,
};
use ratatui::crossterm::event::KeyCode;

/// The Update fn for selecting the focused span.
pub struct SelectSpanUpdate;
impl Update for SelectSpanUpdate {
    fn update(&self, action: &Action, s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
        if !matches!(action, Action::Key(KeyCode::Enter)) {
            return Vec::new();
        }

        if s.layout_model.get_focus() != ComponentId::OtelFlameGraph {
            return Vec::new();
        }

        if let Some(focused_span) = &root.otel_page.view_state.focused_span {
            // Set the currently focused span as the selected
            root.otel_page.view_state.selected_span = Some(focused_span.clone());
        }

        Vec::new()
    }
}

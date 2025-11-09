use crate::{
    app_state::AppState,
    states::{Action, InspectOption, WidgetSlot},
    update::Update,
};
use crossterm::event::MouseEventKind;
use tracing::debug;

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

        let Some((_, rect)) = s
            .layout_model
            .find_hovered_slot(mouse_event.column, mouse_event.row)
        else {
            debug!("Couldn't find slot for click {:?}", mouse_event);
            return Vec::new();
        };

        let relative_row = mouse_event.row.saturating_sub(rect.y + 1) as usize;

        if *s.get_inspect_tabs().cursor.current() == InspectOption::Otel
            && s.layout_model.get_focus() == WidgetSlot::Details
            && let Some(selected_trace) = s.otel_view.trace_list.selected_item()
        {
            let trace_graph = s.otel_view.trace_graph_source.load();
            let mut trace_iter = trace_graph.trace_iter(selected_trace);

            if let Some(span_id) = trace_iter.nth(relative_row) {
                s.otel_view.focused_span = trace_graph.spans.get(&span_id).cloned();
            }
        }
        Vec::new()
    }
}

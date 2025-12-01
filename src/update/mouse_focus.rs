use crate::{
    app_state::AppState,
    components::root::RootComponent,
    otel::span_ext::SpanExt,
    states::{Action, ComponentId},
    update::Update,
};
use crossterm::event::MouseEventKind;
use tracing::debug;

pub struct MouseFocusUpdate;
impl Update for MouseFocusUpdate {
    fn update(&self, action: &Action, s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
        let Action::MouseEvent(mouse_event) = action else {
            return Vec::new();
        };

        if mouse_event.kind != MouseEventKind::Moved {
            // We only care about mouse move events
            return Vec::new();
        }

        s.layout_model
            .set_focus_by_location(mouse_event.column, mouse_event.row);

        let Some((component_id, rect)) = s
            .layout_model
            .find_hovered_component(mouse_event.column, mouse_event.row)
        else {
            debug!("Couldn't find slot for click {:?}", mouse_event);
            return Vec::new();
        };

        let relative_row = mouse_event.row.saturating_sub(rect.y + 1) as usize;

        if component_id != ComponentId::OtelFlameGraph {
            return Vec::new();
        }

        let trace_graph = root.otel_page.view_state.trace_graph.load();
        // The render logic displays different lists depending on whether a span is selected.
        // We iterate the same list to map the mouse row to the correct span.
        let hovered_span_id = if let Some(selected_span) = &root.otel_page.view_state.selected_span
        {
            // Zoomed View: Ancestors (Root -> Parent) + Descendants (Span -> Children)
            let selected_id = selected_span.span_id();

            // Ancestors are iterated Parent -> Root, so we must collect and reverse
            let ancestors = trace_graph
                .ancestor_iter(selected_id)
                .collect::<Vec<_>>()
                .into_iter()
                .rev();
            let descendants = trace_graph.descendent_iter(selected_id);
            ancestors.chain(descendants).nth(relative_row)
        } else if let Some(selected_trace) = &root.otel_page.view_state.selected_trace_id {
            // Full View: The entire trace in default order
            trace_graph.trace_iter(selected_trace).nth(relative_row)
        } else {
            None
        };
        // Update the focused span based on what we found (or clear it if we found nothing)
        root.otel_page.view_state.focused_span =
            hovered_span_id.and_then(|span_id| trace_graph.spans.get(&span_id).cloned());
        Vec::new()
    }
}

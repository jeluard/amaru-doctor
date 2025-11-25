use crate::{
    app_state::AppState,
    components::{Component, root::RootComponent},
    states::{
        Action,
        ComponentId::{self, *},
    },
    update::Update,
};
use crossterm::event::{MouseButton, MouseEventKind};
use tracing::debug;

pub struct MouseClickUpdate;

impl Update for MouseClickUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let (column, row) = match action {
            Action::MouseEvent(mouse_event) => {
                if mouse_event.kind != MouseEventKind::Down(MouseButton::Left) {
                    return Vec::new();
                }
                (mouse_event.column, mouse_event.row)
            }
            Action::MouseClick(col, row) => (*col, *row),
            _ => return Vec::new(),
        };

        let Some((component_id, rect)) = s.layout_model.find_hovered_slot(column, row) else {
            return Vec::new();
        };

        match component_id {
            InspectTabs => {
                if let Some(root) = s.component_registry.get_mut(&ComponentId::Root)
                    && let Some(root_comp) = root.as_any_mut().downcast_mut::<RootComponent>()
                {
                    root_comp.tabs.handle_click(rect, row, column);
                    return vec![Action::UpdateLayout(s.frame_area)];
                }
            }
            OtelTraceList => {
                s.get_trace_list_mut().handle_click(rect, row, column);

                // Side Effect: Update Focused Span based on new Trace selection
                let graph = s.otel_view.trace_graph_source.load();
                let new_focused_span = s
                    .get_trace_list()
                    .selected_item()
                    .and_then(|trace_id| graph.traces.get(trace_id))
                    .and_then(|trace_meta| trace_meta.roots().first_key_value())
                    .and_then(|(_, root_ids)| root_ids.first())
                    .and_then(|root_id| graph.spans.get(root_id))
                    .cloned();

                s.otel_view.focused_span = new_focused_span;
                s.otel_view.selected_span = None;
            }
            OtelFlameGraph => {
                if let Some(span) = &s.otel_view.focused_span {
                    s.otel_view.selected_span = Some(span.clone());
                }
            }

            _ => {
                debug!(
                    "Clicked component {} with no specific handler",
                    component_id
                );
            }
        }

        Vec::new()
    }
}

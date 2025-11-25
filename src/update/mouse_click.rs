use crate::{
    app_state::AppState,
    components::{Component, otel_page::OtelPageComponent, root::RootComponent},
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

        let Some((component_id, rect)) = s.layout_model.find_hovered_component(column, row) else {
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
                // TODO: This should no longer be in an Update Fn
                if let Some(page) = s.component_registry.get_mut(&OtelPage)
                    && let Some(otel_page) = page.as_any_mut().downcast_mut::<OtelPageComponent>()
                {
                    otel_page.trace_list.handle_click(rect, row, column);
                }

                let graph = s.otel_view.trace_graph_source.load();

                let selected_trace_id = s
                    .component_registry
                    .get(&OtelPage)
                    .and_then(|c| c.as_any().downcast_ref::<OtelPageComponent>())
                    .and_then(|p| p.trace_list.selected_item());

                let new_focused_span = selected_trace_id
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

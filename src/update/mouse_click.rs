use crate::{
    app_state::AppState,
    components::{Component, root::RootComponent},
    states::{Action, ComponentId::*},
    update::Update,
};
use crossterm::event::{MouseButton, MouseEventKind};
use tracing::debug;

pub struct MouseClickUpdate;
impl Update for MouseClickUpdate {
    fn update(&self, action: &Action, s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
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
                root.tabs.handle_click(rect, row, column);
                return vec![Action::UpdateLayout(s.frame_area)];
            }
            OtelTraceList => {
                root.otel_page.trace_list.handle_click(rect, row, column);
                let selected_trace = root.otel_page.trace_list.selected_item().copied();
                root.otel_page.view_state.select_trace(selected_trace);
            }
            OtelFlameGraph => {
                if let Some(span) = &root.otel_page.view_state.focused_span {
                    root.otel_page.view_state.selected_span = Some(span.clone());
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

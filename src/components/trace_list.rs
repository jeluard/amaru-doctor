use crate::{
    components::Component,
    otel::id::TraceId,
    states::{Action, ComponentId},
    viewmodel::dynamic_list::DynamicListViewModel,
};
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::{Frame, layout::Rect};
use std::any::Any;

pub struct TraceListComponent {
    id: ComponentId,
    // TODO: Should this be a ListComponent?
    list: DynamicListViewModel<TraceId>,
}

impl TraceListComponent {
    pub fn new(id: ComponentId) -> Self {
        Self {
            id,
            list: DynamicListViewModel::new("Traces"),
        }
    }

    pub fn sync_state(&mut self, new_trace_ids: Vec<TraceId>) {
        self.list.set_items(new_trace_ids);
    }

    pub fn selected_item(&self) -> Option<&TraceId> {
        self.list.selected_item()
    }

    pub fn handle_click(&mut self, area: Rect, row: u16, _col: u16) -> Vec<Action> {
        let relative_row = row.saturating_sub(area.y + 1) as usize;
        self.list.select_index_by_row(relative_row);
        Vec::new()
    }

    pub fn render_focused(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        self.list.draw(f, area, is_focused);
    }
}

impl Component for TraceListComponent {
    fn id(&self) -> ComponentId {
        self.id
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        self.list.set_height(area.height as usize);

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Up => {
                    self.list.cursor_back();
                }
                KeyCode::Down => {
                    self.list.cursor_next();
                }
                _ => {}
            },

            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => {
                    self.list.cursor_back();
                }
                MouseEventKind::ScrollDown => {
                    self.list.cursor_next();
                }
                MouseEventKind::Down(MouseButton::Left) => {
                    return self.handle_click(area, mouse.row, mouse.column);
                }
                _ => {}
            },
            _ => {}
        }

        Vec::new()
    }
}

use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, MouseScrollDirection, ScrollDirection},
    otel::id::TraceId,
    states::{Action, ComponentId},
    viewmodel::dynamic_list::DynamicListViewModel,
};
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use std::any::Any;

pub struct TraceListComponent {
    id: ComponentId,
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

    fn calculate_layout(&self, area: Rect, _s: &AppState) -> ComponentLayout {
        let mut layout = ComponentLayout::new();
        layout.insert(self.id, area);
        layout
    }

    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };
        let is_focused = s.layout_model.is_component_focused(self.id);

        self.list.draw(f, area, is_focused);
    }

    fn handle_scroll(&mut self, direction: ScrollDirection) -> Vec<Action> {
        match direction {
            ScrollDirection::Up => self.list.cursor_back(),
            ScrollDirection::Down => self.list.cursor_next(),
        }
        // Scrolling this list needs to clear the selected span
        Vec::new()
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action> {
        Vec::new()
    }

    fn handle_click(&mut self, area: Rect, row: u16, _col: u16) -> Vec<Action> {
        let relative_row = row.saturating_sub(area.y + 1) as usize;
        self.list.select_index_by_row(relative_row);
        Vec::new()
    }

    fn handle_mouse_scroll(&mut self, direction: MouseScrollDirection) -> Vec<Action> {
        match direction {
            MouseScrollDirection::Up => self.list.cursor_back(),
            MouseScrollDirection::Down => self.list.cursor_next(),
        }
        Vec::new()
    }

    fn handle_mouse_drag(&mut self, direction: ScrollDirection) -> Vec<Action> {
        match direction {
            ScrollDirection::Up => self.list.advance_window(),
            ScrollDirection::Down => self.list.retreat_window(),
        }
        Vec::new()
    }
}

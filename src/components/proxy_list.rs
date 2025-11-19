use crate::{
    app_state::AppState,
    components::{
        Component, ComponentLayout, MouseScrollDirection, ScrollDirection, list::ListModel,
    },
    states::{Action, ComponentId},
    view::empty_list::draw_empty_list,
};
use crossterm::event::KeyEvent;
use ratatui::prelude::*;
use std::any::Any;

pub type Accessor<M> = Box<dyn Fn(&AppState) -> Option<&M> + Send + Sync>;

pub struct ProxyListComponent<M> {
    id: ComponentId,
    accessor: Accessor<M>,
    empty_title: &'static str,
    empty_message: &'static str,
}

impl<M: ListModel> ProxyListComponent<M> {
    pub fn new(
        id: ComponentId,
        accessor: Accessor<M>,
        empty_title: &'static str,
        empty_message: &'static str,
    ) -> Self {
        Self {
            id,
            accessor,
            empty_title,
            empty_message,
        }
    }
}

impl<M: ListModel> Component for ProxyListComponent<M> {
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
        let mut l = ComponentLayout::new();
        l.insert(self.id, area);
        l
    }

    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };
        let is_focused = s.layout_model.is_focused(self.id);

        if let Some(model) = (self.accessor)(s) {
            model.draw(f, area, is_focused);
        } else {
            draw_empty_list(f, area, self.empty_title, self.empty_message, is_focused);
        }
    }

    // Events are handled by the Update loop (SearchUpdate), not this component
    fn handle_scroll(&mut self, _d: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_key_event(&mut self, _k: KeyEvent) -> Vec<Action> {
        Vec::new()
    }
    fn handle_click(&mut self, _area: Rect, _row: u16, _col: u16) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_scroll(&mut self, _d: MouseScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_drag(&mut self, _d: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
}

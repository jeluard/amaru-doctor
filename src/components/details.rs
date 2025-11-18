use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, MouseScrollDirection, ScrollDirection},
    states::{Action, ComponentId},
    ui::ToRichText,
    view::item_details::draw_details,
};
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use std::{any::Any, marker::PhantomData};

type DataAccessor<T> = Box<dyn Fn(&AppState) -> Option<&T> + Send + Sync>;

pub struct DetailsComponent<T>
where
    T: ToRichText + Send + Sync + 'static,
{
    id: ComponentId,
    title: &'static str,
    data_accessor: DataAccessor<T>,
    _phantom: PhantomData<T>,
}

impl<T> DetailsComponent<T>
where
    T: ToRichText + Send + Sync + 'static,
{
    pub fn new(id: ComponentId, title: &'static str, data_accessor: DataAccessor<T>) -> Self {
        Self {
            id,
            title,
            data_accessor,
            _phantom: PhantomData,
        }
    }
}

impl<T> Component for DetailsComponent<T>
where
    T: ToRichText + Send + Sync + 'static,
{
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
        let selected_item = (self.data_accessor)(s);
        draw_details(f, area, self.title.to_string(), selected_item, is_focused);
    }

    fn handle_scroll(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action> {
        Vec::new()
    }
    fn handle_click(&mut self, _area: Rect, _row: u16, _col: u16) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_scroll(&mut self, _direction: MouseScrollDirection) -> Vec<Action> {
        Vec::new()
    }
    fn handle_mouse_drag(&mut self, _direction: ScrollDirection) -> Vec<Action> {
        Vec::new()
    }
}

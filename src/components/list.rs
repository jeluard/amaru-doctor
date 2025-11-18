use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, MouseScrollDirection, ScrollDirection},
    model::list_view::ListModelView,
    states::{Action, ComponentId},
    ui::to_list_item::ToListItem,
};
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};
use std::{any::Any, marker::PhantomData};
use tracing::{debug, info};

/// A stateful, reusable component that renders a scrollable list.
/// It encapsulates the `ListModelViewState` (data + view state).
pub struct ListComponent<T>
where
    T: ToListItem + Send + Sync + 'static,
{
    id: ComponentId,
    pub model_view: ListModelView<T>,
    _phantom: PhantomData<T>,
}

impl<T> ListComponent<T>
where
    T: ToListItem + Send + Sync + 'static,
{
    pub fn new(
        id: ComponentId,
        title: &'static str,
        iter: impl Iterator<Item = T> + Send + Sync + 'static,
        initial_buffer_size: usize,
    ) -> Self {
        Self {
            id,
            model_view: ListModelView::new(title, iter, initial_buffer_size),
            _phantom: PhantomData,
        }
    }
}

impl<T> Component for ListComponent<T>
where
    T: ToListItem + Send + Sync + 'static,
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

    /// Renders the list widget.
    fn render(&self, f: &mut Frame, s: &AppState, layout: &ComponentLayout) {
        let Some(&area) = layout.get(&self.id) else {
            return;
        };
        // Use the old focus model for now
        let is_focused = s.layout_model.is_component_focused(self.id);
        self.model_view.draw(f, area, is_focused);
    }

    /// Handles mouse clicks to select an item.
    fn handle_click(&mut self, area: Rect, row: u16, _col: u16) -> Vec<Action> {
        // +1 to account for the border
        let relative_row = row.saturating_sub(area.y + 1) as usize;
        self.model_view.select_index_by_row(relative_row);
        Vec::new()
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action> {
        info!("No key actions for ListComponent");
        Vec::new()
    }

    fn handle_scroll(&mut self, direction: ScrollDirection) -> Vec<Action> {
        debug!(
            "ListComponent: handle_scroll for id {:?} with direction {:?}",
            self.id, direction
        );
        match direction {
            ScrollDirection::Up => self.model_view.cursor_back(),
            ScrollDirection::Down => self.model_view.cursor_next(),
        }
        Vec::new()
    }

    fn handle_mouse_scroll(&mut self, direction: MouseScrollDirection) -> Vec<Action> {
        debug!(
            "ListComponent: handle_mouse_scroll for id {:?} with direction {:?}",
            self.id, direction
        );
        match direction {
            MouseScrollDirection::Up => self.model_view.cursor_back(),
            MouseScrollDirection::Down => self.model_view.cursor_next(),
        }
        Vec::new()
    }

    fn handle_mouse_drag(&mut self, direction: ScrollDirection) -> Vec<Action> {
        debug!(
            "ListComponent: handle_mouse_drag for id {:?} with direction {:?}",
            self.id, direction
        );
        match direction {
            ScrollDirection::Up => self.model_view.advance_window(),
            ScrollDirection::Down => self.model_view.retreat_window(),
        }
        Vec::new()
    }
}

use crate::{
    app_state::AppState,
    components::{Component, ComponentLayout, MouseScrollDirection, ScrollDirection},
    model::list_view::ListModelView,
    states::{Action, ComponentId},
    tui::Event,
    ui::to_list_item::ToListItem,
};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, MouseButton, MouseEventKind},
    layout::Rect,
};
use std::any::Any;
use tracing::info;

/// Abstraction for any list-like data model that can be drawn and navigated.
pub trait ListModel: Send + Sync + 'static {
    type Item;
    fn draw(&self, f: &mut Frame, area: Rect, is_focused: bool);
    fn selected_item(&self) -> Option<&Self::Item>;
    fn select_index_by_row(&mut self, relative_row: usize);
    fn cursor_back(&mut self);
    fn cursor_next(&mut self);
    fn retreat_window(&mut self);
    fn advance_window(&mut self);
    fn set_height(&mut self, height: usize);
}

// Implement for the Static List Model (based on StreamingIter)
impl<T> ListModel for ListModelView<T>
where
    T: ToListItem + Send + Sync + 'static,
{
    type Item = T;

    fn draw(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        self.draw(f, area, is_focused);
    }

    fn selected_item(&self) -> Option<&Self::Item> {
        self.selected_item()
    }

    fn select_index_by_row(&mut self, relative_row: usize) {
        self.select_index_by_row(relative_row);
    }

    fn cursor_back(&mut self) {
        self.cursor_back();
    }

    fn cursor_next(&mut self) {
        self.cursor_next();
    }

    fn retreat_window(&mut self) {
        self.retreat_window();
    }

    fn advance_window(&mut self) {
        self.advance_window();
    }

    fn set_height(&mut self, height: usize) {
        self.set_height(height);
    }
}

/// A stateful, reusable component that renders a scrollable list.
/// It wraps any model that implements `ListModel`.
pub struct ListComponent<M>
where
    M: ListModel,
{
    id: ComponentId,
    pub model: M,
    last_drag_y: Option<u16>,
}

impl<M> ListComponent<M>
where
    M: ListModel,
{
    pub fn new(id: ComponentId, model: M) -> Self {
        Self {
            id,
            model,
            last_drag_y: None,
        }
    }
}

impl<M> Component for ListComponent<M>
where
    M: ListModel,
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
        let is_focused = s.layout_model.is_focused(self.id);
        self.model.draw(f, area, is_focused);
    }

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Up => self.handle_scroll(ScrollDirection::Up),
                KeyCode::Down => self.handle_scroll(ScrollDirection::Down),
                _ => Vec::new(),
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollUp => self.handle_mouse_scroll(MouseScrollDirection::Up),
                MouseEventKind::ScrollDown => self.handle_mouse_scroll(MouseScrollDirection::Down),
                MouseEventKind::Down(MouseButton::Left) => {
                    self.last_drag_y = Some(mouse.row);
                    self.handle_click(area, mouse.row, mouse.column)
                }
                MouseEventKind::Drag(ratatui::crossterm::event::MouseButton::Left) => {
                    let Some(last_y) = self.last_drag_y else {
                        // Safety: If we missed the Down event, start tracking now
                        self.last_drag_y = Some(mouse.row);
                        return Vec::new();
                    };

                    if mouse.row > last_y {
                        self.last_drag_y = Some(mouse.row);
                        self.handle_mouse_drag(ScrollDirection::Down)
                    } else if mouse.row < last_y {
                        self.last_drag_y = Some(mouse.row);
                        self.handle_mouse_drag(ScrollDirection::Up)
                    } else {
                        Vec::new()
                    }
                }
                MouseEventKind::Up(_) => {
                    self.last_drag_y = None;
                    Vec::new()
                }
                _ => Vec::new(),
            },
            _ => Vec::new(),
        }
    }

    fn handle_click(&mut self, area: Rect, row: u16, _col: u16) -> Vec<Action> {
        // +1 to account for the border
        let relative_row = row.saturating_sub(area.y + 1) as usize;
        self.model.select_index_by_row(relative_row);
        Vec::new()
    }

    fn handle_key_event(&mut self, _key: KeyEvent) -> Vec<Action> {
        info!("No key actions for ListComponent");
        Vec::new()
    }

    fn handle_scroll(&mut self, direction: ScrollDirection) -> Vec<Action> {
        match direction {
            ScrollDirection::Up => self.model.cursor_back(),
            ScrollDirection::Down => self.model.cursor_next(),
        }
        Vec::new()
    }

    fn handle_mouse_scroll(&mut self, direction: MouseScrollDirection) -> Vec<Action> {
        match direction {
            MouseScrollDirection::Up => self.model.cursor_back(),
            MouseScrollDirection::Down => self.model.cursor_next(),
        }
        Vec::new()
    }

    fn handle_mouse_drag(&mut self, direction: ScrollDirection) -> Vec<Action> {
        match direction {
            ScrollDirection::Up => self.model.advance_window(),
            ScrollDirection::Down => self.model.retreat_window(),
        }
        Vec::new()
    }
}

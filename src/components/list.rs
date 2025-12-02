use crate::{
    components::{Component, ComponentLayout},
    model::list_view::ListModelView,
    states::{Action, ComponentId},
    tui::Event,
    ui::to_list_item::ToListItem,
};
use ratatui::{
    Frame,
    crossterm::event::{KeyCode, MouseButton, MouseEventKind},
    layout::Rect,
};
use std::any::Any;

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

    pub fn render_focused(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        self.model.draw(f, area, is_focused);
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

    fn calculate_layout(&self, area: Rect) -> ComponentLayout {
        let mut layout = ComponentLayout::new();
        layout.insert(self.id, area);
        layout
    }

    fn render(&self, _f: &mut Frame, _layout: &ComponentLayout) {}

    fn handle_event(&mut self, event: &Event, area: Rect) -> Vec<Action> {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Up => {
                    self.model.cursor_back();
                }
                KeyCode::Down => {
                    self.model.cursor_next();
                }
                _ => {}
            },
            Event::Mouse(mouse) => {
                match mouse.kind {
                    MouseEventKind::ScrollUp => {
                        self.model.cursor_back();
                    }
                    MouseEventKind::ScrollDown => {
                        self.model.cursor_next();
                    }

                    MouseEventKind::Down(MouseButton::Left) => {
                        self.last_drag_y = Some(mouse.row);

                        // Direct Click Handling
                        // +1 for border
                        let relative_row = mouse.row.saturating_sub(area.y + 1) as usize;
                        self.model.select_index_by_row(relative_row);
                    }

                    MouseEventKind::Drag(MouseButton::Left) => {
                        let Some(last_y) = self.last_drag_y else {
                            self.last_drag_y = Some(mouse.row);
                            return Vec::new();
                        };

                        if mouse.row > last_y {
                            self.last_drag_y = Some(mouse.row);
                            self.model.retreat_window(); // Drag Down -> Retreat
                        } else if mouse.row < last_y {
                            self.last_drag_y = Some(mouse.row);
                            self.model.advance_window(); // Drag Up -> Advance
                        }
                    }

                    MouseEventKind::Up(_) => {
                        self.last_drag_y = None;
                    }

                    _ => {}
                }
            }
            _ => {}
        }

        // We consumed the event and mutated state.
        // If this is an Options list, App.rs will inject UpdateLayout automatically.
        Vec::new()
    }
}

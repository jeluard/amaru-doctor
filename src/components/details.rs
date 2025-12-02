use crate::{
    components::{Component, ScrollDirection},
    states::{Action, ComponentId},
    tui::Event,
    ui::ToRichText,
    view::item_details::draw_details,
};
use crossterm::event::{KeyCode, MouseButton, MouseEventKind};
use ratatui::{Frame, layout::Rect};
use std::{
    any::Any,
    marker::PhantomData,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct DetailsComponent<T>
where
    T: ToRichText + Send + Sync + 'static,
{
    id: ComponentId,
    title: &'static str,
    scroll_offset: u16,
    is_focused: AtomicBool,
    _phantom: PhantomData<T>,
}

impl<T> DetailsComponent<T>
where
    T: ToRichText + Send + Sync + 'static,
{
    pub fn new(id: ComponentId, title: &'static str) -> Self {
        Self {
            id,
            title,
            scroll_offset: 0,
            is_focused: AtomicBool::new(false),
            _phantom: PhantomData,
        }
    }

    pub fn render_with_data(&self, f: &mut Frame, area: Rect, is_focused: bool, item: Option<&T>) {
        self.is_focused.store(is_focused, Ordering::Relaxed);
        draw_details(f, area, self.title.to_string(), item, is_focused);
    }

    fn perform_scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Up => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
            ScrollDirection::Down => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
            }
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

    fn handle_event(&mut self, event: &Event, _area: Rect) -> Vec<Action> {
        match event {
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::Moved => {
                    if !self.is_focused.load(Ordering::Relaxed) {
                        return vec![Action::SetFocus(self.id)];
                    }
                }
                MouseEventKind::Down(MouseButton::Left) => {
                    return vec![Action::SetFocus(self.id)];
                }
                MouseEventKind::ScrollUp => {
                    self.perform_scroll(ScrollDirection::Up);
                }
                MouseEventKind::ScrollDown => {
                    self.perform_scroll(ScrollDirection::Down);
                }
                _ => {}
            },
            Event::Key(key) => match key.code {
                KeyCode::Up => {
                    self.perform_scroll(ScrollDirection::Up);
                }
                KeyCode::Down => {
                    self.perform_scroll(ScrollDirection::Down);
                }
                _ => {}
            },
            _ => {}
        }
        Vec::new()
    }
}

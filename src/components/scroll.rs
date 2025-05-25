use crate::focus::Focusable;
use crate::{action::Action, window::WindowedIter};
use color_eyre::Result;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};

use super::Component;

pub struct ScrollableListComponent<T, I, F>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem + Copy,
{
    title: String,
    window: WindowedIter<T, I>,
    render_item: F,
    list_state: ListState,
    has_focus: bool,
}

impl<T, I, F> ScrollableListComponent<T, I, F>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem + Copy,
{
    pub fn new(title: String, iter: I, window_size: usize, render_item: F) -> Self
    where
        I: Iterator<Item = T>,
    {
        let window = WindowedIter::new(iter, window_size);
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            title,
            window,
            render_item,
            has_focus: false,
            list_state,
        }
    }
}

impl<T, I, F> Focusable for ScrollableListComponent<T, I, F>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem + Copy,
{
    fn set_focus(&mut self, focus: bool) {
        self.has_focus = focus;
    }

    fn has_focus(&self) -> bool {
        self.has_focus
    }
}

impl<T, I, F> Component for ScrollableListComponent<T, I, F>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem + Copy,
{
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            return Ok(vec![]);
        }
        match key.code {
            KeyCode::Up => {
                if let Some(i) = self.list_state.selected() {
                    if i == 0 {
                        self.window.scroll_up();
                    } else {
                        self.list_state.select(Some(i - 1));
                    }
                }
            }
            KeyCode::Down => {
                let view_len = self.window.view().len();
                if let Some(i) = self.list_state.selected() {
                    if i + 1 >= view_len {
                        self.window.scroll_down();
                    } else {
                        self.list_state.select(Some(i + 1));
                    }
                }
            }
            _ => {}
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.window.set_window_size(area.rows().count());
        let view: Vec<T> = self.window.view().to_vec();
        let items: Vec<ListItem> = view.iter().map(self.render_item).collect();

        let mut block = Block::default()
            .title("Resources")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);
        if self.has_focus() {
            block = block.border_style(Style::default().fg(Color::Blue));
        }
        let list = List::new(items)
            .highlight_symbol(">> ")
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .block(block);

        StatefulWidget::render(list, area, frame.buffer_mut(), &mut self.list_state);
        Ok(())
    }
}

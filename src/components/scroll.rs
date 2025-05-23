use crate::{action::Action, window::WindowedIter};
use color_eyre::Result;
use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use ratatui::{prelude::*, widgets::*};

use super::Component;

pub struct ScrollableListComponent<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem,
{
    title: String,
    window: WindowedIter<T, I>,
    render_item: F,
}

impl<T, I, F> ScrollableListComponent<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem,
{
    pub fn new(title: String, iter: I, window_size: usize, render_item: F) -> Self
    where
        I: Iterator<Item = T>,
    {
        let window = WindowedIter::new(iter, window_size);
        Self {
            title,
            window,
            render_item,
        }
    }
}

impl<T, I, F> Component for ScrollableListComponent<T, I, F>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem,
{
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        match key.code {
            KeyCode::Up => self.window.scroll_up(),
            KeyCode::Down => self.window.scroll_down(),
            _ => {}
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let items: Vec<ListItem> = self.window.view().iter().map(&self.render_item).collect();

        let list = List::new(items).block(
            Block::default()
                .title(self.title.to_string())
                .borders(Borders::ALL),
        );
        frame.render_widget(list, area);
        Ok(())
    }
}

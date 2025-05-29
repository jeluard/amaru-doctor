use super::Component;
use crate::action::{Action, SelectedItem};
use crate::focus::Focusable;
use crate::window::state::WindowState;
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};

pub struct ScrollableListComponent<T, I, F, S>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem + Copy,
    S: Fn(&T) -> Option<SelectedItem> + Copy,
{
    title: String,
    state: WindowState<T, I>,
    render_item: F,
    select_mapper: S,
    has_focus: bool,
}

impl<T, I, F, S> ScrollableListComponent<T, I, F, S>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem + Copy,
    S: Fn(&T) -> Option<SelectedItem> + Copy,
{
    pub fn new(
        title: String,
        iter: I,
        window_size: usize,
        render_item: F,
        select_mapper: S,
    ) -> Self {
        let state = WindowState::new(iter, window_size);
        Self {
            title,
            state,
            render_item,
            select_mapper,
            has_focus: false,
        }
    }
}

impl<T, I, F, S> Focusable for ScrollableListComponent<T, I, F, S>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem + Copy,
    S: Fn(&T) -> Option<SelectedItem> + Copy,
{
    fn set_focus(&mut self, focus: bool) {
        self.has_focus = focus;
    }

    fn has_focus(&self) -> bool {
        self.has_focus
    }
}

impl<T, I, F, S> Component for ScrollableListComponent<T, I, F, S>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(&T) -> ListItem + Copy,
    S: Fn(&T) -> Option<SelectedItem> + Copy,
{
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            return Ok(vec![]);
        }

        match key.code {
            KeyCode::Up => self.state.scroll_up(),
            KeyCode::Down => self.state.scroll_down(),
            _ => {}
        }

        if let Some(item) = self.state.selected_item() {
            if let Some(selected) = (self.select_mapper)(item) {
                return Ok(vec![Action::SelectItem(selected)]);
            }
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.state.set_window_size(area.rows().count());
        let (view, selected) = self.state.window_with_selected_index();
        let items: Vec<ListItem> = view.iter().map(self.render_item).collect();

        let mut block = Block::default()
            .title(self.title.clone())
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);

        if self.has_focus {
            block = block.border_style(Style::default().fg(Color::Blue));
        }

        let list = List::new(items)
            .highlight_symbol(">> ")
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .block(block);

        let mut list_state = ListState::default().with_selected(Some(selected));
        frame.render_stateful_widget(list, area, &mut list_state);
        Ok(())
    }
}

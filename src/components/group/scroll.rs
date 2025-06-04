use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::Getter,
    to_list_item::ToListItem,
    to_rich::ToRichText,
    window::state::WindowState,
};
use color_eyre::{Result, eyre::Ok};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tracing::trace;

pub struct ScrollableListComponent<T, I>
where
    T: Clone + ToListItem,
    I: Iterator<Item = T>,
{
    title: String,
    state: WindowState<T, I>,
    focus: FocusState,
}

impl<T, I> ScrollableListComponent<T, I>
where
    T: Clone + ToListItem,
    I: Iterator<Item = T>,
{
    pub fn new(title: String, iter: I, window_size: usize) -> Self {
        let state = WindowState::new(iter, window_size);
        Self {
            title,
            state,
            focus: FocusState::default(),
        }
    }
}

impl<T, I> Getter<T> for ScrollableListComponent<T, I>
where
    T: Clone + ToListItem,
    I: Iterator<Item = T>,
{
    fn get_mut(&mut self) -> Option<T> {
        self.state.selected_item().cloned()
    }
}

impl<T, I> FocusableComponent for ScrollableListComponent<T, I>
where
    T: Clone + ToListItem,
    I: Iterator<Item = T>,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl<T, I> Component for ScrollableListComponent<T, I>
where
    T: Clone + ToListItem,
    I: Iterator<Item = T>,
{
    fn debug_name(&self) -> String {
        format!("ScrollableListComponent:{}", self.title)
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            trace!("ScrollableListComponent::{}: no focus", self.title);
            return Ok(vec![]);
        }
        trace!("ScrollableListComponent::{}: has focus", self.title);

        match key.code {
            KeyCode::Up => self.state.scroll_up(),
            KeyCode::Down => self.state.scroll_down(),
            _ => {
                trace!(
                    "ScrollableListComponent::{}: no match for key code {}",
                    self.title, key.code
                );
            }
        }

        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let is_focused = self.has_focus();
        self.state.set_window_size(area.rows().count());
        let (view, selected) = self.state.window_with_selected_index();
        let items: Vec<ListItem> = view.iter().map(|i| i.to_list_item()).collect();

        let mut block = Block::default()
            .title(self.title.clone())
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);

        if is_focused {
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

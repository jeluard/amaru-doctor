use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::Getter,
    to_list_item::ToListItem,
    window::state::WindowState,
};
use color_eyre::{Result, eyre::Ok};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use std::cell::{Ref, RefMut};
use tracing::trace;

pub struct ScrollableListComponent<'a, T>
where
    T: Clone + ToListItem,
{
    title: String,
    state: WindowState<'a, T>,
    focus: FocusState,
}

impl<'a, T> ScrollableListComponent<'a, T>
where
    T: Clone + ToListItem,
{
    pub fn new<I>(title: String, iter: I, window_size: usize) -> Self
    where
        I: Iterator<Item = T> + 'a,
    {
        let state = WindowState::new(iter, window_size);
        Self {
            title,
            state,
            focus: FocusState::default(),
        }
    }
}

impl<'a, T> Getter<T> for ScrollableListComponent<'a, T>
where
    T: Clone + ToListItem + 'a,
{
    fn get(&self) -> Option<Ref<T>> {
        self.state.selected_item()
    }

    fn get_mut(&self) -> Option<RefMut<T>> {
        self.state.selected_item_mut()
    }
}

impl<'a, T> FocusableComponent for ScrollableListComponent<'a, T>
where
    T: Clone + ToListItem,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl<'a, T> Component for ScrollableListComponent<'a, T>
where
    T: Clone + ToListItem,
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

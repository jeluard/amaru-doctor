use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::Getter,
    to_list_item::ToListItem,
    window::{WindowSource, WindowState},
};
use color_eyre::{Result, eyre::Ok};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use std::{cell::Ref, rc::Rc};
use tracing::trace;

pub struct ScrollableListComponent<'a, T>
where
    T: Clone + ToListItem + 'a,
{
    title: String,
    state: WindowState<'a, T>,
    focus: FocusState,
}

impl<'a, T> ScrollableListComponent<'a, T>
where
    T: Clone + ToListItem + 'a,
{
    pub fn new(title: String, source: Rc<dyn WindowSource<T> + 'a>, window_size: usize) -> Self {
        Self {
            title,
            state: WindowState::new(source, window_size),
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
}

impl<'a, T> FocusableComponent for ScrollableListComponent<'a, T>
where
    T: Clone + ToListItem + 'a,
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
    T: Clone + ToListItem + 'a,
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
                    "ScrollableListComponent::{}: no match for key code {:?}",
                    self.title, key.code
                );
            }
        }

        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.state.set_window_size(area.rows().count());
        let (view, selected) = self.state.window_with_selected_index();
        let items: Vec<ListItem> = view.iter().map(|i| i.to_list_item()).collect();

        let mut block = Block::default()
            .title(self.title.clone())
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);

        if self.has_focus() {
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

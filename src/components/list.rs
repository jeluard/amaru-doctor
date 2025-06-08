use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::Getter,
    to_list_item::ToListItem,
    window::{WindowSource, WindowState},
};
use crossterm::event::KeyCode;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use std::{cell::Ref, rc::Rc};

pub struct ListComponent<T>
where
    T: ToListItem,
{
    title: String,
    state: WindowState<T>,
    focus: FocusState,
}

impl<T> ListComponent<T>
where
    T: ToListItem,
{
    pub fn new(title: String, source: Rc<dyn WindowSource<T>>, window_size: usize) -> Self {
        Self {
            title,
            state: WindowState::new(source, window_size),
            focus: FocusState::default(),
        }
    }
}

impl<T> Getter<T> for ListComponent<T>
where
    T: ToListItem,
{
    fn get(&self) -> Option<Ref<T>> {
        self.state.selected_item()
    }
}

impl<T> FocusableComponent for ListComponent<T>
where
    T: ToListItem,
{
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }
    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl<T> Component for ListComponent<T>
where
    T: ToListItem,
{
    fn debug_name(&self) -> String {
        format!("ListComponent:{}", self.title)
    }

    fn handle_key_event(
        &mut self,
        evt: crossterm::event::KeyEvent,
    ) -> color_eyre::Result<Vec<Action>> {
        if !self.has_focus() {
            return Ok(vec![]);
        }
        match evt.code {
            KeyCode::Up => self.state.scroll_up(),
            KeyCode::Down => self.state.scroll_down(),
            _ => {}
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        self.state.set_window_size(area.rows().count());

        let (view, selected) = self.state.window_with_selected_index();
        let items = view
            .iter()
            .map(|i| i.to_list_item())
            .collect::<Vec<ListItem>>();

        let mut block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL);
        if self.has_focus() {
            block = block
                .border_style(Style::default().fg(Color::Blue))
                .title_style(Style::default().fg(Color::White));
        }

        let list = List::new(items)
            .highlight_symbol(">> ")
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .block(block);

        let mut state = ListState::default();
        state.select(Some(selected));
        frame.render_stateful_widget(list, area, &mut state);

        Ok(())
    }
}

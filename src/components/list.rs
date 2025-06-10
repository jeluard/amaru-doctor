use crate::{
    action::Action,
    components::Component,
    focus::{FocusState, FocusableComponent},
    shared::GetterOpt,
    to_list_item::ToListItem,
    window::WindowState,
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use tracing::trace;

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
    pub fn from_iter(title: String, iter: Box<dyn Iterator<Item = T> + 'static>) -> Self {
        Self {
            title,
            state: WindowState::new(iter),
            focus: FocusState::default(),
        }
    }
}

impl<T> GetterOpt<T> for ListComponent<T>
where
    T: ToListItem,
{
    fn get(&self) -> Option<&T> {
        self.state.get()
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

    fn handle_key_event(&mut self, evt: KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            trace!("{}: No focus, len {}", self.debug_name(), self.state.len());
            return Ok(vec![]);
        }
        trace!(
            "{}: Have focus, len {}",
            self.debug_name(),
            self.state.len()
        );
        match evt.code {
            KeyCode::Up => self.state.scroll_up(),
            KeyCode::Down => self.state.scroll_down(),
            _ => {}
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        self.state.set_window_size(area.rows().count());

        let (view, selected) = self.state.window_view();
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

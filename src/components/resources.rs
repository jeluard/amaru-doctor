use super::Component;
use crate::{
    action::Action,
    focus::{FocusState, Focusable},
};
use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use tracing::trace;

pub struct ResourceList {
    state: ListState,
    focus: FocusState,
}

impl Default for ResourceList {
    fn default() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            state,
            focus: FocusState::default(),
        }
    }
}

impl ResourceList {
    fn next(&mut self, len: usize) {
        let i = match self.state.selected() {
            Some(i) if i + 1 < len => i + 1,
            _ => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self, len: usize) {
        let i = match self.state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => len.saturating_sub(1),
        };
        self.state.select(Some(i));
    }
}

impl Focusable for ResourceList {
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl Component for ResourceList {
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<Vec<Action>> {
        trace!("ResourceList::handle_key_event - {:?}", key);
        let items_len = 3;
        if !self.has_focus() {
            return Ok(vec![]);
        }
        match key.code {
            KeyCode::Down => {
                trace!("Moving down");
                self.next(items_len);
            }
            KeyCode::Up => {
                trace!("Moving up");
                self.previous(items_len);
            }
            _ => return Ok(vec![]),
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let resources: Vec<ListItem> = vec![String::from("utxos"), String::from("WIP")]
            .iter()
            .map(|r| ListItem::new(format!("{:}", r)))
            .collect();

        let mut block = Block::default()
            .title("Resources")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);
        if self.has_focus() {
            block = block.border_style(Style::default().fg(Color::Blue));
        }
        let list = List::new(resources)
            .highlight_symbol(">> ")
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .block(block);

        StatefulWidget::render(list, area, frame.buffer_mut(), &mut self.state);

        Ok(())
    }
}

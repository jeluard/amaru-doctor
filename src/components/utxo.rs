use super::Component;
use crate::{
    action::{Action, SelectedItem, SelectedState},
    focus::{FocusState, Focusable},
    to_rich::ToRichText,
};
use amaru_ledger::store::{ReadOnlyStore, columns::utxo::Key};
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::{prelude::*, widgets::*};
use std::sync::Arc;

pub struct UtxoDetailsComponent {
    db: Arc<RocksDB>,
    selected: SelectedState<Key>,
    scroll_offset: u16,
    focus: FocusState,
}

impl UtxoDetailsComponent {
    pub fn new(db: Arc<RocksDB>) -> Self {
        Self {
            db,
            selected: SelectedState::new(|s| match s {
                SelectedItem::Utxo(k) => Some(k.clone()),
                _ => None,
            }),
            scroll_offset: 0,
            focus: FocusState::default(),
        }
    }
}

impl Focusable for UtxoDetailsComponent {
    fn focus_state(&self) -> &FocusState {
        &self.focus
    }

    fn focus_state_mut(&mut self) -> &mut FocusState {
        &mut self.focus
    }
}

impl Component for UtxoDetailsComponent {
    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        if self.selected.update(&action) {
            self.scroll_offset = 0;
        }
        Ok(vec![])
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> Result<Vec<Action>> {
        if !self.has_focus() {
            return Ok(vec![]);
        }

        match key.code {
            KeyCode::Up => self.scroll_offset = self.scroll_offset.saturating_sub(1),
            KeyCode::Down => self.scroll_offset = self.scroll_offset.saturating_add(1),
            _ => {}
        }
        return Ok(vec![]);
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::default()
            .title("Utxo Details")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);
        if self.has_focus() {
            block = block.border_style(Style::default().fg(Color::Blue));
        }

        let lines = match self.selected.value.as_ref() {
            Some(key) => match self.db.utxo(key)? {
                Some(val) => (key.clone(), val).into_rich_text().unwrap_lines(),
                None => vec![Line::from("Not found")],
            },
            None => vec![Line::from("None selected")],
        };

        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: true })
            .scroll((self.scroll_offset, 0));
        frame.render_widget(paragraph, area);
        Ok(())
    }
}

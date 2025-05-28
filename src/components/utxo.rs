use std::sync::Arc;

use super::Component;
use crate::{action::Action, focus::Focusable, to_rich::ToRichText};
use amaru_ledger::store::{ReadOnlyStore, columns::utxo::Key};
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};

pub struct UtxoDetailsComponent {
    db: Arc<RocksDB>,
    cur_key: Option<Key>,
    has_focus: bool,
}

impl UtxoDetailsComponent {
    pub fn new(db: Arc<RocksDB>) -> Self {
        Self {
            db,
            cur_key: None,
            has_focus: false,
        }
    }
}

impl Focusable for UtxoDetailsComponent {
    fn set_focus(&mut self, focus: bool) {
        self.has_focus = focus;
    }

    fn has_focus(&self) -> bool {
        self.has_focus
    }
}

impl Component for UtxoDetailsComponent {
    fn update(&mut self, action: Action) -> Result<Vec<Action>> {
        if let Action::SelectItem(crate::action::SelectedItem::Utxo(key)) = action {
            self.cur_key = Some(key);
        }
        Ok(vec![])
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let mut block = Block::default()
            .title("Utxo Details")
            .title_style(Style::default().fg(Color::White))
            .borders(Borders::ALL);
        if self.has_focus() {
            block = block.border_style(Style::default().fg(Color::Blue));
        }

        let lines = match &self.cur_key {
            Some(key) => match self.db.utxo(key)? {
                Some(val) => (key.clone(), val).into_rich_text().unwrap_lines(),
                None => vec![Line::from("Not found")],
            },
            None => vec![Line::from("None selected")],
        };

        let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
        Ok(())
    }
}

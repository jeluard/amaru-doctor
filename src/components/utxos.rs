use std::sync::Arc;

use super::Component;
use crate::action::Action;
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};

pub struct UtxoList {
    db: Arc<RocksDB>,
}

impl UtxoList {
    pub fn new(db: Arc<RocksDB>) -> Self {
        Self { db }
    }
}

impl Component for UtxoList {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let utxos: Vec<ListItem> = self
            .db
            .as_ref()
            .iter_utxos()?
            .map(|(k, _)| ListItem::new(format!("{:?}", k.transaction_id)))
            .take(10)
            .collect();

        let list = List::new(utxos).block(Block::default().title("UTXOs").borders(Borders::ALL));
        frame.render_widget(list, area);

        Ok(())
    }
}

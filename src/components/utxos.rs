use super::Component;
use super::scroll::ScrollableListComponent;
use amaru_ledger::store::ReadOnlyStore;
use amaru_ledger::store::columns::utxo;
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use ratatui::widgets::*;

pub struct UtxoList<'a> {
    window: ScrollableListComponent<
        (utxo::Key, utxo::Value),
        Box<dyn Iterator<Item = (utxo::Key, utxo::Value)> + 'a>,
        fn(&(utxo::Key, utxo::Value)) -> ListItem,
    >,
}

impl<'a> UtxoList<'a> {
    pub fn new(db: &'a RocksDB) -> Result<Self> {
        let iter = db.iter_utxos()?;
        let iter: Box<dyn Iterator<Item = (utxo::Key, utxo::Value)> + 'a> = Box::new(iter);

        fn render((input, _): &(utxo::Key, utxo::Value)) -> ListItem {
            ListItem::new(format!("{:?}", input.transaction_id))
        }

        let window = ScrollableListComponent::new(
            "UTXOs".to_string(),
            iter,
            10,
            render as fn(&(utxo::Key, utxo::Value)) -> ListItem,
        );
        Ok(Self { window })
    }
}

impl<'a> Component for UtxoList<'a> {
    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) -> Result<()> {
        todo!()
    }
}

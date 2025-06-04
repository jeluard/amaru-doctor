use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_list_item::ToListItem,
    to_rich::utxo::TransactionInputDisplay,
};
use amaru_ledger::store::{ReadOnlyStore, columns::utxo};
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

type UtxoItem = (utxo::Key, utxo::Value);

impl ToListItem for UtxoItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(TransactionInputDisplay(&self.0).to_string())
    }
}

pub fn new_utxo_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<UtxoItem, impl Iterator<Item = UtxoItem>> {
    ScrollableListComponent::new("UTXOs".to_string(), db.iter_utxos().unwrap(), 10)
}

pub fn new_utxo_details_component(
    shared_getter: SharedGetter<UtxoItem>,
) -> DetailsComponent<UtxoItem> {
    DetailsComponent::new("UTXO Details".to_string(), shared_getter)
}

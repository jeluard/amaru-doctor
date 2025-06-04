use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::{Getter, SharedGetter, shared},
};
use amaru_kernel::{TransactionInput, TransactionOutput};
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

type UtxoListEntry = (TransactionInput, TransactionOutput);
type UtxoEnumEntry = (usize, UtxoListEntry);
type UtxoListRenderer = fn(&UtxoEnumEntry) -> ListItem;

pub fn new_utxo_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<UtxoEnumEntry, impl Iterator<Item = UtxoEnumEntry>, UtxoListRenderer> {
    fn render(item: &UtxoEnumEntry) -> ListItem {
        let (i, (input, _)) = item;
        ListItem::new(format!("{}: {}", i, input.transaction_id))
    }

    let iter = db.iter_utxos().unwrap().enumerate();

    ScrollableListComponent::new("UTXOs".to_string(), iter, 10, render)
}

struct MappedGetter<'a> {
    inner: SharedGetter<'a, UtxoEnumEntry>,
}

impl<'a> Getter<UtxoListEntry> for MappedGetter<'a> {
    fn get_mut(&mut self) -> Option<UtxoListEntry> {
        self.inner.borrow_mut().get_mut().map(|(_, val)| val)
    }
}

fn map(shared_getter: SharedGetter<UtxoEnumEntry>) -> SharedGetter<UtxoListEntry> {
    shared(MappedGetter {
        inner: shared_getter,
    })
}

pub fn new_utxo_details_component(
    shared_getter: SharedGetter<UtxoEnumEntry>,
) -> DetailsComponent<UtxoListEntry> {
    DetailsComponent::new("UTXO Details".to_string(), map(shared_getter))
}

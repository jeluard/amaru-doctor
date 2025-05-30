use super::{details::DetailsComponent, scroll::ScrollableListComponent};
use crate::{
    action::SelectedItem,
    to_rich::{RichText, ToRichText},
};
use amaru_kernel::{TransactionInput, TransactionOutput};
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use ratatui::widgets::ListItem;
use std::sync::Arc;

type UtxoListEntry = (TransactionInput, TransactionOutput);
type UtxoEnumEntry = (usize, UtxoListEntry);
type UtxoListSelector = fn(&UtxoEnumEntry) -> Option<SelectedItem>;
type UtxoListRenderer = fn(&UtxoEnumEntry) -> ListItem;

pub fn new_utxo_list_component<'a>(
    db: &'a Arc<RocksDB>,
) -> ScrollableListComponent<
    UtxoEnumEntry,
    impl Iterator<Item = UtxoEnumEntry>,
    UtxoListSelector,
    UtxoListRenderer,
> {
    fn select(item: &UtxoEnumEntry) -> Option<SelectedItem> {
        let (_, (input, _)) = item;
        Some(SelectedItem::Utxo(input.clone()))
    }

    fn render(item: &UtxoEnumEntry) -> ListItem {
        let (i, (input, _)) = item;
        ListItem::new(format!("{}: {}", i, input.transaction_id))
    }

    let iter = db.iter_utxos().unwrap().enumerate();

    ScrollableListComponent::new("UTXOs".to_string(), iter, 10, select, render)
}

pub fn new_utxo_details_component<'a>(
    db: &'a Arc<RocksDB>,
) -> DetailsComponent<TransactionInput, impl Fn(&TransactionInput) -> Result<Option<RichText>> + 'a>
{
    let render = move |key: &TransactionInput| {
        let val = db.utxo(key)?;
        Ok(val.map(|v| (key.clone(), v).into_rich_text()))
    };

    let first_key = db
        .iter_utxos()
        .ok()
        .and_then(|mut i| i.next().map(|(k, _)| k));

    DetailsComponent::new("UTXO Details".to_string(), first_key, render)
}

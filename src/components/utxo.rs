use super::{details::DetailsComponent, scroll::ScrollableListComponent};
use crate::{
    action::SelectedItem,
    to_rich::{RichText, ToRichText},
};
use amaru_kernel::{PseudoTransactionOutput, TransactionInput, Value, alonzo::NativeScript};
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use pallas_primitives::{
    PlutusData,
    babbage::PseudoPostAlonzoTransactionOutput,
    conway::{PseudoDatumOption, PseudoScript},
};
use ratatui::widgets::ListItem;
use std::sync::Arc;

type UtxoListEntry = (
    usize,
    (
        TransactionInput,
        PseudoTransactionOutput<
            PseudoPostAlonzoTransactionOutput<
                Value,
                PseudoDatumOption<PlutusData>,
                PseudoScript<NativeScript>,
            >,
        >,
    ),
);

type UtxoListRenderer = fn(&UtxoListEntry) -> ListItem;
type UtxoListSelector = fn(&UtxoListEntry) -> Option<SelectedItem>;

pub fn new_utxo_list_component<'a>(
    db: &'a Arc<RocksDB>,
) -> ScrollableListComponent<
    UtxoListEntry,
    impl Iterator<Item = UtxoListEntry>,
    UtxoListSelector,
    UtxoListRenderer,
> {
    fn select(item: &UtxoListEntry) -> Option<SelectedItem> {
        let (_, (input, _)) = item;
        Some(SelectedItem::Utxo(input.clone()))
    }

    fn render(item: &UtxoListEntry) -> ListItem {
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

    DetailsComponent::new("UTXO Details".to_string(), render)
}

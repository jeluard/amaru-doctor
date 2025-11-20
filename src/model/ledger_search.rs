use crate::{
    components::{async_list::AsyncListModel, search_list::SearchProvider},
    model::async_provider::AsyncProvider,
    ui::to_list_item::UtxoItem,
};
use amaru_kernel::Address;
use amaru_ledger::store::ReadStore;
use amaru_stores::rocksdb::ReadOnlyRocksDB;
use std::sync::Arc;

pub struct LedgerUtxoProvider {
    pub db: Arc<ReadOnlyRocksDB>,
}

impl SearchProvider<Address, UtxoItem> for LedgerUtxoProvider {
    fn search(&self, address: &Address) -> Option<AsyncListModel<UtxoItem>> {
        let db = self.db.clone();
        let owned_addr = address.clone();

        let provider = AsyncProvider::new(move |tx| {
            if let Ok(iter) = ReadStore::iter_utxos(&*db) {
                let filtered = iter.filter(move |(_, out)| out.address == owned_addr);
                for item in filtered {
                    if tx.blocking_send(item).is_err() {
                        break;
                    }
                }
            }
        });
        Some(AsyncListModel::new("Utxos by Addr", provider))
    }
}

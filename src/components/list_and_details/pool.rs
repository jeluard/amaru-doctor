use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_rich::pool::PoolIdDisplay,
};
use amaru_kernel::PoolId;
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

type PoolListEntry = (PoolId, amaru_ledger::store::columns::pools::Row);
type PoolListRenderer = fn(&PoolListEntry) -> ListItem;

pub fn new_pool_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<PoolListEntry, impl Iterator<Item = PoolListEntry>, PoolListRenderer> {
    fn render(item: &PoolListEntry) -> ListItem {
        let (key, _) = item;
        ListItem::new(format!("{}", PoolIdDisplay(*key)))
    }

    let iter = db.iter_pools().unwrap();

    ScrollableListComponent::new("Pools".to_string(), iter, 10, render)
}

pub fn new_pool_details_component<'a>(
    shared: SharedGetter<PoolListEntry>,
) -> DetailsComponent<PoolListEntry> {
    DetailsComponent::new("Pool Details".to_string(), shared)
}

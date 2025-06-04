use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_list_item::ToListItem,
};
use amaru_ledger::store::{ReadOnlyStore, columns::pools};
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

type PoolItem = (pools::Key, pools::Row);

impl ToListItem for PoolItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(self.0.to_string())
    }
}

pub fn new_pool_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<PoolItem, impl Iterator<Item = PoolItem>> {
    ScrollableListComponent::new("Pools".to_string(), db.iter_pools().unwrap(), 10)
}

pub fn new_pool_details_component(shared: SharedGetter<PoolItem>) -> DetailsComponent<PoolItem> {
    DetailsComponent::new("Pool Details".to_string(), shared)
}

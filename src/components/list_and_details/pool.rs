use crate::{
    action::SelectedItem,
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    to_rich::{RichText, ToRichText, pool::PoolIdDisplay},
};
use amaru_kernel::PoolId;
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use ratatui::widgets::ListItem;
use std::sync::Arc;

type PoolListEntry = (PoolId, amaru_ledger::store::columns::pools::Row);
type PoolListSelector = fn(&PoolListEntry) -> Option<SelectedItem>;
type PoolListRenderer = fn(&PoolListEntry) -> ListItem;

pub fn new_pool_list_component<'a>(
    db: &'a Arc<RocksDB>,
) -> ScrollableListComponent<
    PoolListEntry,
    impl Iterator<Item = PoolListEntry>,
    PoolListSelector,
    PoolListRenderer,
> {
    fn select(item: &PoolListEntry) -> Option<SelectedItem> {
        let (pool_id, _) = item;
        Some(SelectedItem::Pool(pool_id.clone()))
    }

    fn render(item: &PoolListEntry) -> ListItem {
        let (key, _) = item;
        ListItem::new(format!("{}", PoolIdDisplay(key.clone()).to_string()))
    }

    let iter = db.iter_pools().unwrap();

    ScrollableListComponent::new("Pools".to_string(), iter, 10, select, render)
}

pub fn new_pool_details_component<'a>(
    db: &'a Arc<RocksDB>,
) -> DetailsComponent<PoolId, impl Fn(&PoolId) -> Result<Option<RichText>> + 'a> {
    let render = move |key: &PoolId| {
        let val = db.pool(key)?;
        Ok(val.map(|v| (key.clone(), v).into_rich_text()))
    };

    let first_key = db
        .iter_pools()
        .ok()
        .and_then(|mut i| i.next().map(|(k, _)| k));

    DetailsComponent::new("Pool Details".to_string(), first_key, render)
}

use crate::{
    action::SelectedItem,
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
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

pub fn new_pool_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<
    PoolListEntry,
    impl Iterator<Item = PoolListEntry>,
    // PoolListSelector,
    PoolListRenderer,
> {
    // fn select(item: &PoolListEntry) -> Option<SelectedItem> {
    //     let (pool_id, _) = item;
    //     Some(SelectedItem::Pool(*pool_id))
    // }

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
    // let render = |item: &PoolListEntry| Ok(item.into_rich_text()));

    // let first_key = db
    //     .iter_pools()
    //     .ok()
    //     .and_then(|mut i| i.next().map(|(k, _)| k));

    DetailsComponent::new("Pool Details".to_string(), shared)
}

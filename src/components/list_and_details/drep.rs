use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_rich::account::StakeCredentialDisplay,
};
use amaru_ledger::store::{ReadOnlyStore, columns::dreps};
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

pub type DRepItem = (dreps::Key, dreps::Row);
type DRepItemRenderer = fn(&DRepItem) -> ListItem;

pub fn new_drep_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<DRepItem, impl Iterator<Item = DRepItem>, DRepItemRenderer> {
    fn render(item: &DRepItem) -> ListItem {
        let (key, _) = item;
        ListItem::new(format!("{}", StakeCredentialDisplay(key)))
    }

    let iter = db.iter_dreps().unwrap();

    ScrollableListComponent::new("Accounts".to_string(), iter, 10, render)
}

pub fn new_drep_details_component(shared: SharedGetter<DRepItem>) -> DetailsComponent<DRepItem> {
    DetailsComponent::new("DRep Details".to_string(), shared)
}

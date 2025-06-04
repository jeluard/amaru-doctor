use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_list_item::ToListItem,
    to_rich::account::StakeCredentialDisplay,
};
use amaru_ledger::store::{ReadOnlyStore, columns::dreps};
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

pub type DRepItem = (dreps::Key, dreps::Row);

impl ToListItem for DRepItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(StakeCredentialDisplay(&self.0).to_string())
    }
}

pub fn new_drep_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<DRepItem, impl Iterator<Item = DRepItem>> {
    ScrollableListComponent::new("Accounts".to_string(), db.iter_dreps().unwrap(), 10)
}

pub fn new_drep_details_component(shared: SharedGetter<DRepItem>) -> DetailsComponent<DRepItem> {
    DetailsComponent::new("DRep Details".to_string(), shared)
}

use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_list_item::ToListItem,
    to_rich::proposal::ProposalIdDisplay,
};
use amaru_ledger::store::{ReadOnlyStore, columns::proposals};
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

pub type ProposalItem = (proposals::Key, proposals::Row);

impl ToListItem for ProposalItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(ProposalIdDisplay(&self.0).to_string())
    }
}

pub fn new_proposal_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<ProposalItem, impl Iterator<Item = ProposalItem>> {
    ScrollableListComponent::new("Accounts".to_string(), db.iter_proposals().unwrap(), 10)
}

pub fn new_proposal_details_component(
    shared: SharedGetter<ProposalItem>,
) -> DetailsComponent<ProposalItem> {
    DetailsComponent::new("Proposal Details".to_string(), shared)
}

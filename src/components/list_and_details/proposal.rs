use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_rich::proposal::ProposalIdDisplay,
};
use amaru_ledger::store::{ReadOnlyStore, columns::proposals};
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

pub type ProposalItem = (proposals::Key, proposals::Row);
type ProposalItemRenderer = fn(&ProposalItem) -> ListItem;

pub fn new_proposal_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<ProposalItem, impl Iterator<Item = ProposalItem>, ProposalItemRenderer>
{
    fn render(item: &ProposalItem) -> ListItem {
        let (key, _) = item;
        ListItem::new(format!("{}", ProposalIdDisplay(key)))
    }

    let iter = db.iter_proposals().unwrap();

    ScrollableListComponent::new("Accounts".to_string(), iter, 10, render)
}

pub fn new_proposal_details_component(
    shared: SharedGetter<ProposalItem>,
) -> DetailsComponent<ProposalItem> {
    DetailsComponent::new("Proposal Details".to_string(), shared)
}

use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_list_item::ToListItem,
    to_rich::account::StakeCredentialDisplay,
};
use amaru_ledger::store::{ReadOnlyStore, columns::accounts};
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

pub type AccountItem = (accounts::Key, accounts::Row);

impl ToListItem for AccountItem {
    fn to_list_item(&self) -> ListItem<'static> {
        ListItem::new(StakeCredentialDisplay(&self.0).to_string())
    }
}

pub fn new_account_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<AccountItem, impl Iterator<Item = AccountItem>> {
    ScrollableListComponent::new("Accounts".to_string(), db.iter_accounts().unwrap(), 10)
}

pub fn new_account_details_component(
    shared: SharedGetter<AccountItem>,
) -> DetailsComponent<AccountItem> {
    DetailsComponent::new("Account Details".to_string(), shared)
}

use crate::{
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    shared::SharedGetter,
    to_rich::account::StakeCredentialDisplay,
};
use amaru_kernel::StakeCredential;
use amaru_ledger::store::{ReadOnlyStore, columns::accounts};
use amaru_stores::rocksdb::RocksDB;
use ratatui::widgets::ListItem;
use std::sync::Arc;

pub type AccountListEntry = (StakeCredential, accounts::Row);
type AccountListRenderer = fn(&AccountListEntry) -> ListItem;

pub fn new_account_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<
    AccountListEntry,
    impl Iterator<Item = AccountListEntry>,
    AccountListRenderer,
> {
    fn render(item: &AccountListEntry) -> ListItem {
        let (key, _) = item;
        ListItem::new(format!("{}", StakeCredentialDisplay(key)))
    }

    let iter = db.iter_accounts().unwrap();

    ScrollableListComponent::new("Accounts".to_string(), iter, 10, render)
}

pub fn new_account_details_component<'a>(
    shared: SharedGetter<AccountListEntry>,
) -> DetailsComponent<AccountListEntry> {
    DetailsComponent::new("Account Details".to_string(), shared)
}

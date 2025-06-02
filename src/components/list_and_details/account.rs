use crate::{
    action::SelectedItem,
    components::{details::DetailsComponent, group::scroll::ScrollableListComponent},
    to_rich::{RichText, ToRichText, account::StakeCredentialDisplay},
};
use amaru_kernel::StakeCredential;
use amaru_ledger::store::ReadOnlyStore;
use amaru_stores::rocksdb::RocksDB;
use color_eyre::Result;
use ratatui::widgets::ListItem;
use std::sync::Arc;

type AccountListEntry = (StakeCredential, amaru_ledger::store::columns::accounts::Row);
type AccountListSelector = fn(&AccountListEntry) -> Option<SelectedItem>;
type AccountListRenderer = fn(&AccountListEntry) -> ListItem;

pub fn new_account_list_component(
    db: &Arc<RocksDB>,
) -> ScrollableListComponent<
    AccountListEntry,
    impl Iterator<Item = AccountListEntry>,
    AccountListSelector,
    AccountListRenderer,
> {
    fn select(item: &AccountListEntry) -> Option<SelectedItem> {
        let (key, _) = item;
        Some(SelectedItem::Account(key.clone()))
    }

    fn render(item: &AccountListEntry) -> ListItem {
        let (key, _) = item;
        ListItem::new(format!("{}", StakeCredentialDisplay(key.clone())))
    }

    let iter = db.iter_accounts().unwrap();

    ScrollableListComponent::new("Accounts".to_string(), iter, 10, select, render)
}

pub fn new_account_details_component<'a>(
    db: &'a Arc<RocksDB>,
) -> DetailsComponent<StakeCredential, impl Fn(&StakeCredential) -> Result<Option<RichText>> + 'a> {
    let render = move |key: &StakeCredential| {
        let val = db.account(key)?;
        Ok(val.map(|v| (key.clone(), v).into_rich_text()))
    };

    let first_key = db
        .iter_accounts()
        .ok()
        .and_then(|mut i| i.next().map(|(k, _)| k));

    DetailsComponent::new("Account Details".to_string(), first_key, render)
}

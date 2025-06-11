use crate::{
    components::r#static::search_types::SearchOptions,
    cursor::Cursor,
    shared::{Shared, shared},
    states::{BrowseOptions, Slot, Tab},
    store::{
        owned_iter::{
            OwnedAccountsIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::RocksDBSwitch,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
    window::WindowState,
};
use std::sync::Arc;
use strum::IntoEnumIterator;

/// Holds ALL the state for the app. Does not self-mutate.
pub struct AppState {
    pub slot_focus: Shared<Cursor<Slot>>,
    pub tabs: Shared<Cursor<Tab>>,
    // Don't put these in Map, however tempting--it will cause pain with generics and ultimately increases complexity
    pub browse_options: Shared<WindowState<BrowseOptions>>,
    pub search_options: Shared<WindowState<SearchOptions>>,
    pub accounts: Shared<WindowState<AccountItem>>,
    pub block_issuers: Shared<WindowState<BlockIssuerItem>>,
    pub dreps: Shared<WindowState<DRepItem>>,
    pub pools: Shared<WindowState<PoolItem>>,
    pub proposals: Shared<WindowState<ProposalItem>>,
    pub utxos: Shared<WindowState<UtxoItem>>,
}

impl AppState {
    pub fn new(db: Arc<RocksDBSwitch>) -> Self {
        Self {
            slot_focus: shared(Cursor::new(Slot::iter().collect())),
            tabs: shared(Cursor::new(Tab::iter().collect())),
            browse_options: shared(WindowState::new(Box::new(BrowseOptions::iter()))),
            search_options: shared(WindowState::new(Box::new(SearchOptions::iter()))),
            accounts: shared(WindowState::new(Box::new(OwnedAccountsIter::new(
                db.clone(),
            )))),
            block_issuers: shared(WindowState::new(Box::new(OwnedBlockIssuerIter::new(
                db.clone(),
            )))),
            dreps: shared(WindowState::new(Box::new(OwnedDRepIter::new(db.clone())))),
            pools: shared(WindowState::new(Box::new(OwnedPoolIter::new(db.clone())))),
            proposals: shared(WindowState::new(Box::new(OwnedProposalIter::new(
                db.clone(),
            )))),
            utxos: shared(WindowState::new(Box::new(OwnedUtxoIter::new(db.clone())))),
        }
    }
}

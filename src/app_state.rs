use crate::{
    model::{cursor::Cursor, window::WindowState},
    states::{BrowseOptions, SearchOptions, Tab, WidgetSlot},
    store::{
        owned_iter::{
            OwnedAccountsIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter,
            OwnedProposalIter, OwnedUtxoIter,
        },
        rocks_db_switch::RocksDBSwitch,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
};
use std::sync::Arc;
use strum::IntoEnumIterator;

/// Holds ALL the state for the app. Does not self-mutate.
/// Does provide readers / helper calcs
pub struct AppState {
    pub slot_focus: Cursor<WidgetSlot>,
    pub tabs: Cursor<Tab>,
    // Don't put these in Map, however tempting--it will cause pain with generics and ultimately increases complexity
    pub browse_options: WindowState<BrowseOptions>,
    pub search_options: WindowState<SearchOptions>,
    pub accounts: WindowState<AccountItem>,
    pub block_issuers: WindowState<BlockIssuerItem>,
    pub dreps: WindowState<DRepItem>,
    pub pools: WindowState<PoolItem>,
    pub proposals: WindowState<ProposalItem>,
    pub utxos: WindowState<UtxoItem>,
}

impl AppState {
    pub fn new(db: Arc<RocksDBSwitch>) -> Self {
        let browse_options = WindowState::new(Box::new(BrowseOptions::iter()));
        Self {
            slot_focus: Cursor::new(WidgetSlot::iter().collect()),
            tabs: Cursor::new(Tab::iter().collect()),
            browse_options,
            search_options: WindowState::new(Box::new(SearchOptions::iter())),
            accounts: WindowState::new(Box::new(OwnedAccountsIter::new(db.clone()))),
            block_issuers: WindowState::new(Box::new(OwnedBlockIssuerIter::new(db.clone()))),
            dreps: WindowState::new(Box::new(OwnedDRepIter::new(db.clone()))),
            pools: WindowState::new(Box::new(OwnedPoolIter::new(db.clone()))),
            proposals: WindowState::new(Box::new(OwnedProposalIter::new(db.clone()))),
            utxos: WindowState::new(Box::new(OwnedUtxoIter::new(db.clone()))),
        }
    }
}

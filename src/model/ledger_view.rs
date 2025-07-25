use crate::{
    model::window::WindowState,
    states::{LedgerBrowse, LedgerSearch},
    store::owned_iter::{
        OwnedAccountIter, OwnedBlockIssuerIter, OwnedDRepIter, OwnedPoolIter, OwnedProposalIter,
        OwnedUtxoIter,
    },
    ui::to_list_item::{AccountItem, BlockIssuerItem, DRepItem, PoolItem, ProposalItem, UtxoItem},
    update::search::SearchState,
};
use amaru_kernel::Address;
use amaru_stores::rocksdb::ReadOnlyRocksDB;
use std::sync::Arc;
use strum::IntoEnumIterator;

/// Holds all state related to the Ledger view.
pub struct LedgerViewState {
    // Options: types of lists
    pub options_window_size: usize,
    pub browse_options: WindowState<LedgerBrowse>,
    pub search_options: WindowState<LedgerSearch>,

    // Lists: lists to browse for items
    pub list_window_size: usize,
    pub accounts: WindowState<AccountItem>,
    pub block_issuers: WindowState<BlockIssuerItem>,
    pub dreps: WindowState<DRepItem>,
    pub pools: WindowState<PoolItem>,
    pub proposals: WindowState<ProposalItem>,
    pub utxos: WindowState<UtxoItem>,
    pub utxos_by_addr_search: SearchState<Address, WindowState<UtxoItem>>,
}

impl LedgerViewState {
    pub fn new(ledger_db_arc: Arc<ReadOnlyRocksDB>) -> Self {
        Self {
            options_window_size: 0,
            browse_options: WindowState::from_iter(LedgerBrowse::iter()),
            search_options: WindowState::from_iter(LedgerSearch::iter()),
            list_window_size: 0,
            accounts: WindowState::from_iter(OwnedAccountIter::new(ledger_db_arc.clone())),
            block_issuers: WindowState::from_iter(OwnedBlockIssuerIter::new(ledger_db_arc.clone())),
            dreps: WindowState::from_iter(OwnedDRepIter::new(ledger_db_arc.clone())),
            pools: WindowState::from_iter(OwnedPoolIter::new(ledger_db_arc.clone())),
            proposals: WindowState::from_iter(OwnedProposalIter::new(ledger_db_arc.clone())),
            utxos: WindowState::from_iter(OwnedUtxoIter::new(ledger_db_arc)),
            utxos_by_addr_search: SearchState::default(),
        }
    }
}

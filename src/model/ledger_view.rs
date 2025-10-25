use crate::{
    model::list_view::ListModelViewState,
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

/// Holds the model state (underlying data) and view state (ui) for the Ledger
/// page
pub struct LedgerModelViewState {
    // Options: types of lists
    pub options_window_height: usize,
    pub browse_options: ListModelViewState<LedgerBrowse>,
    pub search_options: ListModelViewState<LedgerSearch>,

    // Lists: lists to browse for items
    pub list_window_height: usize,
    pub accounts: ListModelViewState<AccountItem>,
    pub block_issuers: ListModelViewState<BlockIssuerItem>,
    pub dreps: ListModelViewState<DRepItem>,
    pub pools: ListModelViewState<PoolItem>,
    pub proposals: ListModelViewState<ProposalItem>,
    pub utxos: ListModelViewState<UtxoItem>,

    pub utxos_by_addr_search: SearchState<Address, ListModelViewState<UtxoItem>>,
}

impl LedgerModelViewState {
    pub fn new(
        ledger_db_arc: Arc<ReadOnlyRocksDB>,
        options_window_height: usize,
        list_window_height: usize,
    ) -> Self {
        Self {
            options_window_height,
            browse_options: ListModelViewState::new(
                "Browse Options",
                LedgerBrowse::iter(),
                options_window_height,
            ),
            search_options: ListModelViewState::new(
                "Search Options",
                LedgerSearch::iter(),
                options_window_height,
            ),
            list_window_height,
            accounts: ListModelViewState::new(
                "Accounts",
                OwnedAccountIter::new(ledger_db_arc.clone()),
                list_window_height,
            ),
            block_issuers: ListModelViewState::new(
                "Block Issuers",
                OwnedBlockIssuerIter::new(ledger_db_arc.clone()),
                list_window_height,
            ),
            dreps: ListModelViewState::new(
                "DReps",
                OwnedDRepIter::new(ledger_db_arc.clone()),
                list_window_height,
            ),
            pools: ListModelViewState::new(
                "Pools",
                OwnedPoolIter::new(ledger_db_arc.clone()),
                list_window_height,
            ),
            proposals: ListModelViewState::new(
                "Proposals",
                OwnedProposalIter::new(ledger_db_arc.clone()),
                list_window_height,
            ),
            utxos: ListModelViewState::new(
                "Utxos",
                OwnedUtxoIter::new(ledger_db_arc.clone()),
                list_window_height,
            ),
            utxos_by_addr_search: SearchState::default(),
        }
    }
}

use crate::{
    app_state::AppState,
    model::list_view::ListModelViewState,
    states::{InspectOption, LedgerSearch, WidgetSlot},
    store::owned_iter::OwnedUtxoIter,
    ui::to_list_item::UtxoItem,
    update::search::{SearchHandler, SearchState},
};
use amaru_consensus::{BlockHeader, Nonces, ReadOnlyChainStore};
use amaru_kernel::{Address, Hash, RawBlock};
use tracing::trace;

#[derive(Default)]
pub struct LedgerUtxosByAddr;
impl SearchHandler for LedgerUtxosByAddr {
    type Query = Address;
    type Result = ListModelViewState<UtxoItem>;

    fn debug_name(&self) -> &'static str {
        "LedgerUtxosByAddr"
    }

    fn slot(&self) -> WidgetSlot {
        WidgetSlot::SearchBar
    }

    fn is_active(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Ledger
            && s.ledger_mvs.search_options.selected_item() == Some(&LedgerSearch::UtxosByAddress)
    }

    fn state<'a>(&self, s: &'a AppState) -> &'a SearchState<Self::Query, Self::Result> {
        &s.ledger_mvs.utxos_by_addr_search
    }

    fn state_mut<'a>(&self, s: &'a mut AppState) -> &'a mut SearchState<Self::Query, Self::Result> {
        &mut s.ledger_mvs.utxos_by_addr_search
    }

    fn compute(&self, s: &AppState, query: &Self::Query) -> Self::Result {
        let owned_query = query.clone();
        let iter = OwnedUtxoIter::new(s.ledger_db.clone())
            .filter(move |(_, out): &UtxoItem| out.address == owned_query);
        ListModelViewState::new("Utxos by Address", iter, s.ledger_mvs.list_window_height)
    }
}

#[derive(Default)]
pub struct ChainSearch;
impl SearchHandler for ChainSearch {
    type Query = Hash<32>;
    type Result = Option<(BlockHeader, RawBlock, Nonces)>;

    fn debug_name(&self) -> &'static str {
        "ChainSearch"
    }

    fn slot(&self) -> WidgetSlot {
        WidgetSlot::SearchBar
    }

    fn is_active(&self, s: &AppState) -> bool {
        *s.inspect_tabs.cursor.current() == InspectOption::Chain
    }

    fn state<'a>(&self, s: &'a AppState) -> &'a SearchState<Self::Query, Self::Result> {
        &s.chain_view.chain_search
    }

    fn state_mut<'a>(&self, s: &'a mut AppState) -> &'a mut SearchState<Self::Query, Self::Result> {
        &mut s.chain_view.chain_search
    }

    fn compute(&self, s: &AppState, query: &Self::Query) -> Self::Result {
        let header: BlockHeader = match s.chain_db.load_header(query) {
            Some(h) => h,
            None => {
                trace!("{} Found no header for query {}", self.debug_name(), query);
                return None;
            }
        };
        let block = match ReadOnlyChainStore::<BlockHeader>::load_block(&*s.chain_db, query) {
            Ok(b) => b,
            Err(e) => {
                trace!(
                    "{} Error loading block for query {}",
                    self.debug_name(),
                    query
                );
                trace!("{} {}", self.debug_name(), e.to_string());
                return None;
            }
        };
        let nonces = match ReadOnlyChainStore::<BlockHeader>::get_nonces(&*s.chain_db, query) {
            Some(n) => n,
            None => {
                trace!("{} Found no nonces for query {}", self.debug_name(), query);
                return None;
            }
        };
        Some((header, block, nonces))
    }
}

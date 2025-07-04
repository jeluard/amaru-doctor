use crate::{
    app_state::AppState,
    model::window::WindowState,
    states::{InspectOption, LedgerSearchOption, WidgetSlot},
    store::owned_iter::OwnedUtxoIter,
    ui::to_list_item::UtxoItem,
    update::search::{SearchHandler, SearchState},
};
use amaru_consensus::{Nonces, consensus::store::ReadOnlyChainStore};
use amaru_kernel::{Address, HasAddress, Hash, Header, RawBlock};
use tracing::trace;

#[derive(Default)]
pub struct LedgerUtxosByAddr;
impl SearchHandler for LedgerUtxosByAddr {
    type Query = Address;
    type Result = WindowState<UtxoItem>;

    fn debug_name(&self) -> &'static str {
        "LedgerUtxosByAddr"
    }

    fn slot(&self) -> WidgetSlot {
        WidgetSlot::SearchBar
    }

    fn is_active(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Ledger
            && s.ledger_search_options.selected() == Some(&LedgerSearchOption::UtxosByAddress)
    }

    fn state<'a>(&self, s: &'a AppState) -> &'a SearchState<Self::Query, Self::Result> {
        &s.utxos_by_addr_search
    }

    fn state_mut<'a>(&self, s: &'a mut AppState) -> &'a mut SearchState<Self::Query, Self::Result> {
        &mut s.utxos_by_addr_search
    }

    fn compute(&self, s: &AppState, query: &Self::Query) -> Self::Result {
        let owned_query = query.clone();
        let iter = OwnedUtxoIter::new(s.ledger_db.clone()).filter(move |(_, out): &UtxoItem| {
            out.address().ok().is_some_and(|addr| addr == owned_query)
        });
        let mut window = WindowState::from_iter(iter);
        window.set_window_size(s.list_window_size);
        window
    }
}

#[derive(Default)]
pub struct ChainSearch;
impl SearchHandler for ChainSearch {
    type Query = Hash<32>;
    type Result = Option<(Header, RawBlock, Nonces)>;

    fn debug_name(&self) -> &'static str {
        "ChainSearch"
    }

    fn slot(&self) -> WidgetSlot {
        WidgetSlot::SearchBar
    }

    fn is_active(&self, s: &AppState) -> bool {
        *s.inspect_option.current() == InspectOption::Chain
    }

    fn state<'a>(&self, s: &'a AppState) -> &'a SearchState<Self::Query, Self::Result> {
        &s.chain_search
    }

    fn state_mut<'a>(&self, s: &'a mut AppState) -> &'a mut SearchState<Self::Query, Self::Result> {
        &mut s.chain_search
    }

    fn compute(&self, s: &AppState, query: &Self::Query) -> Self::Result {
        let header: Header = match s.chain_db.load_header(query) {
            Some(h) => h,
            None => {
                trace!("{} Found no header for query {}", self.debug_name(), query);
                return None;
            }
        };
        let block = match ReadOnlyChainStore::<Header>::load_block(&*s.chain_db, query) {
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
        let nonces = match ReadOnlyChainStore::<Header>::get_nonces(&*s.chain_db, query) {
            Some(n) => n,
            None => {
                trace!("{} Found no nonces for query {}", self.debug_name(), query);
                return None;
            }
        };
        Some((header, block, nonces))
    }
}

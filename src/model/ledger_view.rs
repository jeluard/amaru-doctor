use crate::{
    model::async_list_model::AsyncListModel, ui::to_list_item::UtxoItem,
    update::search::SearchState,
};
use amaru_kernel::Address;

/// Holds the model state (underlying data) and view state (ui) for the Ledger
/// page
pub struct LedgerModelViewState {
    pub options_window_height: usize,
    pub list_window_height: usize,
    pub utxos_by_addr_search: SearchState<Address, AsyncListModel<UtxoItem>>,
}

impl LedgerModelViewState {
    pub fn new(options_window_height: usize, list_window_height: usize) -> Self {
        Self {
            options_window_height,
            list_window_height,
            utxos_by_addr_search: SearchState::default(),
        }
    }
}

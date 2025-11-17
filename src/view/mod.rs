use crate::{app_state::AppState, states::WidgetSlot, view::defs::*};
use ratatui::{Frame, layout::Rect};
use std::collections::HashMap;

pub mod adapter;
pub mod defs;
pub mod empty_list;
pub mod flame_graph;
pub mod item_details;
pub mod list;
pub mod search;
pub mod span;
pub mod span_bar;

pub trait View: Sync {
    fn slot(&self) -> WidgetSlot;
    fn is_visible(&self, s: &AppState) -> bool;
    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState);
}

/// All views
static VIEW_DEFS: &[&dyn View] = &[
    &INSPECT_TABS_VIEW,
    &LEDGER_MODE_TABS_VIEW,
    &SEARCH_BAR_VIEW,
    &LEDGER_BROWSE_OPTIONS_VIEW,
    &LEDGER_SEARCH_OPTIONS_VIEW,
    &LEDGER_ACCOUNTS_LIST_VIEW,
    &LEDGER_BLOCK_ISSUERS_LIST_VIEW,
    &LEDGER_DREPS_LIST_VIEW,
    &LEDGER_POOLS_LIST_VIEW,
    &LEDGER_PROPOSALS_LIST_VIEW,
    &LEDGER_UTXOS_LIST_VIEW,
    &LedgerUtxosByAddr,
    &LEDGER_ACCOUNT_DETAILS_VIEW,
    &LEDGER_BLOCK_ISSUER_DETAILS_VIEW,
    &LEDGER_DREP_DETAILS_VIEW,
    &LEDGER_POOL_DETAILS_VIEW,
    &LEDGER_PROPOSAL_DETAILS_VIEW,
    &LEDGER_UTXO_DETAILS_VIEW,
    &LedgerSearchUtxoDetails,
    &CHAIN_SEARCH_HEADER_VIEW,
    &CHAIN_SEARCH_BLOCK_VIEW,
    &CHAIN_SEARCH_NONCES_VIEW,
    &TRACE_LIST_VIEW,
    &FlameGraphDetails,
    &SpanDetails,
    &PROM_METRICS_VIEW,
];

pub type SlotViews = HashMap<WidgetSlot, &'static dyn View>;

pub fn compute_slot_views(s: &AppState) -> SlotViews {
    VIEW_DEFS
        .iter()
        .filter(|v| v.is_visible(s))
        .map(|v| (v.slot(), *v))
        .collect()
}

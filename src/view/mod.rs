use crate::{app_state::AppState, states::WidgetSlot, view::defs::*};
use ratatui::{Frame, layout::Rect};
use std::collections::HashMap;

pub mod adapter;
pub mod block;
pub mod defs;
pub mod empty_list;
pub mod flame_graph;
pub mod header;
pub mod item_details;
pub mod list;
pub mod nonces;
pub mod prom_metrics;
pub mod search;
pub mod span;
pub mod span_bar;
pub mod time_series;

pub trait View: Sync {
    fn slot(&self) -> WidgetSlot;
    fn is_visible(&self, s: &AppState) -> bool;
    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState);
}

/// All views
static VIEW_DEFS: &[&dyn View] = &[
    &INSPECT_TABS_VIEW,
    &LEDGER_MODE_TABS_VIEW,
    //&SearchBar,
    &LedgerBrowseOptions,
    //&LedgerSearchOptions,
    &LedgerAccounts,
    &LedgerBlockIssuers,
    &LedgerDReps,
    &LedgerPools,
    &LedgerProposals,
    &LedgerUtxos,
    &LedgerUtxosByAddr,
    &LedgerAccountDetails,
    &LedgerBlockIssuerDetails,
    &LedgerDRepDetails,
    &LedgerPoolDetails,
    &LedgerProposalDetails,
    &LedgerUtxoDetails,
    &LedgerSearchUtxoDetails,
    &ChainSearchHeader,
    &ChainSearchBlock,
    &ChainSearchNonces,
    &TraceList,
    &FlameGraphDetails,
    &SpanDetails,
    &PromMetrics,
];

pub type SlotViews = HashMap<WidgetSlot, &'static dyn View>;

pub fn compute_slot_views(s: &AppState) -> SlotViews {
    VIEW_DEFS
        .iter()
        .filter(|v| v.is_visible(s))
        .map(|v| (v.slot(), *v))
        .collect()
}

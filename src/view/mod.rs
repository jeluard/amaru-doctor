use crate::{app_state::AppState, states::WidgetSlot, view::defs::*};
use anyhow::Result;
use ratatui::{Frame, layout::Rect};
use std::collections::HashMap;

pub mod block;
pub mod defs;
pub mod details;
pub mod flame_graph;
pub mod header;
pub mod line;
pub mod nonces;
pub mod prom_metrics;
pub mod search;
pub mod span;
pub mod span_bar;
pub mod tabs;
pub mod time_series;
pub mod trace_list;
pub mod window;

pub trait View: Sync {
    fn slot(&self) -> WidgetSlot;
    fn is_visible(&self, s: &AppState) -> bool;
    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState) -> Result<()>;
}

/// All views
static VIEW_DEFS: &[&dyn View] = &[
    &InspectTabs,
    &LedgerModeTabs,
    &SearchBar,
    &LedgerBrowseOptions,
    &LedgerSearchOptions,
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
    &BottomLine,
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

use crate::{app_state::AppState, states::WidgetSlot, view::defs::*};
use color_eyre::Result;
use ratatui::{Frame, layout::Rect};
use std::collections::HashMap;

pub mod defs;
pub mod details;
pub mod line;
pub mod list;
pub mod search;
pub mod tabs;

pub trait View: Sync {
    fn slot(&self) -> WidgetSlot;
    fn is_visible(&self, s: &AppState) -> bool;
    fn render(&self, frame: &mut Frame, area: Rect, s: &AppState) -> Result<()>;
}

static VIEW_DEFS: &[&dyn View] = &[
    &TopLine,
    &StoreTabs,
    &LedgerModeTabs,
    &SearchBar,
    &BrowseOptions,
    &SearchOptions,
    &Accounts,
    &BlockIssuers,
    &DReps,
    &Pools,
    &Proposals,
    &Utxos,
    &UtxosByAddrList,
    &AccountDetails,
    &BlockIssuerDetails,
    &DRepDetails,
    &PoolDetails,
    &ProposalDetails,
    &UtxoDetails,
    &SearchUtxoDetails,
    &BottomLine,
];

pub type SlotViews = HashMap<WidgetSlot, &'static dyn View>;

pub fn compute_slot_views(s: &AppState) -> SlotViews {
    VIEW_DEFS
        .iter()
        .filter(|v| v.is_visible(s))
        .map(|v| (v.slot(), *v))
        .collect()
}

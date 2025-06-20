use crate::{
    app_state::AppState,
    states::{BrowseOption, LedgerMode, SearchOption, WidgetSlot},
    view::{
        details::{DetailsView, OptDetailsView},
        empty::EmptyView,
        line::LineView,
        list::{ListView, OptListView},
        search::SearchQueryView,
        tabs::TabsView,
    },
};
use color_eyre::Result;
use ratatui::{Frame, layout::Rect};
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub mod details;
pub mod empty;
pub mod line;
pub mod list;
pub mod search;
pub mod tabs;

pub type SlotViews = HashMap<WidgetSlot, Box<dyn View>>;

pub trait View {
    fn render(&self, frame: &mut Frame, area: Rect, app_state: &AppState) -> Result<()>;
}

pub fn compute_slot_views(app_state: &AppState) -> SlotViews {
    WidgetSlot::iter()
        .map(|slot| (slot, view_for(app_state, slot)))
        .collect()
}

pub fn view_for(app_state: &AppState, widget_slot: WidgetSlot) -> Box<dyn View> {
    match widget_slot {
        WidgetSlot::TopLine => Box::new(LineView::new(|s: &AppState| {
            format!("Reading amaru dir at {}", &s.ledger_path)
        })),
        WidgetSlot::StoreOption => Box::new(TabsView::new("Store", widget_slot, |s: &AppState| {
            &s.store_option
        })),
        WidgetSlot::LedgerMode => Box::new(TabsView::new("Mode", widget_slot, |s: &AppState| {
            &s.ledger_mode
        })),
        WidgetSlot::SearchBar => Box::new(SearchQueryView::new(
            "Search",
            widget_slot,
            |s: &AppState| &s.search_query_bldr,
        )),
        WidgetSlot::Options => match app_state.ledger_mode.current() {
            LedgerMode::Browse => Box::new(ListView::new(
                "Browse Options",
                widget_slot,
                |s: &AppState| &s.browse_options,
            )),
            LedgerMode::Search => {
                Box::new(ListView::new("Queries", widget_slot, |s: &AppState| {
                    &s.search_options
                }))
            }
        },
        WidgetSlot::List => match app_state.ledger_mode.current() {
            LedgerMode::Browse => match app_state.browse_options.selected() {
                Some(browse_opt) => match browse_opt {
                    BrowseOption::Accounts => {
                        Box::new(ListView::new("Accounts", widget_slot, |s: &AppState| {
                            &s.accounts
                        }))
                    }
                    BrowseOption::BlockIssuers => Box::new(ListView::new(
                        "Block Issuers",
                        widget_slot,
                        |s: &AppState| &s.block_issuers,
                    )),
                    BrowseOption::DReps => {
                        Box::new(ListView::new("DReps", widget_slot, |s: &AppState| &s.dreps))
                    }
                    BrowseOption::Pools => {
                        Box::new(ListView::new("Pools", widget_slot, |s: &AppState| &s.pools))
                    }
                    BrowseOption::Proposals => {
                        Box::new(ListView::new("Proposals", widget_slot, |s: &AppState| {
                            &s.proposals
                        }))
                    }
                    BrowseOption::Utxos => {
                        Box::new(ListView::new("Utxos", widget_slot, |s: &AppState| &s.utxos))
                    }
                },
                None => Box::new(EmptyView::new(widget_slot)),
            },
            LedgerMode::Search => match app_state.search_options.selected() {
                Some(search_opt) => match search_opt {
                    SearchOption::UtxosByAddress => Box::new(OptListView::new(
                        "Utxos by Address",
                        widget_slot,
                        |s: &AppState| {
                            s.search_query_addr
                                .as_ref()
                                .and_then(|addr| s.utxos_by_addr_search_res.get(addr))
                        },
                    )),
                },
                None => Box::new(EmptyView::new(widget_slot)),
            },
        },
        WidgetSlot::Details => match app_state.ledger_mode.current() {
            LedgerMode::Browse => match app_state.browse_options.selected() {
                Some(browse_opt) => match browse_opt {
                    BrowseOption::Accounts => Box::new(DetailsView::new(
                        "Account Details",
                        widget_slot,
                        |s: &AppState| &s.accounts,
                    )),
                    BrowseOption::BlockIssuers => Box::new(DetailsView::new(
                        "Block Issuer Details",
                        widget_slot,
                        |s: &AppState| &s.block_issuers,
                    )),
                    BrowseOption::DReps => Box::new(DetailsView::new(
                        "DRep Details",
                        widget_slot,
                        |s: &AppState| &s.dreps,
                    )),
                    BrowseOption::Pools => Box::new(DetailsView::new(
                        "Pool Details",
                        widget_slot,
                        |s: &AppState| &s.pools,
                    )),
                    BrowseOption::Proposals => Box::new(DetailsView::new(
                        "Proposal Details",
                        widget_slot,
                        |s: &AppState| &s.proposals,
                    )),
                    BrowseOption::Utxos => Box::new(DetailsView::new(
                        "Utxo Details",
                        widget_slot,
                        |s: &AppState| &s.utxos,
                    )),
                },
                None => Box::new(EmptyView::new(widget_slot)),
            },
            LedgerMode::Search => match app_state.search_options.selected() {
                Some(search_opt) => match search_opt {
                    SearchOption::UtxosByAddress => Box::new(OptDetailsView::new(
                        "Utxo Details",
                        widget_slot,
                        |s: &AppState| {
                            s.search_query_addr
                                .as_ref()
                                .and_then(|a| s.utxos_by_addr_search_res.get(a))
                        },
                    )),
                },
                None => Box::new(EmptyView::new(widget_slot)),
            },
        },
        WidgetSlot::BottomLine => Box::new(LineView::new(|_s: &AppState| {
            "Use shift + arrow keys to move focus. Use arrow keys to scroll.".to_owned()
        })),
    }
}

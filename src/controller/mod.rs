use crate::{
    app_state::AppState,
    states::{
        BrowseOption::*,
        SearchOption,
        TabOption::*,
        WidgetId::{self, *},
        WidgetSlot::{self},
    },
};

pub mod layout;

pub fn is_widget_focused(app_state: &AppState, widget_id: &WidgetId) -> bool {
    focused_widget_id(app_state) == *widget_id
}

pub fn focused_widget_id(app_state: &AppState) -> WidgetId {
    let slot = app_state.slot_focus.current();
    resolve_placed_widget_id(app_state, *slot)
}

pub fn resolve_placed_widget_id(app_state: &AppState, slot: WidgetSlot) -> WidgetId {
    match slot {
        WidgetSlot::Header => Header,
        WidgetSlot::Footer => Footer,
        WidgetSlot::Nav => Nav,
        WidgetSlot::SearchBar => match app_state.tabs.current() {
            Browse => Empty,
            Search => SearchQuery,
        },
        WidgetSlot::Options => match app_state.tabs.current() {
            Browse => BrowseOptions,
            Search => SearchOptions,
        },
        WidgetSlot::List => match app_state.tabs.current() {
            Browse => match app_state.browse_options.selected() {
                Accounts => ListAccounts,
                BlockIssuers => ListBlockIssuers,
                DReps => ListDReps,
                Pools => ListPools,
                Proposals => ListProposals,
                Utxos => ListUtxos,
            },
            Search => match app_state.search_options.selected() {
                SearchOption::UtxosByAddress => ListUtxosByAddr,
            },
        },
        WidgetSlot::Details => match app_state.browse_options.selected() {
            Accounts => DetailsAccount,
            BlockIssuers => DetailsBlockIssuer,
            DReps => DetailsDRep,
            Pools => DetailsPool,
            Proposals => DetailsProposal,
            Utxos => DetailsUtxo,
        },
    }
}

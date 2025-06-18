use crate::{
    app_state::AppState,
    states::{
        BrowseOption::*,
        LedgerMode::*,
        SearchOption::*,
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
        WidgetSlot::TopLine => TopInfo,
        WidgetSlot::BottomLine => BottomInfo,
        WidgetSlot::StoreOption => StoreOption,
        WidgetSlot::LedgerMode => LedgerMode,
        WidgetSlot::SearchBar => match app_state.ledger_mode.current() {
            Browse => Empty,
            Search => SearchQuery,
        },
        WidgetSlot::Options => match app_state.ledger_mode.current() {
            Browse => BrowseOptions,
            Search => SearchOptions,
        },
        WidgetSlot::List => match app_state.ledger_mode.current() {
            Browse => match app_state.browse_options.selected() {
                Some(Accounts) => ListAccounts,
                Some(BlockIssuers) => ListBlockIssuers,
                Some(DReps) => ListDReps,
                Some(Pools) => ListPools,
                Some(Proposals) => ListProposals,
                Some(Utxos) => ListUtxos,
                None => Empty,
            },
            Search => match app_state.search_options.selected() {
                Some(UtxosByAddress) => ListUtxosByAddr,
                None => Empty,
            },
        },
        WidgetSlot::Details => match app_state.browse_options.selected() {
            Some(Accounts) => DetailsAccount,
            Some(BlockIssuers) => DetailsBlockIssuer,
            Some(DReps) => DetailsDRep,
            Some(Pools) => DetailsPool,
            Some(Proposals) => DetailsProposal,
            Some(Utxos) => DetailsUtxo,
            None => Empty,
        },
    }
}

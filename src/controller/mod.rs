use crate::{
    app_state::AppState,
    states::{
        BrowseOption::*,
        TabOption::*,
        WidgetId::{self, *},
        WidgetSlot::{self, *},
    },
};

pub mod layout;

pub fn is_widget_focused(app_state: &AppState, widget_id: &WidgetId) -> bool {
    get_focused_widget(app_state) == Some(widget_id.clone())
}

pub fn get_focused_widget(app_state: &AppState) -> Option<WidgetId> {
    app_state
        .slot_focus
        .current()
        .and_then(|s| get_selected_widget_id(app_state, s))
}

pub fn get_selected_widget_id(app_state: &AppState, slot: &WidgetSlot) -> Option<WidgetId> {
    match slot {
        Tabs => Some(WidgetId::CursorTabs),
        Options => match app_state.tabs.current() {
            Some(Browse) => Some(ListBrowseOptions),
            Some(Search) => Some(ListSearchOptions),
            None => None,
        },
        WidgetSlot::List => match app_state.browse_options.selected() {
            Some(Accounts) => Some(ListAccounts),
            Some(BlockIssuers) => Some(ListBlockIssuers),
            Some(DReps) => Some(ListDReps),
            Some(Pools) => Some(ListPools),
            Some(Proposals) => Some(ListProposals),
            Some(Utxos) => Some(ListUtxos),
            None => None,
        },
        WidgetSlot::Details => match app_state.browse_options.selected() {
            Some(Accounts) => Some(DetailsAccount),
            Some(BlockIssuers) => Some(DetailsBlockIssuer),
            Some(DReps) => Some(DetailsDRep),
            Some(Pools) => Some(DetailsPool),
            Some(Proposals) => Some(DetailsProposal),
            Some(Utxos) => Some(DetailsUtxo),
            None => None,
        },
    }
}

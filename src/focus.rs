use crate::{
    app_state::AppState,
    shared::Shared,
    states::{BrowseOptions, Tab, Slot, WidgetId},
};

pub fn get_focused(app_state: Shared<AppState>) -> Option<WidgetId> {
    app_state
        .borrow()
        .slot_focus
        .borrow()
        .current()
        .and_then(|s| get_slot_selection(app_state.clone(), s.clone()))
}

pub fn get_slot_selection(app_state: Shared<AppState>, slot: Slot) -> Option<WidgetId> {
    match slot {
        Slot::Nav => Some(WidgetId::ListTabs),
        Slot::NavType => match app_state.borrow().tabs.borrow().current() {
            Some(Tab::Browse) => Some(WidgetId::ListBrowseOptions),
            Some(Tab::Search) => Some(WidgetId::ListSearchOptions),
            None => None,
        },
        Slot::List => match app_state.borrow().browse_options.borrow().selected() {
            Some(BrowseOptions::Accounts) => Some(WidgetId::ListAccounts),
            Some(BrowseOptions::BlockIssuers) => Some(WidgetId::ListBlockIssuers),
            Some(BrowseOptions::DReps) => Some(WidgetId::ListDReps),
            Some(BrowseOptions::Pools) => Some(WidgetId::ListPools),
            Some(BrowseOptions::Proposals) => Some(WidgetId::ListProposals),
            Some(BrowseOptions::Utxos) => Some(WidgetId::ListUtxos),
            None => None,
        },
        Slot::Details => match app_state.borrow().browse_options.borrow().selected() {
            Some(BrowseOptions::Accounts) => Some(WidgetId::DetailAccount),
            Some(BrowseOptions::BlockIssuers) => Some(WidgetId::DetailBlockIssuer),
            Some(BrowseOptions::DReps) => Some(WidgetId::DetailDRep),
            Some(BrowseOptions::Pools) => Some(WidgetId::DetailPool),
            Some(BrowseOptions::Proposals) => Some(WidgetId::DetailProposal),
            Some(BrowseOptions::Utxos) => Some(WidgetId::DetailUtxo),
            None => None,
        },
    }
}

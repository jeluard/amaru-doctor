use crate::{
    app_state::AppState,
    states::{
        Action::{self, *},
        BrowseOption::*,
        TabOption::*,
        WidgetSlot::{self},
    },
    update::Update,
};

pub struct ScrollUpdate {}

impl Update for ScrollUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) {
        let Some(slot) = app_state.slot_focus.current() else {
            return;
        };

        match (action, slot) {
            (ScrollUp, WidgetSlot::Tabs) => {
                app_state.tabs.next_back();
            }
            (ScrollDown, WidgetSlot::Tabs) => {
                app_state.tabs.next();
            }

            (ScrollUp, WidgetSlot::Options) => match app_state.tabs.current() {
                Some(Browse) => app_state.browse_options.scroll_up(),
                Some(Search) => app_state.search_options.scroll_up(),
                _ => {}
            },
            (ScrollDown, WidgetSlot::Options) => match app_state.tabs.current() {
                Some(Browse) => app_state.browse_options.scroll_down(),
                Some(Search) => app_state.search_options.scroll_down(),
                _ => {}
            },

            (ScrollUp, WidgetSlot::List) => match app_state.browse_options.selected() {
                Some(Accounts) => app_state.accounts.scroll_up(),
                Some(BlockIssuers) => app_state.block_issuers.scroll_up(),
                Some(DReps) => app_state.dreps.scroll_up(),
                Some(Pools) => app_state.pools.scroll_up(),
                Some(Proposals) => app_state.proposals.scroll_up(),
                Some(Utxos) => app_state.utxos.scroll_up(),
                _ => {}
            },
            (ScrollDown, WidgetSlot::List) => match app_state.browse_options.selected() {
                Some(Accounts) => app_state.accounts.scroll_down(),
                Some(BlockIssuers) => app_state.block_issuers.scroll_down(),
                Some(DReps) => app_state.dreps.scroll_down(),
                Some(Pools) => app_state.pools.scroll_down(),
                Some(Proposals) => app_state.proposals.scroll_down(),
                Some(Utxos) => app_state.utxos.scroll_down(),
                _ => {}
            },

            // TODO: Add offset state to AppState
            // (ScrollUp, WidgetSlot::Details) => match app_state.browse_options.selected() {
            //     Some(Accounts) => app_state.accounts.scroll_up(),
            //     Some(BlockIssuers) => app_state.block_issuers.scroll_up(),
            //     Some(DReps) => app_state.dreps.scroll_up(),
            //     Some(Pools) => app_state.pools.scroll_up(),
            //     Some(Proposals) => app_state.proposals.scroll_up(),
            //     Some(Utxos) => app_state.utxos.scroll_up(),
            //     _ => {}
            // },

            // (ScrollDown, WidgetSlot::Details) => match app_state.browse_options.selected() {
            //     Some(Accounts) => app_state.accounts.scroll_down(),
            //     Some(BlockIssuers) => app_state.block_issuers.scroll_down(),
            //     Some(DReps) => app_state.dreps.scroll_down(),
            //     Some(Pools) => app_state.pools.scroll_down(),
            //     Some(Proposals) => app_state.proposals.scroll_down(),
            //     Some(Utxos) => app_state.utxos.scroll_down(),
            //     _ => {}
            // },
            _ => {}
        }
    }
}

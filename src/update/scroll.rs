use crate::{
    app_state::AppState,
    states::{
        Action::{self, *},
        BrowseOption::*,
        SearchOption::*,
        TabOption::*,
        WidgetSlot::{self},
    },
    update::Update,
};

pub struct ScrollUpdate {}

impl Update for ScrollUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action> {
        match (action, app_state.slot_focus.current()) {
            (ScrollUp, WidgetSlot::Nav) => {
                app_state.tabs.next_back();
            }
            (ScrollDown, WidgetSlot::Nav) => {
                app_state.tabs.next();
            }
            (ScrollUp, WidgetSlot::Options) => match app_state.tabs.current() {
                Browse => app_state.browse_options.scroll_up(),
                Search => app_state.search_options.scroll_up(),
            },
            (ScrollDown, WidgetSlot::Options) => match app_state.tabs.current() {
                Browse => app_state.browse_options.scroll_down(),
                Search => app_state.search_options.scroll_down(),
            },
            (ScrollUp, WidgetSlot::List) => match app_state.tabs.current() {
                Browse => match app_state.browse_options.selected() {
                    Accounts => app_state.accounts.scroll_up(),
                    BlockIssuers => app_state.block_issuers.scroll_up(),
                    DReps => app_state.dreps.scroll_up(),
                    Pools => app_state.pools.scroll_up(),
                    Proposals => app_state.proposals.scroll_up(),
                    Utxos => app_state.utxos.scroll_up(),
                },
                Search => match app_state.search_options.selected() {
                    UtxosByAddress => {
                        if let Some(addr) = &app_state.search_query_addr {
                            if let Some(state) = app_state.utxos_by_addr_search_res.get_mut(addr) {
                                state.scroll_up();
                            }
                        }
                    }
                },
            },
            (ScrollDown, WidgetSlot::List) => match app_state.tabs.current() {
                Browse => match app_state.browse_options.selected() {
                    Accounts => app_state.accounts.scroll_down(),
                    BlockIssuers => app_state.block_issuers.scroll_down(),
                    DReps => app_state.dreps.scroll_down(),
                    Pools => app_state.pools.scroll_down(),
                    Proposals => app_state.proposals.scroll_down(),
                    Utxos => app_state.utxos.scroll_down(),
                },
                Search => match app_state.search_options.selected() {
                    UtxosByAddress => {
                        if let Some(addr) = &app_state.search_query_addr {
                            if let Some(state) = app_state.utxos_by_addr_search_res.get_mut(addr) {
                                state.scroll_down();
                            }
                        }
                    }
                },
            },

            // TODO: Add Details scroll offset to AppState
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
        None
    }
}

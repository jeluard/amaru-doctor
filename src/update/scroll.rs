use strum::Display;
use tracing::trace;

use crate::{
    app_state::AppState,
    states::{
        Action::{self, *},
        BrowseOption::*,
        LedgerMode::*,
        WidgetSlot::{self},
    },
    update::Update,
};

pub struct ScrollUpdate {}

#[derive(Display)]
pub enum ScrollDirection {
    Up,
    Down,
}

pub trait Scrollable {
    fn scroll_up(&mut self);
    fn scroll_down(&mut self);
}

impl Update for ScrollUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action> {
        let direction = match action {
            ScrollUp => ScrollDirection::Up,
            ScrollDown => ScrollDirection::Down,
            _ => return None,
        };

        let widget_slot = app_state.slot_focus.current();
        trace!("Will scroll {} {}", widget_slot, direction);
        match (widget_slot, app_state.ledger_mode.current()) {
            (WidgetSlot::StoreOption, _) => scroll_store_option(app_state, direction),
            (WidgetSlot::LedgerMode, _) => scroll_ledger_mode(app_state, direction),
            (WidgetSlot::Options, Browse) => scroll_browse_options(app_state, direction),
            (WidgetSlot::Options, Search) => scroll_search_options(app_state, direction),
            (WidgetSlot::List, Browse) => scroll_browse_list(app_state, direction),
            (WidgetSlot::List, Search) => scroll_search_list(app_state, direction),
            // TODO: Add Details scroll offset to AppState
            // (WidgetSlot::Details, Search) => ...
            _ => {}
        }
        None
    }
}

fn scroll_store_option(state: &mut AppState, direction: ScrollDirection) {
    trace!("Will scroll ledger mode {}", direction);
    match direction {
        ScrollDirection::Up => state.store_option.scroll_up(),
        ScrollDirection::Down => state.store_option.scroll_down(),
    }
}

fn scroll_ledger_mode(state: &mut AppState, direction: ScrollDirection) {
    trace!("Will scroll ledger mode {}", direction);
    match direction {
        ScrollDirection::Up => state.ledger_mode.scroll_up(),
        ScrollDirection::Down => state.ledger_mode.scroll_down(),
    }
}

fn scroll_browse_options(state: &mut AppState, direction: ScrollDirection) {
    trace!("Will browse options {}", direction);
    match direction {
        ScrollDirection::Up => state.browse_options.scroll_up(),
        ScrollDirection::Down => state.browse_options.scroll_down(),
    }
}

fn scroll_search_options(state: &mut AppState, direction: ScrollDirection) {
    trace!("Will scroll search options {}", direction);
    match direction {
        ScrollDirection::Up => state.search_options.scroll_up(),
        ScrollDirection::Down => state.search_options.scroll_down(),
    }
}

fn scroll_browse_list(state: &mut AppState, direction: ScrollDirection) {
    trace!("Will scroll browse list {}", direction);
    match state.browse_options.selected() {
        Accounts => apply_scroll(direction, &mut state.accounts),
        BlockIssuers => apply_scroll(direction, &mut state.block_issuers),
        DReps => apply_scroll(direction, &mut state.dreps),
        Pools => apply_scroll(direction, &mut state.pools),
        Proposals => apply_scroll(direction, &mut state.proposals),
        Utxos => apply_scroll(direction, &mut state.utxos),
    }
}

fn scroll_search_list(state: &mut AppState, direction: ScrollDirection) {
    trace!("Will scroll search list {}", direction);
    // UtxosByAddress is the only search option
    if let Some(addr) = &state.search_query_addr {
        if let Some(utxo_state) = state.utxos_by_addr_search_res.get_mut(addr) {
            apply_scroll(direction, utxo_state);
        }
    }
}

fn apply_scroll<T: Scrollable>(direction: ScrollDirection, target: &mut T) {
    match direction {
        ScrollDirection::Up => target.scroll_up(),
        ScrollDirection::Down => target.scroll_down(),
    }
}

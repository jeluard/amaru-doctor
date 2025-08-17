use crate::{
    app_state::AppState,
    model::{cursor::Cursor, ledger_view::LedgerViewState, window::WindowState},
    states::{Action, LedgerBrowse, LedgerMode, WidgetSlot},
    update::Update,
};
use strum::Display;
use tracing::trace;

#[derive(Display, Debug, Clone, Copy)]
pub enum ScrollDirection {
    Up,
    Down,
}

pub trait ScrollableList {
    fn scroll(&mut self, direction: ScrollDirection);
}

impl<T> ScrollableList for WindowState<T> {
    fn scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Up => WindowState::scroll_up(self),
            ScrollDirection::Down => WindowState::scroll_down(self),
        }
    }
}

impl<T> ScrollableList for Cursor<T> {
    fn scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Up => {
                Cursor::next_back(self);
            }
            ScrollDirection::Down => {
                Cursor::next(self);
            }
        }
    }
}

pub struct ScrollUpdate;
impl Update for ScrollUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let direction = match action {
            Action::ScrollUp => ScrollDirection::Up,
            Action::ScrollDown => ScrollDirection::Down,
            _ => return Vec::new(),
        };

        let focused_slot = s.slot_focus;
        trace!("Scrolling {:?} {:?}", focused_slot, direction);

        match focused_slot {
            WidgetSlot::InspectOption => {
                s.inspect_option.scroll(direction);
                Vec::new()
            }
            WidgetSlot::LedgerMode => {
                // Tab selection should not change on scroll, only on click
                // Scrolling in this widget is disabled to prevent accidental tab changes
                Vec::new()
            }
            WidgetSlot::LedgerOptions | WidgetSlot::LedgerList => {
                let mode = s.ledger_mode.current();
                scroll_ledger_view(&mut s.ledger_view, direction, mode, focused_slot);
                Vec::new()
            }
            WidgetSlot::Details => {
                // TODO: Item details may be scrolled.
                Vec::new()
            }
            _ => {
                trace!("No scroll logic for slot {:?}", focused_slot);
                Vec::new()
            }
        }
    }
}

fn scroll_ledger_view(
    ledger_view: &mut LedgerViewState,
    direction: ScrollDirection,
    mode: &LedgerMode,
    focused_slot: WidgetSlot,
) {
    match focused_slot {
        WidgetSlot::LedgerOptions => {
            let options_list: &mut dyn ScrollableList = match mode {
                LedgerMode::Browse => &mut ledger_view.browse_options,
                LedgerMode::Search => &mut ledger_view.search_options,
            };

            options_list.scroll(direction);
        }
        WidgetSlot::LedgerList => {
            let list_to_scroll: Option<&mut dyn ScrollableList> = match mode {
                LedgerMode::Browse => match ledger_view.browse_options.selected() {
                    Some(LedgerBrowse::Accounts) => Some(&mut ledger_view.accounts),
                    Some(LedgerBrowse::BlockIssuers) => Some(&mut ledger_view.block_issuers),
                    Some(LedgerBrowse::DReps) => Some(&mut ledger_view.dreps),
                    Some(LedgerBrowse::Pools) => Some(&mut ledger_view.pools),
                    Some(LedgerBrowse::Proposals) => Some(&mut ledger_view.proposals),
                    Some(LedgerBrowse::Utxos) => Some(&mut ledger_view.utxos),
                    None => None,
                },
                LedgerMode::Search => ledger_view
                    .utxos_by_addr_search
                    .get_current_res_mut()
                    .map(|r| r as &mut dyn ScrollableList),
            };

            if let Some(list) = list_to_scroll {
                list.scroll(direction);
            }
        }
        _ => {}
    }
}

use crate::{
    app_state::AppState,
    model::{cursor::Cursor, ledger_view::LedgerViewState, window::WindowState},
    states::{Action, InspectOption, LedgerBrowse, LedgerMode, WidgetSlot},
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
            ScrollDirection::Up => self.scroll_up(),
            ScrollDirection::Down => self.scroll_down(),
        }
    }
}

impl<T> ScrollableList for Cursor<T> {
    fn scroll(&mut self, direction: ScrollDirection) {
        match direction {
            ScrollDirection::Up => {
                self.next_back();
            }
            ScrollDirection::Down => {
                self.next();
            }
        }
    }
}

pub struct ScrollUpdate;

impl Update for ScrollUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Some(direction) = (match action {
            Action::ScrollUp => Some(ScrollDirection::Up),
            Action::ScrollDown => Some(ScrollDirection::Down),
            _ => None,
        }) else {
            return Vec::new();
        };

        trace!("Scrolling {:?} {:?}", s.slot_focus, direction);

        match s.slot_focus {
            WidgetSlot::InspectOption => s.inspect_option.scroll(direction),
            WidgetSlot::LedgerMode => {
                s.ledger_mode.scroll(direction);
                // This widget triggers a layout update on scroll.
                return vec![Action::UpdateLayout(s.frame_area)];
            }
            WidgetSlot::LedgerOptions | WidgetSlot::List => {
                let mode = s.ledger_mode.current();
                match s.inspect_option.current() {
                    InspectOption::Ledger => {
                        scroll_ledger_view(&mut s.ledger_view, direction, mode, s.slot_focus)
                    }
                    InspectOption::Otel => {}
                    InspectOption::Chain => {}
                }
            }
            WidgetSlot::Details => match s.inspect_option.current() {
                InspectOption::Otel => {}
                InspectOption::Ledger => { /* TODO: Impl item details scroll. */ }
                InspectOption::Chain => {}
            },
            _ => trace!("No scroll logic for slot {:?}", s.slot_focus),
        }
        Vec::new()
    }
}

/// Determines which list in the ledger view to scroll and scrolls it.
fn scroll_ledger_view(
    ledger_view: &mut LedgerViewState,
    direction: ScrollDirection,
    mode: &LedgerMode,
    focused_slot: WidgetSlot,
) {
    if let Some(list) = get_scrollable_ledger_list(ledger_view, mode, focused_slot) {
        list.scroll(direction);
    }
}

/// Helper to get a mutable reference to the currently active scrollable list in the ledger.
fn get_scrollable_ledger_list<'a>(
    ledger_view: &'a mut LedgerViewState,
    mode: &LedgerMode,
    focused_slot: WidgetSlot,
) -> Option<&'a mut dyn ScrollableList> {
    match focused_slot {
        WidgetSlot::LedgerOptions => match mode {
            LedgerMode::Browse => Some(&mut ledger_view.browse_options),
            LedgerMode::Search => Some(&mut ledger_view.search_options),
        },
        WidgetSlot::List => match mode {
            LedgerMode::Browse => match ledger_view.browse_options.selected()? {
                LedgerBrowse::Accounts => Some(&mut ledger_view.accounts),
                LedgerBrowse::BlockIssuers => Some(&mut ledger_view.block_issuers),
                LedgerBrowse::DReps => Some(&mut ledger_view.dreps),
                LedgerBrowse::Pools => Some(&mut ledger_view.pools),
                LedgerBrowse::Proposals => Some(&mut ledger_view.proposals),
                LedgerBrowse::Utxos => Some(&mut ledger_view.utxos),
            },
            LedgerMode::Search => ledger_view
                .utxos_by_addr_search
                .get_current_res_mut()
                .map(|r| r as &mut dyn ScrollableList),
        },
        _ => None,
    }
}

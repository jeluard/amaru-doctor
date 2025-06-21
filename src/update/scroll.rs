use crate::{
    app_state::AppState,
    states::{Action, BrowseOption, LedgerMode, StoreOption, WidgetSlot},
    update::Update,
};
use strum::Display;
use tracing::trace;

#[derive(Display, Debug)]
pub enum ScrollDirection {
    Up,
    Down,
}

pub struct ScrollUpdate;

pub trait Scrollable {
    fn scroll_up(&mut self);
    fn scroll_down(&mut self);
}

struct ScrollDef {
    slot: WidgetSlot,
    target: fn(&mut AppState) -> Option<&mut dyn Scrollable>,
}

static SCROLL_DEFS: &[ScrollDef] = &[
    ScrollDef {
        slot: WidgetSlot::StoreOption,
        target: |s| Some(&mut s.store_option),
    },
    ScrollDef {
        slot: WidgetSlot::LedgerMode,
        target: |s| Some(&mut s.ledger_mode),
    },
    ScrollDef {
        slot: WidgetSlot::Options,
        target: |s| match s.store_option.current() {
            StoreOption::Ledger => match s.ledger_mode.current() {
                LedgerMode::Browse => Some(&mut s.ledger_browse_options),
                LedgerMode::Search => Some(&mut s.ledger_search_options),
            },
            StoreOption::Chain => Some(&mut s.chain_search_options),
        },
    },
    ScrollDef {
        slot: WidgetSlot::List,
        target: |s| {
            if *s.store_option.current() == StoreOption::Ledger {
                match s.ledger_mode.current() {
                    LedgerMode::Browse => match s.ledger_browse_options.selected() {
                        Some(BrowseOption::Accounts) => Some(&mut s.accounts),
                        Some(BrowseOption::BlockIssuers) => Some(&mut s.block_issuers),
                        Some(BrowseOption::DReps) => Some(&mut s.dreps),
                        Some(BrowseOption::Pools) => Some(&mut s.pools),
                        Some(BrowseOption::Proposals) => Some(&mut s.proposals),
                        Some(BrowseOption::Utxos) => Some(&mut s.utxos),
                        None => None,
                    },
                    LedgerMode::Search => s
                        .ledger_search_query_addr
                        .as_ref()
                        .and_then(|a| s.utxos_by_addr_search_res.get_mut(a))
                        .map(|w| w as &mut dyn Scrollable),
                }
            } else {
                None
            }
        },
    },
    ScrollDef {
        slot: WidgetSlot::Details,
        target: |_| None,
    },
    ScrollDef {
        slot: WidgetSlot::BottomLine,
        target: |_| None,
    },
];

impl Update for ScrollUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Option<Action> {
        let direction = match action {
            Action::ScrollUp => ScrollDirection::Up,
            Action::ScrollDown => ScrollDirection::Down,
            _ => return None,
        };

        let slot = *app_state.slot_focus.current();
        let def = match SCROLL_DEFS.iter().find(|d| d.slot == slot) {
            Some(d) => d,
            None => {
                trace!("No scroll def found for slot {:?}", slot);
                return None;
            }
        };

        let scrollable = (def.target)(app_state)?;

        trace!("Scrolling {:?} {:?}", slot, direction);
        match direction {
            ScrollDirection::Up => scrollable.scroll_up(),
            ScrollDirection::Down => scrollable.scroll_down(),
        }

        None
    }
}

use crate::{
    app_state::AppState,
    states::{Action, BrowseOption, InspectOption, LedgerMode, WidgetSlot},
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
    next_action: fn(&AppState) -> Vec<Action>,
}

// TODO: Change this--follow the pattern in Search
static SCROLL_DEFS: &[ScrollDef] = &[
    ScrollDef {
        slot: WidgetSlot::InspectOption,
        target: |s| Some(&mut s.inspect_option),
        next_action: |_| Vec::new(),
    },
    ScrollDef {
        slot: WidgetSlot::LedgerMode,
        target: |s| Some(&mut s.ledger_mode),
        next_action: |s| vec![Action::UpdateLayout(s.frame_area)],
    },
    ScrollDef {
        slot: WidgetSlot::Options,
        target: |s| match s.inspect_option.current() {
            InspectOption::Ledger => match s.ledger_mode.current() {
                LedgerMode::Browse => Some(&mut s.ledger_browse_options),
                LedgerMode::Search => Some(&mut s.ledger_search_options),
            },
            _ => None,
        },
        next_action: |_| Vec::new(),
    },
    ScrollDef {
        slot: WidgetSlot::List,
        target: |s| {
            if *s.inspect_option.current() == InspectOption::Ledger {
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
                        .utxos_by_addr_search
                        .get_current_res_mut()
                        .map(|r| r as &mut dyn Scrollable),
                }
            } else {
                None
            }
        },
        next_action: |_| Vec::new(),
    },
    ScrollDef {
        slot: WidgetSlot::Details,
        target: |_| None,
        next_action: |_| Vec::new(),
    },
    ScrollDef {
        slot: WidgetSlot::BottomLine,
        target: |_| None,
        next_action: |_| Vec::new(),
    },
];

impl Update for ScrollUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let direction = match action {
            Action::ScrollUp => ScrollDirection::Up,
            Action::ScrollDown => ScrollDirection::Down,
            _ => return Vec::new(),
        };

        let focused_slot = s.slot_focus;
        let def = match SCROLL_DEFS.iter().find(|d| d.slot == focused_slot) {
            Some(d) => d,
            None => {
                trace!("No scroll def found for slot {:?}", focused_slot);
                return Vec::new();
            }
        };

        let scrollable = match (def.target)(s) {
            Some(s) => s,
            None => return Vec::new(),
        };

        trace!("Scrolling {:?} {:?}", focused_slot, direction);

        match direction {
            ScrollDirection::Up => {
                scrollable.scroll_up();
                (def.next_action)(s)
            }
            ScrollDirection::Down => {
                scrollable.scroll_down();
                (def.next_action)(s)
            }
        }
    }
}

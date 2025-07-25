use crate::{
    app_state::AppState,
    states::{
        Action,
        WidgetSlot::{self},
    },
    update::Update,
};

pub struct WindowSizeUpdate;

struct WindowSizeDef {
    slot: WidgetSlot,
    handlers: &'static [fn(&mut AppState, usize)],
}

static WINDOW_DEFS: &[WindowSizeDef] = &[
    WindowSizeDef {
        slot: WidgetSlot::LedgerOptions,
        handlers: &[
            |s, size| {
                s.ledger_view.options_window_size = size;
            },
            |s, size| {
                s.ledger_view.browse_options.set_window_size(size);
            },
            |s, size| {
                s.ledger_view.search_options.set_window_size(size);
            },
        ],
    },
    WindowSizeDef {
        slot: WidgetSlot::LedgerList,
        handlers: &[
            |s, size| {
                s.ledger_view.list_window_size = size;
            },
            |s, size| {
                s.ledger_view.accounts.set_window_size(size);
            },
            |s, size| {
                s.ledger_view.block_issuers.set_window_size(size);
            },
            |s, size| {
                s.ledger_view.dreps.set_window_size(size);
            },
            |s, size| {
                s.ledger_view.pools.set_window_size(size);
            },
            |s, size| {
                s.ledger_view.proposals.set_window_size(size);
            },
            |s, size| {
                s.ledger_view.utxos.set_window_size(size);
            },
            |s, size| {
                for w in s.ledger_view.utxos_by_addr_search.results.values_mut() {
                    w.set_window_size(size);
                }
            },
        ],
    },
];

impl Update for WindowSizeUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action> {
        let (slot, size) = match action {
            Action::SetWindowSize(slot, size) => (*slot, *size),
            _ => return Vec::new(),
        };

        let window = match WINDOW_DEFS.iter().find(|d| d.slot == slot) {
            Some(w) => w,
            None => return Vec::new(),
        };
        for handler in window.handlers.iter() {
            handler(app_state, size);
        }

        Vec::new()
    }
}

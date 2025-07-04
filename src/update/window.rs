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
        slot: WidgetSlot::Options,
        handlers: &[
            |s, size| {
                s.options_window_size = size;
            },
            |s, size| {
                s.ledger_browse_options.set_window_size(size);
            },
            |s, size| {
                s.ledger_search_options.set_window_size(size);
            },
        ],
    },
    WindowSizeDef {
        slot: WidgetSlot::List,
        handlers: &[
            |s, size| {
                s.list_window_size = size;
            },
            |s, size| {
                s.accounts.set_window_size(size);
            },
            |s, size| {
                s.block_issuers.set_window_size(size);
            },
            |s, size| {
                s.dreps.set_window_size(size);
            },
            |s, size| {
                s.pools.set_window_size(size);
            },
            |s, size| {
                s.proposals.set_window_size(size);
            },
            |s, size| {
                s.utxos.set_window_size(size);
            },
            |s, size| {
                for w in s.utxos_by_addr_search.results.values_mut() {
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

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
                s.ledger_mvs.options_window_height = size;
            },
            |s, size| {
                s.get_ledger_browse_options_mut().view.set_height(size);
            },
            |s, size| {
                s.get_ledger_search_options_mut().view.set_height(size);
            },
        ],
    },
    WindowSizeDef {
        slot: WidgetSlot::List,
        handlers: &[
            |s, size| {
                s.ledger_mvs.list_window_height = size;
            },
            |s, size| {
                s.get_accounts_list_mut().view.set_height(size);
            },
            |s, size| {
                s.get_block_issuers_list_mut().view.set_height(size);
            },
            |s, size| {
                s.get_dreps_list_mut().view.set_height(size);
            },
            |s, size| {
                s.get_pools_list_mut().view.set_height(size);
            },
            |s, size| {
                s.get_proposals_list_mut().view.set_height(size);
            },
            |s, size| {
                s.get_utxos_list_mut().view.set_height(size);
            },
            |s, size| {
                for w in s.ledger_mvs.utxos_by_addr_search.results.values_mut() {
                    w.set_height(size);
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

use crate::{
    app_state::AppState,
    states::{Action, ComponentId},
    update::Update,
};

pub struct WindowSizeUpdate;

struct WindowSizeDef {
    component_id: ComponentId,
    handlers: &'static [fn(&mut AppState, usize)],
}

static WINDOW_DEFS: &[WindowSizeDef] = &[
    WindowSizeDef {
        component_id: ComponentId::LedgerBrowseOptions,
        handlers: &[
            |s, size| {
                s.ledger_mvs.options_window_height = size;
            },
            |s, size| {
                s.get_ledger_browse_options_mut().model.set_height(size);
            },
        ],
    },
    WindowSizeDef {
        component_id: ComponentId::LedgerAccountsList,
        handlers: &[|s, size| {
            s.get_accounts_list_mut().model.set_height(size);
        }],
    },
    WindowSizeDef {
        component_id: ComponentId::LedgerBlockIssuersList,
        handlers: &[|s, size| {
            s.get_block_issuers_list_mut().model.set_height(size);
        }],
    },
    WindowSizeDef {
        component_id: ComponentId::LedgerDRepsList,
        handlers: &[|s, size| {
            s.get_dreps_list_mut().model.set_height(size);
        }],
    },
    WindowSizeDef {
        component_id: ComponentId::LedgerPoolsList,
        handlers: &[|s, size| {
            s.get_pools_list_mut().model.set_height(size);
        }],
    },
    WindowSizeDef {
        component_id: ComponentId::LedgerProposalsList,
        handlers: &[|s, size| {
            s.get_proposals_list_mut().model.set_height(size);
        }],
    },
    WindowSizeDef {
        component_id: ComponentId::LedgerUtxosList,
        handlers: &[|s, size| {
            s.get_utxos_list_mut().model.set_height(size);
        }],
    },
    WindowSizeDef {
        component_id: ComponentId::LedgerUtxosByAddrList,
        handlers: &[|s, size| {
            if let Some(model) = s.ledger_mvs.utxos_by_addr_search.get_current_res_mut() {
                model.view.set_height(size);
            }
        }],
    },
];

impl Update for WindowSizeUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action> {
        let (component_id, size) = match action {
            Action::SetWindowSize(cid, size) => (*cid, *size),
            _ => return Vec::new(),
        };

        let window = match WINDOW_DEFS.iter().find(|d| d.component_id == component_id) {
            Some(w) => w,
            None => return Vec::new(),
        };
        for handler in window.handlers.iter() {
            handler(app_state, size);
        }

        Vec::new()
    }
}

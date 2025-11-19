use crate::{
    app_state::AppState,
    components::async_list::AsyncListModel,
    model::async_provider::AsyncProvider,
    states::{Action, ComponentId, InspectOption},
    ui::to_list_item::UtxoItem,
    update::Update,
};
use amaru_kernel::Address;
use crossterm::event::KeyCode;
use std::str::FromStr;
use tokio::sync::mpsc;

pub struct SearchUpdate;

impl Update for SearchUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        // Polling to keep async models alive
        if matches!(action, Action::Tick) {
            if let Some(model) = s.ledger_mvs.utxos_by_addr_search.get_current_res_mut() {
                model.poll_data();
            }
            return Vec::new();
        }

        // Only process keys if the SearchBar is focused
        if !s.layout_model.is_focused(ComponentId::SearchBar) {
            return Vec::new();
        }

        let Action::Key(key) = action else {
            return Vec::new();
        };

        // Dispatch based on active tab
        match s.get_inspect_tabs().selected() {
            InspectOption::Ledger => handle_ledger_search(key, s),
            InspectOption::Chain => { /* TODO: Implement Chain Search Input */ }
            _ => {}
        }

        Vec::new()
    }
}

fn handle_ledger_search(key: &KeyCode, s: &mut AppState) {
    let state = &mut s.ledger_mvs.utxos_by_addr_search;

    match key {
        KeyCode::Char(c) => state.push_char(*c),
        KeyCode::Backspace => state.pop_char(),
        KeyCode::Enter => {
            let query_str = state.builder.clone();

            if let Ok(address) = Address::from_str(&query_str) {
                // Check cache first
                if state.results.contains_key(&address) {
                    state.parsed = Some(address);
                    return;
                }

                let owned_addr = address.clone();
                let db = s.ledger_db.clone();

                // Create Async Provider
                let provider = AsyncProvider::new(move |tx: mpsc::Sender<UtxoItem>| {
                    if let Ok(iter) = amaru_ledger::store::ReadStore::iter_utxos(&*db) {
                        let filtered = iter.filter(move |(_, out)| out.address == owned_addr);
                        for item in filtered {
                            if tx.blocking_send(item).is_err() {
                                break;
                            }
                        }
                    }
                });

                let model = AsyncListModel::new("UTxOs by Address", provider);
                state.cache_result(address, model);
            }
        }
        _ => {}
    }
}

use crate::{app_state::AppState, states::Action, update::Update};

/// The Update fn for polling the async search list.
pub struct PollUtxoSearchUpdate;

impl Update for PollUtxoSearchUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        if *a == Action::PollUtxoSearch
            && let Some(active_search_model) =
                s.ledger_mvs.utxos_by_addr_search.get_current_res_mut()
        {
            active_search_model.poll_data();
        }
        Vec::new()
    }
}

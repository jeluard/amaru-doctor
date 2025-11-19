use crate::{
    app_state::AppState,
    components::list::ListModel,
    states::{Action, ComponentId},
    update::Update,
};
use tracing::{debug, warn}; // Ensure tracing is imported

pub struct DragUpdate;

impl Update for DragUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        match action {
            Action::MouseDragDown => {
                debug!("DragUpdate: Handling MouseDragDown");
                let focused = s
                    .layout_model
                    .is_focused(ComponentId::LedgerUtxosByAddrList);
                debug!("DragUpdate: SearchList Focused? {}", focused);

                if let Some(model) = s.ledger_mvs.utxos_by_addr_search.get_current_res_mut() {
                    debug!("DragUpdate: Executing retreat_window()");
                    model.retreat_window();
                } else {
                    warn!("DragUpdate: No search result model found!");
                }
            }
            Action::MouseDragUp => {
                debug!("DragUpdate: Handling MouseDragUp");
                let focused = s
                    .layout_model
                    .is_focused(ComponentId::LedgerUtxosByAddrList);
                debug!("DragUpdate: SearchList Focused? {}", focused);

                if let Some(model) = s.ledger_mvs.utxos_by_addr_search.get_current_res_mut() {
                    debug!("DragUpdate: Executing advance_window()");
                    model.advance_window();
                } else {
                    warn!("DragUpdate: No search result model found!");
                }
            }
            _ => {}
        }

        Vec::new()
    }
}

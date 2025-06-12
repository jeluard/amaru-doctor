use crate::{
    app_state::AppState,
    states::{Action, WidgetSlot::*},
    update::Update,
};

pub struct WindowSizeUpdate;

impl Update for WindowSizeUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) {
        if let Action::SetWindowSize(slot, size) = action {
            match slot {
                Options => {
                    app_state.browse_options.set_window_size(*size);
                    app_state.search_options.set_window_size(*size);
                }
                List => {
                    app_state.accounts.set_window_size(*size);
                    app_state.block_issuers.set_window_size(*size);
                    app_state.dreps.set_window_size(*size);
                    app_state.pools.set_window_size(*size);
                    app_state.proposals.set_window_size(*size);
                    app_state.utxos.set_window_size(*size);
                }
                _ => {}
            }
        }
    }
}

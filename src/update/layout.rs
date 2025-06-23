use tracing::trace;

use crate::{app_state::AppState, controller::compute_slot_layout, states::Action, update::Update};

pub struct LayoutUpdate;
impl Update for LayoutUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action> {
        let Action::UpdateLayout(rect) = action else {
            return Vec::new();
        };
        trace!("Got layout update, will recompute slot layout");
        app_state.layout = compute_slot_layout(app_state, *rect);
        let mut actions = Vec::new();
        for (slot, rect) in app_state.layout.iter() {
            actions.push(Action::SetWindowSize(*slot, rect.height as usize));
        }
        actions
    }
}

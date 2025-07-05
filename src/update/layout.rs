use crate::{app_state::AppState, controller::compute_slot_layout, states::Action, update::Update};
use tracing::trace;

pub struct LayoutUpdate;
impl Update for LayoutUpdate {
    fn update(&self, action: &Action, app_state: &mut AppState) -> Vec<Action> {
        let Action::UpdateLayout(frame_area) = action else {
            return Vec::new();
        };
        trace!("Got layout update, will recompute slot layout");
        app_state.frame_area = *frame_area;
        app_state.layout = compute_slot_layout(app_state, *frame_area);
        let mut actions = Vec::new();
        for (slot, rect) in app_state.layout.iter() {
            actions.push(Action::SetWindowSize(*slot, rect.height as usize));
        }
        actions
    }
}

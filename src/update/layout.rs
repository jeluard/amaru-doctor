use crate::{app_state::AppState, controller::compute_slot_layout, states::Action, update::Update};
use tracing::trace;

pub struct LayoutUpdate;

impl Update for LayoutUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::UpdateLayout(frame_area) = a else {
            return Vec::new();
        };
        trace!("Got layout update, will recompute slot layout");
        s.frame_area = *frame_area;
        let new_layout = compute_slot_layout(
            s.screen_mode,
            s.get_inspect_tabs().selected(),
            s.get_ledger_mode_tabs().selected(),
            *frame_area,
        );
        let actions = new_layout
            .iter()
            .map(|(slot, rect)| Action::SetWindowSize(*slot, rect.height as usize))
            .collect();
        s.layout_model.update_layout(new_layout);
        actions
    }
}

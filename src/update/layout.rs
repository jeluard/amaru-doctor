use crate::{
    app_state::AppState,
    controller::{LayoutContext, compute_component_layout},
    states::Action,
    update::Update,
};
use tracing::trace;

pub struct LayoutUpdate;

impl Update for LayoutUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::UpdateLayout(frame_area) = a else {
            return Vec::new();
        };
        trace!("Got layout update, will recompute component layout");
        s.frame_area = *frame_area;

        let ctx = LayoutContext {
            screen_mode: s.screen_mode,
            inspect_option: s.get_inspect_tabs().selected(),
            ledger_mode: s.get_ledger_mode_tabs().selected(),
            ledger_browse: s
                .get_ledger_browse_options()
                .model
                .selected_item()
                .cloned()
                .unwrap_or_default(),
            ledger_search: s
                .get_ledger_search_options()
                .model
                .selected_item()
                .cloned()
                .unwrap_or_default(),
        };

        let new_layout = compute_component_layout(ctx, *frame_area);
        s.layout_model.set_layout(new_layout);

        s.layout_model
            .layout
            .iter()
            .map(|(component_id, rect)| Action::SetWindowSize(*component_id, rect.height as usize))
            .collect()
    }
}

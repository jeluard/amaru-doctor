use crate::{
    app_state::AppState,
    components::{Component, root::RootComponent},
    states::Action,
    update::Update,
};
use tracing::trace;

pub struct LayoutUpdate;
impl Update for LayoutUpdate {
    fn update(&self, a: &Action, s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
        let Action::UpdateLayout(frame_area) = a else {
            return Vec::new();
        };
        trace!("Got layout update, calling RootComponent to calculate layout");
        s.frame_area = *frame_area;
        let interactive_layout = root.calculate_layout(*frame_area, s);
        s.layout_model.set_layout(interactive_layout);
        Vec::new()
    }
}

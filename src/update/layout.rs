use std::collections::HashMap;

use crate::{
    app_state::AppState,
    states::{Action, ComponentId},
    update::Update,
};
use tracing::trace;

pub struct LayoutUpdate;

impl Update for LayoutUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        let Action::UpdateLayout(frame_area) = a else {
            return Vec::new();
        };
        trace!("Got layout update, calling RootComponent to calculate layout");
        s.frame_area = *frame_area;

        // We build a map of ONLY the interactive leaf widgets
        let mut interactive_layout = HashMap::new();

        if let Some(root) = s.component_registry.get(&ComponentId::Root) {
            let root_layout = root.calculate_layout(*frame_area, s);
            for (page_id, page_rect) in root_layout {
                interactive_layout.insert(page_id, page_rect);

                if let Some(page) = s.component_registry.get(&page_id) {
                    let page_layout = page.calculate_layout(page_rect, s);
                    interactive_layout.extend(page_layout);
                }
            }
        }

        s.layout_model.set_layout(interactive_layout);

        // Resize windows based on the new layout
        s.layout_model
            .layout
            .iter()
            .map(|(id, rect)| Action::SetWindowSize(*id, rect.height as usize))
            .collect()
    }
}

use crate::{
    app_state::AppState,
    states::{ComponentId, WidgetSlot},
    view::View,
};
use ratatui::{Frame, layout::Rect};
use tracing::error;

pub struct ComponentViewAdapter {
    id: ComponentId,
    slot: WidgetSlot,
}

impl ComponentViewAdapter {
    pub const fn new(id: ComponentId, slot: WidgetSlot) -> Self {
        Self { id, slot }
    }
}

impl View for ComponentViewAdapter {
    fn slot(&self) -> WidgetSlot {
        self.slot
    }

    fn is_visible(&self, _s: &AppState) -> bool {
        true
    }

    fn render(&self, f: &mut Frame, area: Rect, s: &AppState) {
        if let Some(component) = s.component_registry.get(&self.id) {
            let mut layout = std::collections::HashMap::new();
            layout.insert(self.id, area);
            component.render(f, s, &layout);
        } else {
            error!(
                "ComponentViewAdapter could not find component with id: {}",
                self.id
            );
        }
    }
}

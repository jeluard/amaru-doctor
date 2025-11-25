use crate::{
    app_state::AppState,
    components::root::RootComponent,
    states::{Action, ComponentId},
    update::Update,
};
use crossterm::event::KeyCode;
use strum::Display;
use tracing::trace;

#[derive(Display, Debug, Clone, Copy)]
enum ScrollDirection {
    Left,
    Right,
}

pub struct TabsUpdate;
impl Update for TabsUpdate {
    fn update(&self, action: &Action, s: &mut AppState) -> Vec<Action> {
        let Some(direction) = (match action {
            Action::Key(KeyCode::Left) => Some(ScrollDirection::Left),
            Action::Key(KeyCode::Right) => Some(ScrollDirection::Right),
            _ => None,
        }) else {
            return Vec::new();
        };

        let focus = s.layout_model.get_focus();

        match focus {
            ComponentId::InspectTabs => {
                if let Some(root) = s.component_registry.get_mut(&ComponentId::Root)
                    && let Some(root_comp) = root.as_any_mut().downcast_mut::<RootComponent>()
                {
                    match direction {
                        ScrollDirection::Left => root_comp.tabs.cursor.next_back(),
                        ScrollDirection::Right => root_comp.tabs.cursor.non_empty_next(),
                    };
                    return vec![Action::UpdateLayout(s.frame_area)];
                }
                Vec::new()
            }
            _ => {
                trace!("No tab scroll logic for component {:?}", focus);
                Vec::new()
            }
        }
    }
}

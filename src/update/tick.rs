use crate::{app_state::AppState, components::root::RootComponent, states::Action, update::Update};

/// The Update fn for each TUI tick.
pub struct TickUpdate;
impl Update for TickUpdate {
    fn update(&self, a: &Action, _: &mut AppState, _root: &mut RootComponent) -> Vec<Action> {
        if *a != Action::Tick {
            return Vec::new();
        };
        vec![Action::GetButtonEvents]
    }
}

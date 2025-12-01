use crate::{app_state::AppState, components::root::RootComponent, states::Action, update::Update};

/// The Update fn for each TUI tick.
pub struct TickUpdate;
impl Update for TickUpdate {
    // TODO: Other calculating of whether a sync needs to happen (like a window
    // size update or layout change) should likely live here.
    fn update(&self, a: &Action, _: &mut AppState, _root: &mut RootComponent) -> Vec<Action> {
        if *a != Action::Tick {
            return Vec::new();
        };
        // The list of actions that should happen each tick. We assume that each
        // corresponding Update impls its own 'efficiency' guard.
        vec![
            Action::SyncPromMetrics,
            Action::GetButtonEvents,
            Action::PollUtxoSearch,
        ]
    }
}

use crate::{app_state::AppState, components::root::RootComponent, states::Action, update::Update};

/// The Update fn for sync'ing Prometheus metrics.
pub struct PromMetricsUpdate;
impl Update for PromMetricsUpdate {
    fn update(&self, a: &Action, _s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
        if *a == Action::SyncPromMetrics {
            root.prometheus_page.metrics.sync();
        }
        Vec::new()
    }
}

use crate::{app_state::AppState, states::Action, update::Update};

/// The Update fn for sync'ing Prometheus metrics.
pub struct PromMetricsUpdate;
impl Update for PromMetricsUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        if *a == Action::SyncPromMetrics {
            s.get_prom_metrics_mut().sync();
        }

        Vec::new()
    }
}

use crate::{
    app_state::AppState,
    components::prometheus_page::PrometheusPageComponent,
    states::{Action, ComponentId},
    update::Update,
};

/// The Update fn for sync'ing Prometheus metrics.
pub struct PromMetricsUpdate;
impl Update for PromMetricsUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        if *a == Action::SyncPromMetrics
            && let Some(page) = s.component_registry.get_mut(&ComponentId::PrometheusPage)
            && let Some(prom_page) = page.as_any_mut().downcast_mut::<PrometheusPageComponent>()
        {
            prom_page.metrics.sync();
        }

        Vec::new()
    }
}

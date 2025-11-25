use crate::{
    app_state::AppState,
    components::otel_page::OtelPageComponent,
    states::{Action, ComponentId},
    update::Update,
};
use std::cmp::Reverse;
use tracing::{debug, warn};

pub struct TraceGraphUpdate;
impl Update for TraceGraphUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        if *a != Action::SyncTraceGraph {
            return Vec::new();
        }

        let page_opt = s.component_registry.get(&ComponentId::OtelPage);
        if page_opt.is_none() {
            warn!("TraceGraphUpdate: OtelPage not found in registry!");
        }

        let selected_trace = page_opt
            .and_then(|c| c.as_any().downcast_ref::<OtelPageComponent>())
            .and_then(|p| p.trace_list.selected_item())
            .copied();

        let changed = s.otel_view.sync_state(selected_trace.as_ref());
        if !changed {
            return Vec::new();
        }

        debug!("TraceGraphUpdate: Changes detected. Updating TraceList...");

        let data = s.otel_view.trace_graph_source.load();
        let mut trace_ids: Vec<_> = data.traces.keys().copied().collect();
        trace_ids.sort_unstable_by_key(|id| Reverse(data.traces.get(id).unwrap().start_time()));

        debug!("TraceGraphUpdate: Found {} traces.", trace_ids.len());

        if let Some(page) = s.component_registry.get_mut(&ComponentId::OtelPage) {
            if let Some(otel_page) = page.as_any_mut().downcast_mut::<OtelPageComponent>() {
                otel_page.trace_list.sync_state(trace_ids);
                debug!("TraceGraphUpdate: Successfully updated trace_list.");
            } else {
                warn!("TraceGraphUpdate: Failed to downcast OtelPage component!");
            }
        } else {
            warn!("TraceGraphUpdate: OtelPage not found in registry for update!");
        }

        Vec::new()
    }
}

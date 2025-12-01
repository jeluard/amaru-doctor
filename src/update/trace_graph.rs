use crate::{app_state::AppState, components::root::RootComponent, states::Action, update::Update};
use std::cmp::Reverse;

pub struct TraceGraphUpdate;
impl Update for TraceGraphUpdate {
    fn update(&self, a: &Action, _s: &mut AppState, root: &mut RootComponent) -> Vec<Action> {
        if *a != Action::SyncTraceGraph {
            return Vec::new();
        }

        let selected_trace = root.otel_page.trace_list.selected_item().copied();

        let changed = root
            .otel_page
            .view_state
            .sync_state(selected_trace.as_ref());
        if !changed {
            return Vec::new();
        }

        let data = root.otel_page.view_state.trace_graph.load();

        let mut trace_ids: Vec<_> = data.traces.keys().copied().collect();
        trace_ids.sort_unstable_by_key(|id| Reverse(data.traces.get(id).unwrap().start_time()));

        root.otel_page.trace_list.sync_state(trace_ids);

        Vec::new()
    }
}

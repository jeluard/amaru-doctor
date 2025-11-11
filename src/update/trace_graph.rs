use crate::{app_state::AppState, states::Action, update::Update};
use std::cmp::Reverse;

/// The Update fn for sync'ing the TraceGraph.
pub struct TraceGraphUpdate;
impl Update for TraceGraphUpdate {
    fn update(&self, a: &Action, s: &mut AppState) -> Vec<Action> {
        if *a != Action::SyncTraceGraph {
            return Vec::new();
        }

        let selected_trace = s.get_trace_list().selected_item().copied();
        let changed = s.otel_view.sync_state(selected_trace.as_ref());
        if !changed {
            return Vec::new();
        }

        // Get the new data
        let data = s.otel_view.trace_graph_source.load();

        // Compute the new list of items
        let mut trace_ids: Vec<_> = data.traces.keys().copied().collect();
        trace_ids.sort_unstable_by_key(|id| Reverse(data.traces.get(id).unwrap().start_time()));

        s.get_trace_list_mut().sync_state(trace_ids);

        Vec::new()
    }
}
